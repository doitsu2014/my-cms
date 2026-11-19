//! Operator-only backup inventory and fail-closed restore planning.
//!
//! This intentionally does not execute a destructive database restore. A safe
//! full Supabase restore requires a verified platform-wide recovery procedure,
//! including Supabase-managed schemas and service coordination.
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    env,
    path::{Component, Path},
    process::ExitCode,
    time::{SystemTime, UNIX_EPOCH},
};

const FORMAT_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Manifest {
    format_version: u32,
    backup_id: String,
    started_unix_seconds: u64,
    completed_unix_seconds: u64,
    consistency: String,
    database_archive: Option<String>,
    database_sha256: Option<String>,
    objects: Vec<ObjectEntry>,
    complete: bool,
    outcome: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ObjectEntry {
    key: String,
    sha256: String,
    bytes: u64,
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 80
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

fn safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn now_seconds() -> Result<u64, String> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .map_err(|_| "system clock precedes Unix epoch".to_string())
}

fn credentials_present() -> bool {
    env::var_os("AWS_ACCESS_KEY_ID").is_some()
        || env::var_os("AWS_PROFILE").is_some()
        || env::var_os("AWS_WEB_IDENTITY_TOKEN_FILE").is_some()
}

fn usage() {
    eprintln!("Usage: my-cms-api backup <create|list|restore BACKUP_ID>\n\
             create requires AWS credentials, BACKUP_S3_ENDPOINT and BACKUP_LOCAL_ROOT.\n\
             restore is currently unavailable: full Supabase managed-data recovery is not verified.");
}

pub async fn handle_args(args: &[String]) -> ExitCode {
    match args.first().map(String::as_str) {
        Some("--help") | Some("-h") => {
            usage();
            ExitCode::SUCCESS
        }
        Some("restore") => {
            let Some(id) = args.get(1) else {
                eprintln!("restore requires a backup ID");
                return ExitCode::FAILURE;
            };
            if !valid_id(id) {
                eprintln!("invalid backup ID");
                return ExitCode::FAILURE;
            }
            eprintln!("restore refused: safe full Supabase restore, including managed schemas and metadata, is not verified; no data was changed");
            ExitCode::FAILURE
        }
        Some("create") => {
            if !credentials_present() {
                eprintln!("backup failed: AWS credentials are not configured");
                return ExitCode::FAILURE;
            }
            if env::var_os("BACKUP_S3_ENDPOINT").is_none()
                || env::var_os("BACKUP_LOCAL_ROOT").is_none()
            {
                eprintln!(
                    "backup failed: BACKUP_S3_ENDPOINT and BACKUP_LOCAL_ROOT must be configured"
                );
                return ExitCode::FAILURE;
            }
            eprintln!("backup failed: S3 transfer adapter is not configured; no complete backup was created");
            ExitCode::FAILURE
        }
        Some("list") => {
            if !credentials_present() {
                eprintln!("list failed: AWS credentials are not configured");
                return ExitCode::FAILURE;
            }
            eprintln!("list failed: S3 transfer adapter is not configured");
            ExitCode::FAILURE
        }
        _ => {
            usage();
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hashes_archive_payload_deterministically() {
        assert_eq!(
            digest(b"archive"),
            "0eb3e36bfb24dcd9bb1d1bece1531216b59539a8fde17ee80224af0653c92aa3"
        );
    }
    #[test]
    fn rejects_unsafe_backup_ids_and_paths() {
        assert!(!valid_id("../prod"));
        assert!(!valid_id("id/../../x"));
        assert!(!safe_relative_path("../secret"));
        assert!(!safe_relative_path("/etc/passwd"));
        assert!(valid_id("backup_2026-01"));
        assert!(safe_relative_path("db/globals.sql"));
    }
    #[test]
    fn manifest_requires_full_version_and_integrity_fields() {
        let manifest = Manifest {
            format_version: FORMAT_VERSION,
            backup_id: "b1".into(),
            started_unix_seconds: 1,
            completed_unix_seconds: 2,
            consistency: "best-effort; no cross-store point-in-time guarantee".into(),
            database_archive: Some("database.dump".into()),
            database_sha256: Some(digest(b"dump")),
            objects: vec![],
            complete: true,
            outcome: "success".into(),
        };
        let encoded = serde_json::to_vec(&manifest).unwrap();
        let decoded: Manifest = serde_json::from_slice(&encoded).unwrap();
        assert_eq!(decoded.format_version, FORMAT_VERSION);
        assert!(decoded.complete);
        assert_eq!(
            decoded.database_sha256.as_deref(),
            Some(digest(b"dump").as_str())
        );
        assert_ne!(
            decoded.database_sha256.as_deref(),
            Some(digest(b"tampered").as_str())
        );
    }
    #[test]
    fn rejects_incomplete_manifest_for_success() {
        let manifest = Manifest {
            format_version: FORMAT_VERSION,
            backup_id: "b1".into(),
            started_unix_seconds: 1,
            completed_unix_seconds: 2,
            consistency: "best-effort".into(),
            database_archive: None,
            database_sha256: None,
            objects: vec![],
            complete: false,
            outcome: "failed".into(),
        };
        assert!(!manifest.complete);
        assert!(manifest.database_archive.is_none());
    }
    #[test]
    fn disallows_invalid_object_keys() {
        assert!(!safe_relative_path("../../outside"));
        assert!(!safe_relative_path(""));
        assert!(safe_relative_path("bucket/path/image.jpg"));
    }
    #[test]
    fn reports_missing_credentials_without_echoing_secrets() {
        let error = "backup failed: AWS credentials are not configured";
        assert!(!error.contains("sentinel-secret"));
    }
}
