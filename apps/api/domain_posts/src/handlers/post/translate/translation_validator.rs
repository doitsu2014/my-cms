//! Translation output validator.
//!
//! Counts `<p>` tags in source and translated HTML, and rejects translations
//! that drop too many paragraphs. Sits between
//! `translate_large_content_internal` and `save_translation` in
//! `PostTranslateHandler::translate_from_openai` so broken translations are
//! never persisted to `post_translations`.
//!
//! Added per
//! `openspec/changes/fix-translator-incomplete-html-output/specs/domain-posts/spec.md`
//! (Requirement: Translation output SHALL preserve paragraph coverage).

use crate::domain::error::AppError;

const PARAGRAPH_TAG_PATTERN: &str = r"(?i)<p\b[^>]*>";

/// Count `<p>` opening tags in an HTML fragment. Case-insensitive and
/// tolerant of attributes (e.g. `<p class="lead">`).
pub fn count_paragraph_tags(html: &str) -> usize {
    let regex = regex::Regex::new(PARAGRAPH_TAG_PATTERN).expect("paragraph tag regex must compile");
    regex.find_iter(html).count()
}

/// Validate that the translated content retains a sufficient share of the
/// source's paragraph coverage.
///
/// Rejection rule (see design Decision 1 in
/// `openspec/changes/fix-translator-incomplete-html-output/design.md`):
/// when the source has one or more `<p>` tags and the translated content
/// has fewer than `max(1, source_count / 2)` paragraphs, return
/// `AppError::Validation("translation", <reason>)`. Otherwise return
/// `Ok(())`.
pub fn validate_paragraph_coverage(source: &str, translated: &str) -> Result<(), AppError> {
    let source_count = count_paragraph_tags(source);
    let translated_count = count_paragraph_tags(translated);

    if source_count == 0 {
        return Ok(());
    }

    let threshold = std::cmp::max(1, source_count / 2);
    if translated_count >= threshold {
        return Ok(());
    }

    Err(AppError::Validation(
        "translation".to_string(),
        format!(
            "translated content has {translated_count} <p> tag(s); source has {source_count}; \
             minimum required is {threshold} (max(1, source / 2))"
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_handles_lowercase_and_uppercase_and_attributes() {
        let html = r#"<P class="lead">a</P><p>b</p><p data-foo="x">c</p>"#;
        assert_eq!(count_paragraph_tags(html), 3);
    }

    #[test]
    fn count_ignores_other_block_tags() {
        let html = "<h2>head</h2><div>d</div><p>only paragraph</p>";
        assert_eq!(count_paragraph_tags(html), 1);
    }

    #[test]
    fn count_returns_zero_for_empty_input() {
        assert_eq!(count_paragraph_tags(""), 0);
    }

    #[test]
    fn complete_translation_returns_ok() {
        let source = "<p>a</p><p>b</p><p>c</p><p>d</p>";
        let translated = "<p>α</p><p>β</p><p>γ</p><p>δ</p>";
        assert!(validate_paragraph_coverage(source, translated).is_ok());
    }

    #[test]
    fn headings_only_translation_is_rejected() {
        let source = "<h2>Overview</h2><p>a</p><p>b</p><p>c</p><p>d</p>";
        let translated = "<h2>Overview</h2><h3>Subhead</h3>";
        let err = validate_paragraph_coverage(source, translated).unwrap_err();
        match err {
            AppError::Validation(field, msg) => {
                assert_eq!(field, "translation");
                assert!(msg.contains("translated content has 0"), "msg was: {msg}");
                assert!(msg.contains("source has 4"), "msg was: {msg}");
            }
            other => panic!("expected AppError::Validation, got {other:?}"),
        }
    }

    #[test]
    fn partial_translation_below_threshold_is_rejected() {
        let source = "<p>1</p><p>2</p><p>3</p><p>4</p><p>5</p><p>6</p>";
        let translated = "<p>1</p><p>2</p>";
        // threshold = max(1, 6/2) = 3; translated has 2 → rejected
        let err = validate_paragraph_coverage(source, translated).unwrap_err();
        match err {
            AppError::Validation(field, msg) => {
                assert_eq!(field, "translation");
                assert!(msg.contains("translated content has 2"), "msg was: {msg}");
                assert!(msg.contains("source has 6"), "msg was: {msg}");
                assert!(msg.contains("minimum required is 3"), "msg was: {msg}");
            }
            other => panic!("expected AppError::Validation, got {other:?}"),
        }
    }

    #[test]
    fn short_source_with_no_paragraphs_is_not_rejected() {
        // source has 0 <p> tags → validator short-circuits to Ok regardless of translated.
        let source = "<h2>head</h2><div>only a div</div>";
        let translated = "<h2>título</h2>";
        assert!(validate_paragraph_coverage(source, translated).is_ok());
    }

    #[test]
    fn translation_at_exact_threshold_is_accepted() {
        // source_count = 6, threshold = 3; translated_count = 3 → accepted.
        let source = "<p>1</p><p>2</p><p>3</p><p>4</p><p>5</p><p>6</p>";
        let translated = "<p>a</p><p>b</p><p>c</p>";
        assert!(validate_paragraph_coverage(source, translated).is_ok());
    }

    #[test]
    fn single_paragraph_source_still_requires_at_least_one_paragraph() {
        // source_count = 1, threshold = max(1, 1/2) = 1; translated has 0 → reject.
        let source = "<p>only paragraph</p>";
        let translated = "<h2>only a heading</h2>";
        let err = validate_paragraph_coverage(source, translated).unwrap_err();
        assert!(matches!(err, AppError::Validation(ref f, _) if f == "translation"));
    }
}

/// Test for the prompt constant. Lives in the validator module so the
/// tightening change has a co-located regression test.
///
/// Asserts that the OpenAI system prompt contains both the original
/// instruction and the paragraph-preservation clause appended in
/// `fix-translator-incomplete-html-output`.
#[cfg(test)]
mod prompt_tests {
    use super::super::translate_handler::TRANSLATION_INSTRUCTION_HTML;

    #[test]
    fn prompt_keeps_original_html_preservation_clause() {
        assert!(
            TRANSLATION_INSTRUCTION_HTML
                .contains("Preserve all HTML tags and structure exactly as they are"),
            "TRANSLATION_INSTRUCTION_HTML must still contain the original preservation clause"
        );
    }

    #[test]
    fn prompt_contains_paragraph_preservation_clause() {
        assert!(
            TRANSLATION_INSTRUCTION_HTML
                .contains("Return the same number of `<p>` paragraphs as the source"),
            "TRANSLATION_INSTRUCTION_HTML must contain the paragraph-preservation clause"
        );
    }

    #[test]
    fn prompt_forbids_headings_only_response() {
        assert!(
            TRANSLATION_INSTRUCTION_HTML
                .contains("Never return a response that contains only headings"),
            "TRANSLATION_INSTRUCTION_HTML must forbid headings-only responses"
        );
    }
}
