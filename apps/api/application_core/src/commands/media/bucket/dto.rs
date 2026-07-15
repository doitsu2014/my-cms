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

lazy_static! {
    pub static ref BUCKET_NAME_REGEX: Regex =
        Regex::new(BUCKET_NAME_REGEX_STR).expect("bucket name regex is valid");
}

pub fn is_valid_bucket_name(name: &str) -> bool {
    BUCKET_NAME_REGEX.is_match(name)
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
}
