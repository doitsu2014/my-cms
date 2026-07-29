//! Re-export of the `translation_jobs` entity and module — owned by the
//! post domain because the translation jobs table is part of the post
//! aggregate.

pub use application_core::entities::translation_jobs;