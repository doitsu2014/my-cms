//! pgvector `VectorStore` — the post translation pipeline owns this.
//!
//! Moved from `application_core::commands::ai::vector_store_pg` per design
//! Decision 2. The vector store depends on `OPENAI_API_KEY` env var.

pub mod vector_store_pg;

pub use vector_store_pg::VectorStore;
