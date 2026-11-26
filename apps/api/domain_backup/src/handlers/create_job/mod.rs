//! `POST /backup/jobs` logic (IF-02; FN-05..FN-08). The HTTP layer (WU-08) maps
//! [`CreateJobError::reason_code`] to the status and body of IFC section 2.5.

pub mod create_job_handler;
pub mod request;

#[cfg(test)]
mod tests;

pub use create_job_handler::{AcceptContext, Accepted, CreateJobHandler};
pub use request::{parse_create_request, CreateRequest, MAX_BODY_BYTES};

use thiserror::Error;

/// Every refusal IF-02 can produce before or during acceptance. Display strings are
/// fixed and carry no database detail, no key and no request content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum CreateJobError {
    /// 415: `Content-Type` is not `application/json`.
    #[error("unsupported media type")]
    UnsupportedMediaType,
    /// 413: body over 16 KiB.
    #[error("payload too large")]
    PayloadTooLarge,
    /// 400: malformed JSON, an unknown field or a missing `instance_id`.
    #[error("malformed request")]
    MalformedRequest,
    /// 400: no `Idempotency-Key` header.
    #[error("idempotency key required")]
    IdempotencyKeyRequired,
    /// 400: the key has a bad length or character set.
    #[error("idempotency key invalid")]
    IdempotencyKeyInvalid,
    /// 400: `instance_id` is not the configured instance.
    #[error("instance not allowed")]
    InstanceNotAllowed,
    /// 409: same key, different request fingerprint.
    #[error("idempotency key reused")]
    IdempotencyKeyReused,
    /// 409: another job is queued or running for the instance (checked after replay).
    #[error("concurrency limit")]
    ConcurrencyLimit,
    /// 503: the audit write was rejected on a reachable database; nothing persisted.
    #[error("audit unavailable")]
    AuditUnavailable,
    /// 503: connection loss, timeout or a failed commit; nothing was accepted.
    #[error("store unavailable")]
    StoreUnavailable,
}

impl CreateJobError {
    /// Stable `reason_code` of the error body (IFC section 2.5).
    pub fn reason_code(self) -> &'static str {
        match self {
            CreateJobError::UnsupportedMediaType => "UNSUPPORTED_MEDIA_TYPE",
            CreateJobError::PayloadTooLarge => "PAYLOAD_TOO_LARGE",
            CreateJobError::MalformedRequest => "MALFORMED_REQUEST",
            CreateJobError::IdempotencyKeyRequired => "IDEMPOTENCY_KEY_REQUIRED",
            CreateJobError::IdempotencyKeyInvalid => "IDEMPOTENCY_KEY_INVALID",
            CreateJobError::InstanceNotAllowed => "INSTANCE_NOT_ALLOWED",
            CreateJobError::IdempotencyKeyReused => "IDEMPOTENCY_KEY_REUSED",
            CreateJobError::ConcurrencyLimit => "CONCURRENCY_LIMIT",
            CreateJobError::AuditUnavailable => "AUDIT_UNAVAILABLE",
            CreateJobError::StoreUnavailable => "STORE_UNAVAILABLE",
        }
    }

    /// HTTP status of the refusal (IFC section 2.5).
    pub fn status(self) -> u16 {
        match self {
            CreateJobError::UnsupportedMediaType => 415,
            CreateJobError::PayloadTooLarge => 413,
            CreateJobError::MalformedRequest
            | CreateJobError::IdempotencyKeyRequired
            | CreateJobError::IdempotencyKeyInvalid
            | CreateJobError::InstanceNotAllowed => 400,
            CreateJobError::IdempotencyKeyReused | CreateJobError::ConcurrencyLimit => 409,
            CreateJobError::AuditUnavailable | CreateJobError::StoreUnavailable => 503,
        }
    }
}
