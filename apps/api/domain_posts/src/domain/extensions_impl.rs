//! `StringExtension::to_slug` — slug helper used by the post handlers.
//!
//! Moved from `application_core::common::extensions` per design Decision 2.

use slugify::slugify;

pub trait StringExtension {
    fn to_slug(&self) -> String;
}

impl StringExtension for String {
    fn to_slug(&self) -> String {
        slugify!(self)
    }
}

impl StringExtension for str {
    fn to_slug(&self) -> String {
        slugify!(&self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_to_slug() {
        let test_string = "This is a test string";
        let expected_slug = "this-is-a-test-string";
        assert_eq!(test_string.to_slug(), expected_slug);
    }
}
