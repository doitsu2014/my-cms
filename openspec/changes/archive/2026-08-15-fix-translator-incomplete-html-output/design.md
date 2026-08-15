## Context

`PostTranslateHandler::handle_translate_post` (in
`apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs`)
orchestrates the 3-tier lookup (DB → pgvector similarity → OpenAI) and
ultimately writes the translation to `post_translations`. The OpenAI leg
calls `translate_large_content_internal`, which:

1. Splits the source HTML at block-tag boundaries (`chunk_html_content`,
   `MAX_CHUNK_SIZE = 2000` chars).
2. Fires the chunks at the OpenAI chat completion endpoint in parallel via
   a `JoinSet`.
3. Concatenates the responses in order with `join("")`.
4. Hands the combined string back to `translate_from_openai`, which then
   hands it to `save_translation`.

The current contract trusts whatever OpenAI returns. There is no structural
check, no length sanity check, no retry path. When the model returns a
headings-only response (because the chunk happened to contain only headings
or because the response was truncated by `MAX_TOKENS_PER_REQUEST`), the
broken string is persisted verbatim and surfaces in the public reader.

The companion reader-side helper `getLocalizedPost` in
`apps/ducth-dev-website/src/lib/i18n/getLocalizedPost.ts` already falls
back to the original content when the saved translation is missing
paragraphs, so the user impact is contained. The right fix, however, is to
stop the bad data from being saved in the first place so every consumer of
the translation benefits, not just the public reader.

## Goals / Non-Goals

**Goals:**

- Reject any translation response that does not retain the source's
  paragraph coverage, so `post_translations.content` only ever holds
  structurally complete translations.
- Keep the existing translator behaviour intact for valid translations
  (no change to chunking, prompt, lookup order, or response shape).
- Keep the existing atomicity: when a forced re-translation fails, the
  previous translation is preserved.
- Surface the rejection through `AppError::Validation("translation", ...)`
  so the HTTP adapter can map it to a 422 response.
- Emit a `tracing::warn!` line on every rejection with the post id,
  target language, source paragraph count, and translated paragraph count
  so we can monitor AI quality over time.
- Lock the regression in with deterministic unit tests that drive the
  validator directly, so a future chunking or prompt change cannot
  silently re-introduce the bug.

**Non-Goals:**

- No changes to the chunking strategy (`chunk_html_content` /
  `chunk_text`), the OpenAI model selection, the temperature, the
  `MAX_CHUNK_SIZE`, or the `MAX_TOKENS_PER_REQUEST` budget. Those are
  orthogonal tunables; the validator sits behind them.
- No changes to the pgvector similarity-reuse path. A reused translation
  has already been validated (it was persisted previously), so it can
  bypass the new validator.
- No new `AppError` variant. The existing `AppError::Validation(String,
  String)` variant is sufficient.
- No changes to the public reader or the reader-side
  `getLocalizedPost` fallback. It stays as a second line of defence.
- No DB migration, no new dependency.

## Decisions

### Decision 1 — Validate by `<p>` tag count

**What:** After the OpenAI response is reassembled but before
`save_translation` is called, count `<p>` tags (case-insensitive) in the
source and in the translated content. If the source has ≥ 1 paragraph and
the translated content has 0 paragraphs, or the translated count is below
`max(1, source_count / 2)`, reject the translation.

**Why:** A headings-only response is the observed failure mode. Paragraph
count is a cheap, deterministic structural proxy that catches it without
needing a full HTML diff. The `max(1, source/2)` floor tolerates short
articles whose paragraphs legitimately collapse in translation (e.g. a
single paragraph that the model rephrases into two list items) while
still rejecting the "lost every paragraph" failure mode.

**Alternatives considered:**

- *Character-length ratio.* Rejected because short source posts and long
  source posts have very different baselines, and a 2x ratio would either
  miss the headings-only case (when the source has short headings) or
  reject legitimate rewrites.
- *Full HTML diff via html5ever.* Rejected for v1 because it adds
  complexity and a dependency on the existing parser for a behaviour we
  can express with a single regex. We can graduate to a structural diff
  later if the heuristic proves too lax or too strict in practice.
- *Re-ask OpenAI to verify its own output.* Rejected — adds latency, cost,
  and a second prompt we cannot deterministically test.

### Decision 2 — Reject via `AppError::Validation`

**What:** Reuse the existing `AppError::Validation("translation",
"<reason>")` variant.

**Why:** The variant already exists, the Display impl already produces a
human-readable message, and the HTTP adapter layer already maps
`Validation` to a 422 response. Adding a new `TranslationOutputTooShort`
variant would be pure boilerplate.

**Alternatives considered:**

- *New `AppError::TranslationOutputTooShort(String)` variant.* Rejected —
  adds a constructor, a Display branch, an `Error::source` branch, and a
  new HTTP mapping for one extra failure mode that the existing
  `Validation` variant already covers.

### Decision 3 — Validator sits between `translate_large_content_internal`
and `save_translation`

**What:** The validation lives in `translate_from_openai`, immediately
after the tuple `(translated_title, translated_preview_content,
translated_content)` is produced and before the tuple is returned to the
caller. The caller (`handle_translate_post`) then passes the tuple to
`save_translation` only if no error was raised.

**Why:** This is the single chokepoint every OpenAI-backed translation
flows through. The pgvector similarity-reuse path produces a translation
that has already been persisted (and therefore already validated), so it
correctly bypasses the new check.

**Alternatives considered:**

- *Validate inside `save_translation`.* Rejected because `save_translation`
  is also called from the similarity-reuse branch, where the translation
  is known-good.
- *Validate inside `translate_large_content_internal`.* Rejected because
  the per-chunk join happens there and we want to validate the joined
  result, not each chunk individually.

### Decision 4 — Tighten the system prompt

**What:** Append an explicit paragraph-preservation clause to
`TRANSLATION_INSTRUCTION_HTML`:

> "Return the same number of `<p>` paragraphs as the source. Never return
> a response that contains only headings; if the source has paragraphs
> your response must include the same paragraphs in the target language."

**Why:** The validator is the primary defence, but a stronger prompt
reduces the rate of bad responses and therefore the rate of rejections
(and the rate of OpenAI bill we pay for rejected translations).

**Alternatives considered:**

- *Few-shot example in the prompt.* Deferred — the rule is small enough to
  state declaratively and we have no representative failure corpus yet.

### Decision 5 — Refactor seam for unit tests

**What:** Extract a small `TranslationValidator` helper (a pure function
with a counter and a threshold) so the three rejection scenarios can be
unit-tested without spinning up an HTTP server, a testcontainer, or an
OpenAI mock. The handler continues to own the call site and the error
mapping.

**Why:** The handler today has no testable seam for "did we run the
validator" — its translation entry point is `handle_translate_post`,
which is `async` and takes a real OpenAI key. Extracting the validator is
the smallest change that makes the regression testable today and gives
future reviewers one obvious place to look when reasoning about quality.

**Alternatives considered:**

- *Test `handle_translate_post` end-to-end with a wiremock for the OpenAI
  client.* Out of scope for this change — too much scaffolding for a
  focused fix; we'll keep the seam small and add wiremock coverage if the
  validator grows.

## Risks / Trade-offs

- **[Risk] Legitimate translations that legitimately drop paragraphs are
  rejected.** → Mitigation: the threshold is `max(1, source_count / 2)`,
  not "must equal source count". A single-paragraph source still requires
  ≥ 1 paragraph in the output; a 6-paragraph source requires ≥ 3. We log
  every rejection with the counts so we can tune the threshold from
  telemetry.

- **[Risk] The validator adds one regex pass per translation.** →
  Negligible — paragraph counting is O(n) over already-in-memory strings
  and adds well under a millisecond to the request.

- **[Risk] The tightened prompt causes OpenAI to produce longer responses,
  increasing cost per call.** → Acceptable trade-off because we save the
  cost of every rejected translation and every broken-translation
  customer-support round-trip. We can revisit if the cost impact is
  measurable in the new `tracing::warn!` line.

- **[Risk] Force-retranslation with `request.force_retranslate = true`
  still deletes the old translation before the new one is validated.**
  → Pre-existing behaviour (see lines ~389–445 of the current handler)
  and unchanged by this change. The window between delete and save is
  documented in the existing code comments. We do not address it here
  because it is out of scope.

## Migration Plan

1. Land the change behind no feature flag — the validation only rejects
   responses that are demonstrably broken, so no operator toggle is
   needed.
2. Deploy the new API. The first translation requests after deploy will
   surface any previously-broken translations via the validator and emit
   `tracing::warn!` lines; operators can use those lines to decide
   whether to manually re-translate affected posts.
3. Rollback: revert the commit. The validator is additive; reverting
   restores the old behaviour exactly (broken translations stop being
   rejected and start being saved again). No DB migration to undo, no
   background jobs to drain.

## Open Questions

- Should we eventually cap the validator's tolerance at the per-chunk
  level too, so a single bad chunk is rejected before it joins the others?
  Deferred — the current failure mode is global (the whole response is
  headings-only), and per-chunk validation would add complexity for
  unclear benefit.
- Should we store the validator's rejection reason on the
  `translation_jobs` row so the admin UI can show "translation failed
  quality check, retry"? Out of scope for this change but a logical
  follow-up.
