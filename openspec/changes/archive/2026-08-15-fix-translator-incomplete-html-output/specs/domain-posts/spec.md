## ADDED Requirements

### Requirement: Translation output SHALL preserve paragraph coverage

The system SHALL validate the OpenAI response inside
`PostTranslateHandler::translate_from_openai` before the result is
returned to the caller. The validation SHALL count `<p>` tags
(case-insensitive) in the source `posts.content` and in the translated
content, and SHALL reject the translation by returning
`AppError::Validation("translation", <reason>)` when the source
contains one or more `<p>` tags and the translated content contains
fewer than `max(1, source_paragraph_count / 2)` `<p>` tags. A rejected
translation SHALL NOT be passed to `save_translation`, so the
`post_translations` row is not overwritten.

#### Scenario: Complete translation is saved

- **WHEN** the source has 4 `<p>` tags and the OpenAI response has 4
  `<p>` tags
- **THEN** `translate_from_openai` returns the tuple and the caller
  passes it to `save_translation`, which writes the row to
  `post_translations`

#### Scenario: Headings-only translation is rejected

- **WHEN** the source has 4 `<p>` tags and the OpenAI response has 0
  `<p>` tags (only `<h2>` and `<h3>` headings)
- **THEN** `translate_from_openai` returns
  `AppError::Validation("translation", ...)`
- **AND** `save_translation` is NOT called
- **AND** the existing `post_translations` row for the same post id and
  language code is NOT overwritten

#### Scenario: Partial translation below the threshold is rejected

- **WHEN** the source has 6 `<p>` tags and the OpenAI response has 2
  `<p>` tags (below the `max(1, 6 / 2) = 3` threshold)
- **THEN** `translate_from_openai` returns
  `AppError::Validation("translation", ...)`
- **AND** `save_translation` is NOT called
- **AND** the existing `post_translations` row is NOT overwritten

#### Scenario: Short source with no paragraphs is not rejected

- **WHEN** the source has 0 `<p>` tags and the OpenAI response has 0
  `<p>` tags
- **THEN** `translate_from_openai` returns the tuple without raising
  `AppError::Validation("translation", ...)`

#### Scenario: Validation skips the similarity-reuse branch

- **WHEN** the translation is sourced from
  `find_similar_translation` (a previously-persisted translation
  reused from another post via pgvector)
- **THEN** the validator is NOT invoked and the reused translation is
  passed straight to `save_translation`

### Requirement: Translation system prompt SHALL instruct paragraph preservation

The system SHALL append the following clause to
`TRANSLATION_INSTRUCTION_HTML` in
`apps/api/domain_posts/src/handlers/post/translate/translate_handler.rs`:

> "Return the same number of `<p>` paragraphs as the source. Never
> return a response that contains only headings; if the source has
> paragraphs your response must include the same paragraphs in the
> target language."

The clause SHALL be appended verbatim and SHALL NOT modify or remove
any existing instruction text.

#### Scenario: Prompt contains the paragraph-preservation clause

- **WHEN** a reviewer reads the value of
  `TRANSLATION_INSTRUCTION_HTML` at compile time
- **THEN** the string contains the substring
  "Return the same number of `<p>` paragraphs as the source"

#### Scenario: Existing instruction text is preserved

- **WHEN** a reviewer reads the value of
  `TRANSLATION_INSTRUCTION_HTML` at compile time
- **THEN** the string still contains the original clause
  "Preserve all HTML tags and structure exactly as they are"

### Requirement: Translation rejections SHALL be observable

The system SHALL emit a `tracing::warn!` line on every rejected
translation that includes the `post_id`, the `target_language_code`,
the source `<p>` count, and the translated `<p>` count, so operators
can monitor AI translation quality.

#### Scenario: Warning is emitted on rejection

- **WHEN** `translate_from_openai` rejects a translation because the
  paragraph count is below the threshold
- **THEN** the tracing span for the translation request contains a
  `warn` event whose payload includes `post_id`,
  `target_language_code`, `source_paragraph_count`, and
  `translated_paragraph_count`
