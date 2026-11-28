//! Operator-only backup inventory and fail-closed restore planning.
//!
//! This intentionally does not execute a destructive database restore. A safe
//! full Supabase restore requires a verified platform-wide recovery procedure,
//! including Supabase-managed schemas and service coordination.
//!
//! Vocabulary: the `outcome` strings of [`Manifest`] (`success`, `failed`) belong to this
//! manifest format only. They are unrelated to the job status of the `domain_backup` crate
//! (`queued`, `running`, `incomplete`, `failed`) and are never mapped to or from it. This
//! CLI stays fail-closed: `create`, `list` and `restore` never contact a database or an
//! object store in this build (TS-33).
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
    /// Manifest-local vocabulary (`success`, `failed`); not a `domain_backup` job status.
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

    // ---- TS-33: `handle_args` fails closed (WU-11). Environment variables are
    // process-global, so every test below holds ENV_LOCK and restores what it changed.

    use std::ffi::OsString;
    use std::net::TcpListener;

    static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

    const ENV_KEYS: [&str; 7] = [
        "AWS_ACCESS_KEY_ID",
        "AWS_PROFILE",
        "AWS_WEB_IDENTITY_TOKEN_FILE",
        "BACKUP_S3_ENDPOINT",
        "BACKUP_LOCAL_ROOT",
        "DATABASE_URL",
        "SUPABASE_DB_URL",
    ];

    /// Sets exactly the given variables (clearing every other one the CLI could read) and
    /// restores the previous values on drop.
    struct EnvScope {
        saved: Vec<(&'static str, Option<OsString>)>,
    }

    impl EnvScope {
        fn with(set: &[(&'static str, &str)]) -> Self {
            let saved = ENV_KEYS
                .iter()
                .map(|k| (*k, env::var_os(k)))
                .collect::<Vec<_>>();
            for k in ENV_KEYS {
                env::remove_var(k);
            }
            for (k, v) in set {
                env::set_var(k, v);
            }
            Self { saved }
        }
    }

    impl Drop for EnvScope {
        fn drop(&mut self) {
            for (k, v) in self.saved.drain(..) {
                match v {
                    Some(v) => env::set_var(k, v),
                    None => env::remove_var(k),
                }
            }
        }
    }

    fn args(parts: &[&str]) -> Vec<String> {
        parts.iter().map(|s| (*s).to_owned()).collect()
    }

    const FAILURE: ExitCode = ExitCode::FAILURE;
    const SUCCESS: ExitCode = ExitCode::SUCCESS;

    #[tokio::test]
    async fn ts33_restore_always_refuses_and_never_succeeds() {
        let _lock = ENV_LOCK.lock().await;
        let long = "a".repeat(81);
        // Without and with every credential and setting present.
        for set in [
            &[][..],
            &[
                ("AWS_ACCESS_KEY_ID", "AKIAFAKEFAKEFAKE"),
                ("BACKUP_S3_ENDPOINT", "http://127.0.0.1:9"),
                ("BACKUP_LOCAL_ROOT", "backups"),
            ][..],
        ] {
            let _env = EnvScope::with(set);
            for case in [
                args(&["restore"]),
                args(&["restore", "backup_2026-01"]),
                args(&["restore", "../prod"]),
                args(&["restore", "id/../../x"]),
                args(&["restore", ""]),
                args(&["restore", &long]),
            ] {
                assert_eq!(handle_args(&case).await, FAILURE, "{case:?}");
            }
        }
    }

    #[tokio::test]
    async fn ts33_create_fails_closed_in_every_configuration() {
        let _lock = ENV_LOCK.lock().await;
        let credentials: [&[(&'static str, &str)]; 4] = [
            &[],
            &[("AWS_ACCESS_KEY_ID", "AKIAFAKEFAKEFAKE")],
            &[("AWS_PROFILE", "backup")],
            &[("AWS_WEB_IDENTITY_TOKEN_FILE", "token")],
        ];
        let settings: [&[(&'static str, &str)]; 4] = [
            &[],
            &[("BACKUP_S3_ENDPOINT", "http://127.0.0.1:9")],
            &[("BACKUP_LOCAL_ROOT", "backups")],
            &[
                ("BACKUP_S3_ENDPOINT", "http://127.0.0.1:9"),
                ("BACKUP_LOCAL_ROOT", "backups"),
            ],
        ];
        for c in credentials {
            for s in settings {
                let set: Vec<_> = c.iter().chain(s.iter()).copied().collect();
                let _env = EnvScope::with(&set);
                assert_eq!(
                    handle_args(&args(&["create"])).await,
                    FAILURE,
                    "create must never succeed: {set:?}"
                );
            }
        }
    }

    #[tokio::test]
    async fn ts33_list_fails_closed_with_and_without_credentials() {
        let _lock = ENV_LOCK.lock().await;
        for set in [
            &[][..],
            &[("AWS_ACCESS_KEY_ID", "AKIAFAKEFAKEFAKE")][..],
            &[
                ("AWS_PROFILE", "backup"),
                ("BACKUP_S3_ENDPOINT", "http://127.0.0.1:9"),
            ][..],
        ] {
            let _env = EnvScope::with(set);
            assert_eq!(handle_args(&args(&["list"])).await, FAILURE, "{set:?}");
        }
    }

    #[tokio::test]
    async fn ts33_unknown_and_empty_input_fail_closed_and_only_help_succeeds() {
        let _lock = ENV_LOCK.lock().await;
        let _env = EnvScope::with(&[]);
        for case in [
            args(&[]),
            args(&[""]),
            args(&["backup"]),
            args(&["frobnicate"]),
            args(&["CREATE"]),
            args(&["Restore", "b1"]),
            args(&["create-all"]),
            args(&["--version"]),
            args(&["-x"]),
        ] {
            assert_eq!(handle_args(&case).await, FAILURE, "{case:?}");
        }
        // The one documented success path is the help text.
        assert_eq!(handle_args(&args(&["--help"])).await, SUCCESS);
        assert_eq!(handle_args(&args(&["-h"])).await, SUCCESS);
    }

    #[tokio::test]
    async fn ts33_a_fully_configured_cli_contacts_no_database_or_object_store() {
        let _lock = ENV_LOCK.lock().await;
        // A local listener stands in for both the S3 endpoint and the database. Any
        // connection attempt made while `handle_args` runs is queued on it.
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind local listener");
        listener.set_nonblocking(true).expect("nonblocking");
        let addr = listener.local_addr().expect("local addr");
        let endpoint = format!("http://{addr}");
        let db = format!("postgres://user:pass@{addr}/db");
        let _env = EnvScope::with(&[
            ("AWS_ACCESS_KEY_ID", "AKIAFAKEFAKEFAKE"),
            ("BACKUP_S3_ENDPOINT", endpoint.as_str()),
            ("BACKUP_LOCAL_ROOT", "backups"),
            ("DATABASE_URL", db.as_str()),
            ("SUPABASE_DB_URL", db.as_str()),
        ]);
        for case in [
            args(&["create"]),
            args(&["list"]),
            args(&["restore", "backup_2026-01"]),
            args(&["unknown"]),
        ] {
            assert_eq!(handle_args(&case).await, FAILURE, "{case:?}");
        }
        match listener.accept() {
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
            other => panic!("the CLI contacted the endpoint or database: {other:?}"),
        }
    }
}
