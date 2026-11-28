//! `domain_backup` - backup/restore U1 domain: status enum and guards, bounded
//! configuration, clock and safe error projection.

#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod domain;
pub mod handlers;
pub mod infra;
pub mod lifecycle;
pub mod migrations;
pub mod migrations_cli;
pub mod ports;
pub mod service;

#[cfg(test)]
mod test_db;

/// OS temp dir for tests, with links resolved on Unix. macOS places it behind `/var ->
/// /private/var`, and the CFG-04 storage-root check rejects a link on any ancestor, so an
/// unresolved temp path would disable backup with `INVALID_CONFIG`. Windows keeps the raw
/// path: `canonicalize` there adds the verbatim `\\?\` prefix that CFG-04 also rejects.
#[cfg(test)]
pub(crate) fn test_temp_dir() -> std::path::PathBuf {
    let dir = std::env::temp_dir();
    if cfg!(windows) {
        dir
    } else {
        std::fs::canonicalize(&dir).expect("resolve OS temp dir")
    }
}
