//! Bucket handlers — create, update, get, list, delete, empty, and
//! access-policy enforcement. DTOs (`Bucket`, `CreateBucketRequest`,
//! `UpdateBucketRequest`) and the visibility cache live alongside.

pub mod access;
pub mod create;
pub mod delete;
pub mod dto;
pub mod empty;
pub mod get;
pub mod list;
pub mod update;

pub use access::access_handler::{BucketAccessPolicy, BucketAccessPolicyTrait};
