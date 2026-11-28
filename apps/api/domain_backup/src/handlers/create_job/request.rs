//! Strict parsing of the create request (FN-05, FN-06). Pure: no I/O.

use serde::Deserialize;

use super::CreateJobError;
use crate::domain::idempotency::{validate_key, KeyError};

/// Route-local body cap of IF-02 (IFC section 2.2, T-13).
pub const MAX_BODY_BYTES: usize = 16 * 1024;

/// A request that passed every check of F-05 and F-06.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateRequest {
    pub instance_id: String,
    /// Validated raw key. Used only to derive the digest; never stored or logged.
    pub idempotency_key: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Body {
    instance_id: String,
}

/// `application/json`, ignoring case and any parameters such as `charset`.
pub fn is_json_content_type(content_type: Option<&str>) -> bool {
    content_type
        .and_then(|ct| ct.split(';').next())
        .is_some_and(|media| media.trim().eq_ignore_ascii_case("application/json"))
}

/// Checks in the order of FD F-05/F-06: media type, size, strict JSON, key, instance.
pub fn parse_create_request(
    content_type: Option<&str>,
    idempotency_key: Option<&str>,
    body: &[u8],
    configured_instance: &str,
) -> Result<CreateRequest, CreateJobError> {
    if !is_json_content_type(content_type) {
        return Err(CreateJobError::UnsupportedMediaType);
    }
    if body.len() > MAX_BODY_BYTES {
        return Err(CreateJobError::PayloadTooLarge);
    }
    let parsed: Body =
        serde_json::from_slice(body).map_err(|_| CreateJobError::MalformedRequest)?;
    let key = validate_key(idempotency_key).map_err(|e| match e {
        KeyError::Missing => CreateJobError::IdempotencyKeyRequired,
        KeyError::Invalid => CreateJobError::IdempotencyKeyInvalid,
    })?;
    if parsed.instance_id != configured_instance {
        return Err(CreateJobError::InstanceNotAllowed);
    }
    Ok(CreateRequest {
        instance_id: parsed.instance_id,
        idempotency_key: key.to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const INSTANCE: &str = "local-1";
    const KEY: &str = "key-12345678";
    const JSON: Option<&str> = Some("application/json");

    fn ok_body() -> Vec<u8> {
        br#"{"instance_id":"local-1"}"#.to_vec()
    }

    fn parse(
        ct: Option<&str>,
        key: Option<&str>,
        body: &[u8],
    ) -> Result<CreateRequest, CreateJobError> {
        parse_create_request(ct, key, body, INSTANCE)
    }

    #[test]
    fn a_valid_request_is_accepted_with_the_key_and_instance() {
        let req = parse(JSON, Some(KEY), &ok_body()).unwrap();
        assert_eq!(req.instance_id, INSTANCE);
        assert_eq!(req.idempotency_key, KEY);
    }

    #[test]
    fn media_type_ignores_case_and_parameters_but_nothing_else() {
        for ok in [
            "application/json",
            "Application/JSON",
            "application/json; charset=utf-8",
        ] {
            assert!(parse(Some(ok), Some(KEY), &ok_body()).is_ok(), "{ok}");
        }
        for bad in [
            None,
            Some("text/plain"),
            Some("application/jsonx"),
            Some("application/x-www-form-urlencoded"),
            Some(""),
        ] {
            assert_eq!(
                parse(bad, Some(KEY), &ok_body()),
                Err(CreateJobError::UnsupportedMediaType),
                "{bad:?}"
            );
        }
    }

    #[test]
    fn body_cap_is_16_kib_inclusive() {
        // Exactly at the cap (valid JSON padded with trailing spaces) the size check
        // passes, proving the cap is not applied one byte early.
        let at_cap = {
            let mut b = ok_body();
            b.extend(std::iter::repeat_n(b' ', MAX_BODY_BYTES - b.len()));
            b
        };
        assert_eq!(at_cap.len(), MAX_BODY_BYTES);
        assert!(parse(JSON, Some(KEY), &at_cap).is_ok());
        let mut over = at_cap.clone();
        over.push(b' ');
        assert_eq!(
            parse(JSON, Some(KEY), &over),
            Err(CreateJobError::PayloadTooLarge)
        );
    }

    #[test]
    fn size_is_checked_before_the_body_is_parsed() {
        let huge = vec![b'x'; MAX_BODY_BYTES + 1];
        assert_eq!(
            parse(JSON, Some(KEY), &huge),
            Err(CreateJobError::PayloadTooLarge)
        );
    }

    #[test]
    fn strict_json_rejects_malformed_unknown_and_missing_fields() {
        for body in [
            &b""[..],
            b"not json",
            b"[]",
            b"{}",
            br#"{"instance_id":1}"#,
            br#"{"instance_id":"local-1","extra":true}"#,
            br#"{"instance_id":"local-1","instance_id":"local-1"}"#,
            br#"{"instanceId":"local-1"}"#,
        ] {
            assert_eq!(
                parse(JSON, Some(KEY), body),
                Err(CreateJobError::MalformedRequest),
                "{}",
                String::from_utf8_lossy(body)
            );
        }
    }

    #[test]
    fn missing_and_invalid_keys_have_distinct_errors() {
        assert_eq!(
            parse(JSON, None, &ok_body()),
            Err(CreateJobError::IdempotencyKeyRequired)
        );
        assert_eq!(
            parse(JSON, Some("short"), &ok_body()),
            Err(CreateJobError::IdempotencyKeyInvalid)
        );
        assert_eq!(
            parse(JSON, Some("bad key bad key"), &ok_body()),
            Err(CreateJobError::IdempotencyKeyInvalid)
        );
    }

    #[test]
    fn a_non_configured_instance_is_refused() {
        assert_eq!(
            parse(JSON, Some(KEY), br#"{"instance_id":"other"}"#),
            Err(CreateJobError::InstanceNotAllowed)
        );
        assert_eq!(
            parse(JSON, Some(KEY), br#"{"instance_id":""}"#),
            Err(CreateJobError::InstanceNotAllowed)
        );
    }

    #[test]
    fn check_order_is_media_type_then_size_then_body_then_key_then_instance() {
        // Everything wrong at once: the media type is reported.
        let huge = vec![b'x'; MAX_BODY_BYTES + 1];
        assert_eq!(
            parse(Some("text/plain"), None, &huge),
            Err(CreateJobError::UnsupportedMediaType)
        );
        // Size beats a malformed body and a missing key.
        assert_eq!(
            parse(JSON, None, &huge),
            Err(CreateJobError::PayloadTooLarge)
        );
        // A malformed body beats a missing key.
        assert_eq!(
            parse(JSON, None, b"nope"),
            Err(CreateJobError::MalformedRequest)
        );
        // A bad key beats a wrong instance.
        assert_eq!(
            parse(JSON, Some("x"), br#"{"instance_id":"other"}"#),
            Err(CreateJobError::IdempotencyKeyInvalid)
        );
    }

    #[test]
    fn the_raw_key_is_not_in_any_error_text() {
        let err = parse(
            JSON,
            Some("needle-key-0123456789"),
            br#"{"instance_id":"other"}"#,
        )
        .unwrap_err();
        assert!(!format!("{err} {err:?}").contains("needle-key"));
    }
}
