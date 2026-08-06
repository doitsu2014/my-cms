//! Post HTTP adapters. Each `api_*` function preserves the existing route
//! path, method, and authorization role.

pub mod create;
pub mod delete;
pub mod graphql;
pub mod modify;
pub mod read;
pub mod translate;
