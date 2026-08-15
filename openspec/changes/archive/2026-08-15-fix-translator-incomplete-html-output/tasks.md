## 1. Validator seam

- [x] 1.1 Add a `TranslationValidator` pure helper module under
      `apps/api/domain_posts/src/handlers/post/translate/`
      exposing `count_paragraph_tags(html: &str) -> usize` and
      `validate_paragraph_coverage(source: &str, translated: &str) ->
      Result<(), AppError>` that returns
      `AppError::Validation("translation", <reason>)` when the source
      has ≥ 1 `<p>` tag and the translated count is below
      `max(1, source / 2)`.
- [x] 1.2 Add a unit test module colocated with the helper that
      asserts: complete translation returns `Ok`, headings-only source
      with 0 paragraphs returns `Ok`, headings-only translation with
      source paragraphs returns the `Validation` error, partial
      translation below threshold returns the `Validation` error, and
      the error message includes the source and translated counts.

## 2. Wire the validator into the handler

- [x] 2.1 In
      `PostTranslateHandler::translate_from_openai`, after the
      `(translated_title, translated_preview_content,
      translated_content)` tuple is produced by
      `translate_large_content_internal`, call the new validator with
      `&post.content` and `&translated_content`. On `Err`, emit a
      `tracing::warn!` line carrying `post_id`,
      `target_language_code`, `source_paragraph_count`, and
      `translated_paragraph_count`, then return the error unchanged.
- [x] 2.2 Confirm the validator is NOT invoked on the similarity-reuse
      branch (the reused translation comes from a previously-validated
      row in `post_translations`).
- [x] 2.3 Confirm the validator runs on both the `force_retranslate =
      true` path and the default 3-tier-lookup OpenAI path.

## 3. Tighten the system prompt

- [x] 3.1 Append the paragraph-preservation clause to
      `TRANSLATION_INSTRUCTION_HTML` verbatim (see
      `specs/domain-posts/spec.md` for the exact wording). Do not
      modify or remove any existing instruction text.
- [x] 3.2 Add a unit test (or extend the prompt test if one exists)
      that asserts the constant contains both the original clause
      "Preserve all HTML tags and structure exactly as they are" and
      the new clause "Return the same number of `<p>` paragraphs as the
      source".

## 4. Integration verification

- [x] 4.1 Run `cargo check --workspace` from `apps/api/` and resolve
      any compile errors or warnings introduced by the new helper
      module.
- [x] 4.2 Run `cargo test -p domain_posts` (or the equivalent target)
      and confirm the new validator tests, the prompt test, and every
      existing translation test still pass.
- [x] 4.3 Run `cargo fmt -- --check` and `cargo clippy --workspace
      -- -D warnings` from `apps/api/` and resolve any new lints
      introduced by the change.
- [x] 4.4 Spot-check one existing testcontainer or wiremock test that
      exercises `handle_translate_post` end-to-end and confirm the new
      validator does not break the happy path (a fully-formed OpenAI
      response still lands in `post_translations`).

## 5. Documentation & change hygiene

- [x] 5.1 Update the inline doc comment on
      `translate_from_openai` to mention the validator and link to the
      requirement in `openspec/changes/fix-translator-incomplete-html-output/specs/domain-posts/spec.md`.
- [x] 5.2 Run `openspec verify --change
      fix-translator-incomplete-html-output` and resolve every
      `CRITICAL` finding before requesting review.
