//! Re-export shim for the post-domain migrations (Task 5).
//!
//! The canonical `Migrator`, constants, and individual migration modules now
//! live in `domain_posts::migrations`. This file preserves the legacy
//! `migration::` import path so existing call sites keep compiling during
//! the transition.

pub use domain_posts::migrations::*;

pub use domain_posts::migrations::Migrator;
