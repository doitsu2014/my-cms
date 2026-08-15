## Why

The AI-powered post translator (`PostTranslateHandler` in
`apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs`) is
occasionally returning translations that contain only the headings of an
article while dropping every `<p>` paragraph. The latest live example is the
post "Agentic AI - Developer (SDLC)": the saved English translation stores
`<h2>Overview</h2><h3>What is SDLC in workflow of Agentic AI - Coding?</h3>`
and nothing else, so the public reader renders a heading-only article body
with no prose.

The current pipeline trusts the OpenAI response verbatim — it chunks the
source HTML, sends each chunk in parallel, concatenates the responses, and
writes the result into `post_translations.content`. There is no structural
check that the translated output retains the same block structure as the
source, no retry when the output is clearly truncated, and no warning when
the translated text is dramatically shorter than the source. As a result,
broken translations silently land in the database and propagate to the
reader.

## What Changes

- **Validate translated HTML structure before persisting.** After the OpenAI
  response is reassembled, compare the count of `<p>` tags (and a few other
  structural markers) between source and translation. If the translation
  drops more than an acceptable threshold of paragraphs relative to the
  source, reject the result by returning an `AppError::TranslationOutputToo
  Short` (or a similarly-named variant) instead of saving it.
- **Refine the OpenAI system prompt** so the model is told explicitly that
  the translated output must contain at least one paragraph per source
  paragraph and that it must never return a headings-only response.
- **Extend the unit-test suite** for `PostTranslateHandler` so the
  "headings-only translation" regression is locked in. Cover the three
  cases: a complete translation is saved, a headings-only translation is
  rejected, and a translation with empty paragraphs is rejected.
- **No changes to the existing reader-side fallback** at
  `apps/ducth-dev-website/src/lib/i18n/getLocalizedPost.ts` — it already
  degrades gracefully when the saved translation is incomplete, and this
  change stops the bad data from being saved in the first place.

No breaking changes to the `TranslatePostRequest` /
`TranslatePostResponse` shape, the `/posts/{post_id}/translate` route, or
the 3-tier lookup order. A translation that fails validation returns a
typed error that the HTTP adapter can map to a 422 response, which is the
right contract for "the AI did not produce a usable translation".

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `domain-posts`: tighten the translation pipeline contract so the system
  SHALL validate the OpenAI response for structural completeness (paragraph
  coverage) before persisting and SHALL reject translations that drop
  paragraphs. Add a Requirement that the OpenAI system prompt SHALL instruct
  the model to keep every paragraph from the source. Add Scenarios that
  cover: (1) a complete translation is saved, (2) a headings-only response
  is rejected without overwriting the existing translation, (3) a
  paragraph-dropping response is rejected without overwriting the existing
  translation.

## Impact

- `apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs`
  — new validation function plus the prompt refinement; new
  `AppError::TranslationOutputTooShort` variant (added to
  `apps/api/domain_posts/src/domain/error.rs` if needed, otherwise mapped
  to the closest existing variant).
- `apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs`
  test module — new unit tests covering the validation cases (mock the
  OpenAI client; existing seam in `translate_from_openai` is internal so
  the test can drive `translate_large_content_internal` directly, or we
  refactor to expose a thin trait seam).
- No DB migration. No new dependency. No changes to other domains
  (`domain_auth`, `domain_media`, `domain_user`, `gateway`).
- No changes to the admin site or the public reader. The existing
  reader-side fallback in `getLocalizedPost` is left untouched and remains
  the second line of defence.
- Observability: a `tracing::warn!` is emitted on every rejected
  translation with the post id, language, source paragraph count, and
  translated paragraph count so the AI quality can be monitored.
