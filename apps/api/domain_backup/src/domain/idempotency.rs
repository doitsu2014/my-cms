//! Idempotency key validation, scoped digest and request fingerprint (FN-06;
//! IFC section 4). The raw key is never stored, logged or echoed: only the digest
//! and the fingerprint leave this module.

use serde::Serialize;
use sha2::{Digest, Sha256};

/// The one operation U1 accepts (`backup_jobs.operation`).
pub const OPERATION: &str = "backup_db_slice";

/// Inclusive bounds of a valid `Idempotency-Key` (IFC section 4, RK-12).
pub const KEY_MIN_LEN: usize = 8;
pub const KEY_MAX_LEN: usize = 128;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// The header is absent: 400 `IDEMPOTENCY_KEY_REQUIRED`.
    Missing,
    /// Wrong length or character set: 400 `IDEMPOTENCY_KEY_INVALID`.
    Invalid,
}

/// Validate the header value: 8..=128 characters of `[A-Za-z0-9_-]`.
pub fn validate_key(key: Option<&str>) -> Result<&str, KeyError> {
    let key = key.ok_or(KeyError::Missing)?;
    let charset = key
        .bytes()
        .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-');
    if (KEY_MIN_LEN..=KEY_MAX_LEN).contains(&key.len()) && charset {
        Ok(key)
    } else {
        Err(KeyError::Invalid)
    }
}

/// Field separator of the digest input (ASCII unit separator): it cannot occur in a
/// valid key or in an operation name, so field boundaries cannot collide.
const SEPARATOR: u8 = 0x1f;

fn sha256_hex(input: &[u8]) -> String {
    format!("{:x}", Sha256::digest(input))
}

/// `hex(sha256(actor_sub || 0x1f || operation || 0x1f || key))`, lowercase.
pub fn key_digest(actor_sub: &str, operation: &str, key: &str) -> String {
    let mut input = Vec::with_capacity(actor_sub.len() + operation.len() + key.len() + 2);
    input.extend_from_slice(actor_sub.as_bytes());
    input.push(SEPARATOR);
    input.extend_from_slice(operation.as_bytes());
    input.push(SEPARATOR);
    input.extend_from_slice(key.as_bytes());
    sha256_hex(&input)
}

/// Canonical request body: struct fields serialize in declaration order, which is
/// lexicographic here, with no whitespace. A struct (not a JSON map) so the order does
/// not depend on serde_json feature unification across the workspace.
#[derive(Serialize)]
struct CanonicalRequest<'a> {
    instance_id: &'a str,
    operation: &'a str,
}

/// `hex(sha256(canonical JSON of {operation, instance_id}))`, lowercase. Canonical
/// means keys in lexicographic order with no whitespace.
pub fn request_fingerprint(operation: &str, instance_id: &str) -> String {
    let canonical = serde_json::to_string(&CanonicalRequest {
        instance_id,
        operation,
    })
    .expect("two string fields always serialize");
    sha256_hex(canonical.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Reference vectors computed independently with `sha256sum`, not with this module.
    const DIGEST_VECTOR: &str = "659aa45349ff4ac46019234dad8bc365a692ffcb7be0fa9c2f369c226ec2301b";
    const FINGERPRINT_VECTOR: &str =
        "b384ffa6bef9835603e1e7c7eefd78748f112d50d502be0329d66cc3354312e6";

    #[test]
    fn missing_key_is_missing_not_invalid() {
        assert_eq!(validate_key(None), Err(KeyError::Missing));
    }

    #[test]
    fn key_length_bounds_are_inclusive() {
        assert_eq!(validate_key(Some("1234567")), Err(KeyError::Invalid));
        assert_eq!(validate_key(Some("")), Err(KeyError::Invalid));
        assert_eq!(validate_key(Some("12345678")), Ok("12345678"));
        let max = "a".repeat(KEY_MAX_LEN);
        assert_eq!(validate_key(Some(&max)), Ok(max.as_str()));
        let over = "a".repeat(KEY_MAX_LEN + 1);
        assert_eq!(validate_key(Some(&over)), Err(KeyError::Invalid));
    }

    #[test]
    fn key_character_set_is_ascii_alphanumeric_underscore_hyphen() {
        assert!(validate_key(Some("Az09_-Az09")).is_ok());
        for bad in [
            "abcd efgh",
            "abcd.efgh",
            "abcd/efgh",
            "abcd:efgh",
            "abcdéfgh",
            "abcd\nefgh",
            "abcd\u{0}efgh",
            "key-12345678=",
        ] {
            assert_eq!(validate_key(Some(bad)), Err(KeyError::Invalid), "{bad:?}");
        }
    }

    #[test]
    fn digest_matches_the_reference_vector() {
        assert_eq!(
            key_digest("alice", OPERATION, "key-12345678"),
            DIGEST_VECTOR
        );
    }

    #[test]
    fn digest_is_scoped_by_actor_operation_and_key() {
        let base = key_digest("alice", OPERATION, "key-12345678");
        assert_eq!(base.len(), 64);
        assert!(base
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
        assert_ne!(base, key_digest("bob", OPERATION, "key-12345678"));
        assert_ne!(base, key_digest("alice", "other_operation", "key-12345678"));
        assert_ne!(base, key_digest("alice", OPERATION, "key-12345679"));
        assert_eq!(base, key_digest("alice", OPERATION, "key-12345678"));
    }

    #[test]
    fn separator_prevents_field_boundary_collisions() {
        // Without the 0x1f separator these two would hash the same bytes.
        assert_ne!(
            key_digest("ab", "c", "key-12345678"),
            key_digest("a", "bc", "key-12345678")
        );
    }

    #[test]
    fn digest_never_contains_the_raw_key() {
        let key = "needle-key-0123456789";
        assert!(!key_digest("alice", OPERATION, key).contains(key));
    }

    #[test]
    fn fingerprint_matches_the_reference_vector() {
        assert_eq!(
            request_fingerprint(OPERATION, "local-1"),
            FINGERPRINT_VECTOR
        );
    }

    #[test]
    fn fingerprint_depends_on_operation_and_instance() {
        let base = request_fingerprint(OPERATION, "local-1");
        assert_ne!(base, request_fingerprint(OPERATION, "local-2"));
        assert_ne!(base, request_fingerprint("other_operation", "local-1"));
    }
}
