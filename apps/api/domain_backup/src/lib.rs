//! `domain_backup` - backup/restore U1 domain: status enum and guards, bounded
//! configuration, clock and safe error projection.

#![deny(unsafe_code)]
#![warn(missing_debug_implementations)]

pub mod domain;
pub mod migrations;
pub mod migrations_cli;

#[cfg(test)]
mod test_db;
