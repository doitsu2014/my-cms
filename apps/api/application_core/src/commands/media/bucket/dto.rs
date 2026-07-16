use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bucket {
    pub id: String,
    pub name: String,
    pub public: bool,
    pub file_size_limit: Option<u64>,
    pub allowed_mime_types: Option<Vec<String>>,
    pub owner: Option<String>,
    #[serde(rename = "type")]
    pub bucket_type: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBucketRequest {
    pub name: String,
    pub public: Option<bool>,
    pub file_size_limit: Option<u64>,
    pub allowed_mime_types: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBucketRequest {
    pub public: Option<bool>,
    pub file_size_limit: Option<Option<u64>>,
    pub allowed_mime_types: Option<Option<Vec<String>>>,
}

pub const BUCKET_NAME_REGEX_STR: &str = r"^[a-z][a-z0-9_-]{2,62}$";
pub const BUCKET_NAME_MIN_LEN: usize = 3;
pub const BUCKET_NAME_MAX_LEN: usize = 63;

lazy_static! {
    pub static ref BUCKET_NAME_REGEX: Regex =
        Regex::new(BUCKET_NAME_REGEX_STR).expect("bucket name regex is valid");
}

pub fn is_valid_bucket_name(name: &str) -> bool {
    BUCKET_NAME_REGEX.is_match(name)
}

pub fn bucket_name_error(name: &str) -> Option<String> {
    if let Some(&first) = name.as_bytes().first() {
        if !first.is_ascii_lowercase() {
            return Some("must start with a lowercase letter".to_string());
        }
    }
    if name.len() < BUCKET_NAME_MIN_LEN {
        return Some(format!("minimum {} characters", BUCKET_NAME_MIN_LEN));
    }
    if name.len() > BUCKET_NAME_MAX_LEN {
        return Some(format!("maximum {} characters", BUCKET_NAME_MAX_LEN));
    }
    if !name
        .bytes()
        .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-' || b == b'_')
    {
        return Some(
            "only lowercase letters, digits, hyphens, and underscores allowed".to_string(),
        );
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_accepts_lowercase_letter_then_alphanumerics_dashes_underscores() {
        assert!(is_valid_bucket_name("media"));
        assert!(is_valid_bucket_name("private-docs"));
        assert!(is_valid_bucket_name("avatars_v2"));
        assert!(is_valid_bucket_name("a1b"));
    }

    #[test]
    fn is_valid_rejects_uppercase_letters() {
        assert!(!is_valid_bucket_name("MyBucket"));
        assert!(!is_valid_bucket_name("mediaX"));
    }

    #[test]
    fn is_valid_rejects_names_starting_with_digit() {
        assert!(!is_valid_bucket_name("3d-models"));
        assert!(!is_valid_bucket_name("1media"));
    }

    #[test]
    fn is_valid_rejects_names_starting_with_dash_or_underscore() {
        assert!(!is_valid_bucket_name("-media"));
        assert!(!is_valid_bucket_name("_media"));
    }

    #[test]
    fn is_valid_rejects_names_shorter_than_3_chars() {
        assert!(!is_valid_bucket_name(""));
        assert!(!is_valid_bucket_name("a"));
        assert!(!is_valid_bucket_name("ab"));
    }

    #[test]
    fn is_valid_rejects_names_longer_than_63_chars() {
        let long = "a".repeat(64);
        assert!(!is_valid_bucket_name(&long));
        let max = "a".repeat(63);
        assert!(is_valid_bucket_name(&max));
    }

    #[test]
    fn is_valid_rejects_disallowed_characters() {
        assert!(!is_valid_bucket_name("my bucket"));
        assert!(!is_valid_bucket_name("my.bucket"));
        assert!(!is_valid_bucket_name("my/bucket"));
    }

    #[test]
    fn bucket_name_error_flags_short_names() {
        let err = bucket_name_error("xx").expect("expected error for 2-char name");
        assert_eq!(err, "minimum 3 characters");
    }

    #[test]
    fn bucket_name_error_accepts_three_char_lowercase_name() {
        assert!(bucket_name_error("xxx").is_none());
    }

    #[test]
    fn bucket_name_error_flags_uppercase_lead() {
        let err = bucket_name_error("Xx").expect("expected error for uppercase start");
        assert_eq!(err, "must start with a lowercase letter");
    }

    #[test]
    fn bucket_name_error_flags_invalid_charset() {
        let err = bucket_name_error("xxx!").expect("expected error for invalid charset");
        assert!(err.contains("only lowercase letters"));
    }

    #[test]
    fn bucket_name_error_flags_overlong_names() {
        let long = "x".repeat(64);
        let err = bucket_name_error(&long).expect("expected error for 64-char name");
        assert_eq!(err, "maximum 63 characters");
    }

    #[test]
    fn bucket_name_error_flags_empty_string() {
        let err = bucket_name_error("").expect("expected error for empty name");
        assert_eq!(err, "minimum 3 characters");
    }
}
