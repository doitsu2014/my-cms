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
