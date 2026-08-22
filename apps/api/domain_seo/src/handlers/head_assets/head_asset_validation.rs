//! Strict, parser-backed validation for trusted head markup.

use html5ever::tendril::StrTendril;
use html5ever::tokenizer::{
    BufferQueue, Tag, TagKind, Token, TokenSink, TokenSinkResult, Tokenizer, TokenizerOpts,
};

pub const MAX_SOURCE_BYTES: usize = 32 * 1024;
pub const MAX_LABEL_CHARS: usize = 128;

#[derive(Default)]
struct Sink {
    tokens: Vec<Token>,
}
impl TokenSink for Sink {
    type Handle = ();
    fn process_token(&mut self, token: Token, _line_number: u64) -> TokenSinkResult<()> {
        self.tokens.push(token);
        TokenSinkResult::Continue
    }
}

fn local_name(name: &html5ever::LocalName) -> String {
    name.as_ref().to_ascii_lowercase()
}
fn attr_name(attr: &html5ever::Attribute) -> String {
    local_name(&attr.name.local)
}
fn is_https(value: &str) -> bool {
    value
        .trim()
        .parse::<url::Url>()
        .map(|url| url.scheme() == "https")
        .unwrap_or(false)
}

/// Validate a complete fragment without executing it, fetching URLs, or
/// changing its bytes. Tokenization is used to enforce a strict element stack
/// and to preserve duplicate-attribute detection that DOM parsers normalize.
pub fn validate_source(source: &str) -> Result<(), String> {
    if source.is_empty() {
        return Err("html must not be empty".into());
    }
    if source.len() > MAX_SOURCE_BYTES {
        return Err("html exceeds 32 KiB".into());
    }
    let mut input = BufferQueue::default();
    input.push_back(StrTendril::from(source));
    let sink = Sink::default();
    let mut tokenizer = Tokenizer::new(sink, TokenizerOpts::default());
    let _ = tokenizer.feed(&mut input);
    tokenizer.end();
    let tokens = tokenizer.sink.tokens;
    let mut stack: Vec<String> = Vec::new();
    let mut external_script_stack: Vec<bool> = Vec::new();
    for token in tokens {
        match token {
            Token::TagToken(Tag {
                kind: TagKind::StartTag,
                name,
                self_closing,
                attrs,
            }) => {
                let tag = local_name(&name);
                if !matches!(tag.as_str(), "script" | "meta" | "link") {
                    return Err(format!("element {tag} is not allowed"));
                }
                let mut names = std::collections::HashSet::new();
                for attr in &attrs {
                    let name = attr_name(attr);
                    if !names.insert(name.clone()) {
                        return Err("duplicate attributes are not allowed".into());
                    }
                    if name.starts_with("on") || name == "style" || name == "id" || name == "class"
                    {
                        return Err(format!("attribute {name} is not allowed"));
                    }
                    let allowed = match tag.as_str() {
                        "script" => {
                            matches!(
                                name.as_str(),
                                "async"
                                    | "defer"
                                    | "src"
                                    | "type"
                                    | "integrity"
                                    | "crossorigin"
                                    | "referrerpolicy"
                            ) || name.starts_with("data-")
                        }
                        "meta" => matches!(name.as_str(), "name" | "property" | "content"),
                        "link" => matches!(
                            name.as_str(),
                            "href"
                                | "rel"
                                | "as"
                                | "type"
                                | "media"
                                | "integrity"
                                | "crossorigin"
                                | "referrerpolicy"
                                | "sizes"
                        ),
                        _ => false,
                    };
                    if !allowed {
                        return Err(format!("attribute {name} is not allowed on {tag}"));
                    }
                    if matches!(name.as_str(), "src" | "href") && !is_https(attr.value.as_ref()) {
                        return Err("external URLs must use HTTPS".into());
                    }
                }
                match tag.as_str() {
                    "script" => {
                        let src = attrs.iter().find(|a| attr_name(a) == "src");
                        if src.is_some() && self_closing {
                            return Err("external script must not be self-closing".into());
                        }
                        external_script_stack.push(src.is_some());
                        stack.push(tag);
                    }
                    "meta" | "link" => {
                        if !self_closing && !matches!(tag.as_str(), "meta" | "link") {
                            return Err("invalid void element".into());
                        }
                        validate_special_attrs(&tag, &attrs)?;
                    }
                    _ => unreachable!(),
                }
            }
            Token::TagToken(Tag {
                kind: TagKind::EndTag,
                name,
                ..
            }) => {
                let tag = local_name(&name);
                if stack.pop().as_deref() != Some(tag.as_str()) {
                    return Err("malformed or mismatched markup".into());
                }
                if tag == "script" {
                    external_script_stack.pop();
                }
            }
            Token::CharacterTokens(text) => {
                if external_script_stack.last().copied().unwrap_or(false) && !text.trim().is_empty()
                {
                    return Err("external scripts must not contain inline content".into());
                }
                if !text_is_allowed(&stack, text.as_ref()) {
                    return Err("text is only allowed inside script elements".into());
                }
            }
            Token::NullCharacterToken => return Err("null characters are not allowed".into()),
            Token::CommentToken(_) | Token::DoctypeToken(_) => {
                return Err("comments and doctypes are not allowed".into())
            }
            Token::ParseError(_) => return Err("malformed markup".into()),
            _ => {}
        }
    }
    if !stack.is_empty() {
        return Err("unclosed script element".into());
    }
    Ok(())
}

fn text_is_allowed(stack: &[String], text: &str) -> bool {
    stack
        .last()
        .map(|tag| tag == "script")
        .unwrap_or_else(|| text.trim().is_empty())
}

fn validate_special_attrs(tag: &str, attrs: &[html5ever::Attribute]) -> Result<(), String> {
    let value = |name: &str| {
        attrs
            .iter()
            .find(|a| attr_name(a) == name)
            .map(|a| a.value.to_string())
    };
    if tag == "meta" {
        let name = value("name");
        let property = value("property");
        if name.is_some() == property.is_some() || value("content").is_none() {
            return Err("meta requires exactly one name/property and content".into());
        }
        let key = name.or(property).unwrap().to_ascii_lowercase();
        if key == "description"
            || key == "robots"
            || key.starts_with("og:")
            || key.starts_with("twitter:")
        {
            return Err("typed metadata is owned by the metadata capability".into());
        }
    } else {
        let rel = value("rel").ok_or_else(|| "link requires rel".to_string())?;
        if value("href").is_none() {
            return Err("link requires HTTPS href".into());
        }
        let allowed = [
            "preconnect",
            "dns-prefetch",
            "preload",
            "modulepreload",
            "stylesheet",
            "icon",
        ];
        if rel
            .split_ascii_whitespace()
            .any(|token| !allowed.contains(&token.to_ascii_lowercase().as_str()))
        {
            return Err("link rel is not allowed".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const GTAG: &str = r#"<script async src="https://www.googletagmanager.com/gtag/js?id=G-TEST"></script><script>window.dataLayer=window.dataLayer||[];</script>"#;
    #[test]
    fn accepts_gtag() {
        assert!(validate_source(GTAG).is_ok());
    }
    #[test]
    fn accepts_json_ld_and_verification() {
        assert!(validate_source(r#"<meta name="google-site-verification" content="x"><script type="application/ld+json">{"@context":"https://schema.org"}</script>"#).is_ok());
    }
    #[test]
    fn rejects_body_event_handler_and_urls() {
        for source in [
            "<body>x</body>",
            r#"<script onload="x"></script>"#,
            r#"<script src="javascript:alert(1)"></script>"#,
            r#"<link rel="stylesheet" href="http://example.com/a.css">"#,
        ] {
            assert!(validate_source(source).is_err(), "{source}");
        }
    }
    #[test]
    fn rejects_typed_metadata_and_script_content_with_src() {
        assert!(validate_source(r#"<meta name="description" content="x">"#).is_err());
        assert!(validate_source(r#"<script src="https://example.com/x">inline</script>"#).is_err());
    }
    #[test]
    fn rejects_duplicate_attributes_and_oversized_source() {
        assert!(validate_source(r#"<script async async></script>"#).is_err());
        assert!(validate_source(&"x".repeat(MAX_SOURCE_BYTES + 1)).is_err());
    }
}
