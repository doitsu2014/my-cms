//! Re-export of the pgvector `VectorStore` owned by the post translation
//! pipeline. Lives in `application_core::commands::ai::vector_store_pg` during
//! the transition; will move into `domain_posts::handlers::vector_store`
//! when the translation handler physically relocates.

pub use application_core::commands::ai::vector_store_pg::VectorStore;