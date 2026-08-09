//! Application-layer command handlers for the post domain.
//!
//! Each submodule owns the application logic for one slice of the post
//! domain: `post` (CRUD + translation), `category` (CRUD on categories and
//! their tags/translations), `tag_helper` (local tag operations owned by the
//! post domain), `ai` (OpenAI model registry + client factory), and
//! `vector_store` (pgvector similarity lookup).

pub mod ai;
pub mod category;
pub mod post;
pub mod tag_helper;
pub mod translation_jobs;
pub mod vector_store;

#[cfg(test)]
pub(crate) mod test;
