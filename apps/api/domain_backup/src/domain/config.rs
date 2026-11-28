//! Backup configuration, bounds validation and loopback guard. F-01, F-39.

use std::net::IpAddr;
use std::path::{Path, PathBuf};

const ESCALATION_OWNER: &str = "project_owner";

/// Closed set of reasons backup is disabled. Carries no value (secret-free).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DisabledReason {
    NotConfigured,
    NonLoopbackBind,
    InvalidConfig,
}

impl DisabledReason {
    /// Wire spelling: NOT_CONFIGURED | NON_LOOPBACK_BIND | INVALID_CONFIG.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NotConfigured => "NOT_CONFIGURED",
            Self::NonLoopbackBind => "NON_LOOPBACK_BIND",
            Self::InvalidConfig => "INVALID_CONFIG",
        }
    }
}

/// Owner-approved bounds CFG-05..CFG-15 (Q-05). Only the default is valid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupBounds {
    staging_expiry_secs: u64,
    orphan_grace_secs: u64,
    sweep_interval_secs: u64,
    cleanup_max_attempts: u32,
    cleanup_backoff_secs: u64,
    job_deadline_secs: u64,
    max_concurrent_jobs: u32,
    alloc_max_bytes: u64,
    total_max_bytes: u64,
    retention_days: u32,
    escalation_owner: String,
}

impl Default for BackupBounds {
    fn default() -> Self {
        Self {
            staging_expiry_secs: 24 * 60 * 60,
            orphan_grace_secs: 60 * 60,
            sweep_interval_secs: 15 * 60,
            cleanup_max_attempts: 3,
            cleanup_backoff_secs: 60,
            job_deadline_secs: 10 * 60,
            max_concurrent_jobs: 1,
            alloc_max_bytes: 256 * 1024 * 1024,
            total_max_bytes: 1024 * 1024 * 1024,
            retention_days: 30,
            escalation_owner: ESCALATION_OWNER.to_string(),
        }
    }
}

/// Validated, enabled configuration. Fields are private: `load` is the only constructor.
///
/// `storage_root` is checked by `load` for shape, links and privacy but not against the
/// forbidden (static assets / web build) paths; consumers must call `validate_storage_root`
/// with those paths before first use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BackupConfig {
    instance_id: String,
    instance_label: String,
    storage_root: PathBuf,
    bounds: BackupBounds,
    allowed_origins: Vec<String>,
    request_timeout_secs: u64,
    max_header_bytes: u64,
}

/// Outcome of loading configuration. Never an error: the CMS must still start.
// The tests pattern-match `Enabled(cfg)` by value, so boxing the variant would break them.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackupConfigState {
    Disabled { reason: DisabledReason },
    Enabled(BackupConfig),
}

impl BackupBounds {
    pub fn staging_expiry_secs(&self) -> u64 {
        self.staging_expiry_secs
    }
    pub fn orphan_grace_secs(&self) -> u64 {
        self.orphan_grace_secs
    }
    pub fn sweep_interval_secs(&self) -> u64 {
        self.sweep_interval_secs
    }
    pub fn cleanup_max_attempts(&self) -> u32 {
        self.cleanup_max_attempts
    }
    pub fn cleanup_backoff_secs(&self) -> u64 {
        self.cleanup_backoff_secs
    }
    pub fn job_deadline_secs(&self) -> u64 {
        self.job_deadline_secs
    }
    pub fn max_concurrent_jobs(&self) -> u32 {
        self.max_concurrent_jobs
    }
    pub fn alloc_max_bytes(&self) -> u64 {
        self.alloc_max_bytes
    }
    pub fn total_max_bytes(&self) -> u64 {
        self.total_max_bytes
    }
    pub fn retention_days(&self) -> u32 {
        self.retention_days
    }
    pub fn escalation_owner(&self) -> &str {
        &self.escalation_owner
    }
}

impl BackupConfig {
    pub fn instance_id(&self) -> &str {
        &self.instance_id
    }
    pub fn instance_label(&self) -> &str {
        &self.instance_label
    }
    pub fn storage_root(&self) -> &Path {
        &self.storage_root
    }
    pub fn bounds(&self) -> &BackupBounds {
        &self.bounds
    }
    pub fn allowed_origins(&self) -> &[String] {
        &self.allowed_origins
    }
    pub fn request_timeout_secs(&self) -> u64 {
        self.request_timeout_secs
    }
    pub fn max_header_bytes(&self) -> u64 {
        self.max_header_bytes
    }
}

impl BackupConfigState {
    /// Closed-set wire value of the disabled reason; `None` when enabled (JSON null).
    pub fn disabled_reason(&self) -> Option<&'static str> {
        match self {
            Self::Disabled { reason } => Some(reason.as_str()),
            Self::Enabled(_) => None,
        }
    }
}

/// Parse a canonical unsigned decimal: ASCII digits only, no sign, padding or leading zero.
fn parse_canonical<T: std::str::FromStr>(value: &str) -> Option<T> {
    let canonical = !value.is_empty()
        && value.bytes().all(|b| b.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'));
    if canonical {
        value.parse().ok()
    } else {
        None
    }
}

/// An optional bound key: absent means the default; present must spell the default exactly.
fn bound_matches_default<T>(value: Option<String>, default: T) -> bool
where
    T: std::str::FromStr + PartialEq,
{
    match value {
        None => true,
        Some(v) => parse_canonical::<T>(&v).is_some_and(|n| n == default),
    }
}

/// A required, non-zero canonical number (CFG-17, CFG-18).
fn required_positive(value: Option<String>) -> Option<u64> {
    value
        .and_then(|v| parse_canonical::<u64>(&v))
        .filter(|n| *n > 0)
}

/// A required value: non-empty and without leading or trailing whitespace.
fn required_text(value: Option<String>) -> Option<String> {
    value.filter(|v| !v.is_empty() && v.trim() == v)
}

/// One exact origin: `http(s)://host[:port]`, no path, wildcard or whitespace; port 1..=65535 (CFG-16).
fn is_valid_origin(origin: &str) -> bool {
    let Some(rest) = origin
        .strip_prefix("http://")
        .or_else(|| origin.strip_prefix("https://"))
    else {
        return false;
    };
    // Bracketed IPv6 origins are out of scope for U1.
    let (host, port) = match rest.split_once(':') {
        Some((host, port)) => (host, Some(port)),
        None => (rest, None),
    };
    let host_ok = host.split('.').all(|label| {
        !label.is_empty()
            && !label.starts_with('-')
            && !label.ends_with('-')
            && label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
    });
    let port_ok = port.is_none_or(|p| parse_canonical::<u16>(p).is_some_and(|n| n > 0));
    host_ok && port_ok
}

fn parse_origins(value: Option<String>) -> Option<Vec<String>> {
    let origins: Vec<String> = value?.split(',').map(str::to_string).collect();
    origins
        .iter()
        .all(|o| is_valid_origin(o))
        .then_some(origins)
}

impl BackupConfig {
    /// Load from an injected key lookup (no process-env mutation in tests).
    ///
    /// Reason precedence (F-01): NOT_CONFIGURED, then NON_LOOPBACK_BIND, then INVALID_CONFIG.
    pub fn load<F>(lookup: F) -> BackupConfigState
    where
        F: Fn(&str) -> Option<String>,
    {
        let disabled = |reason| BackupConfigState::Disabled { reason };

        if lookup("BACKUP_ENABLED").as_deref() != Some("true") {
            return disabled(DisabledReason::NotConfigured);
        }
        let host = lookup("HOST").unwrap_or_else(|| "127.0.0.1".to_string());
        if !is_loopback_bind(&host) {
            return disabled(DisabledReason::NonLoopbackBind);
        }
        match Self::parse(&lookup) {
            Some(config) => BackupConfigState::Enabled(config),
            None => disabled(DisabledReason::InvalidConfig),
        }
    }

    /// Validate every key of an enabled configuration; `None` means INVALID_CONFIG.
    fn parse<F>(lookup: &F) -> Option<BackupConfig>
    where
        F: Fn(&str) -> Option<String>,
    {
        let b = BackupBounds::default();
        let bounds_ok =
            bound_matches_default(lookup("BACKUP_STAGING_EXPIRY_SECS"), b.staging_expiry_secs)
                && bound_matches_default(lookup("BACKUP_ORPHAN_GRACE_SECS"), b.orphan_grace_secs)
                && bound_matches_default(
                    lookup("BACKUP_SWEEP_INTERVAL_SECS"),
                    b.sweep_interval_secs,
                )
                && bound_matches_default(
                    lookup("BACKUP_CLEANUP_MAX_ATTEMPTS"),
                    b.cleanup_max_attempts,
                )
                && bound_matches_default(
                    lookup("BACKUP_CLEANUP_BACKOFF_SECS"),
                    b.cleanup_backoff_secs,
                )
                && bound_matches_default(lookup("BACKUP_JOB_DEADLINE_SECS"), b.job_deadline_secs)
                && bound_matches_default(
                    lookup("BACKUP_MAX_CONCURRENT_JOBS"),
                    b.max_concurrent_jobs,
                )
                && bound_matches_default(lookup("BACKUP_ALLOC_MAX_BYTES"), b.alloc_max_bytes)
                && bound_matches_default(lookup("BACKUP_TOTAL_MAX_BYTES"), b.total_max_bytes)
                && bound_matches_default(lookup("BACKUP_RETENTION_DAYS"), b.retention_days)
                && lookup("BACKUP_ESCALATION_OWNER")
                    .is_none_or(|owner| owner == b.escalation_owner);
        if !bounds_ok {
            return None;
        }

        let storage_root = PathBuf::from(required_text(lookup("BACKUP_STORAGE_ROOT"))?);
        if validate_storage_root(&storage_root, &[]).is_err() {
            return None;
        }

        Some(BackupConfig {
            instance_id: required_text(lookup("BACKUP_INSTANCE_ID"))?,
            instance_label: required_text(lookup("BACKUP_INSTANCE_LABEL"))?,
            storage_root,
            bounds: b,
            allowed_origins: parse_origins(lookup("BACKUP_ALLOWED_ORIGINS"))?,
            request_timeout_secs: required_positive(lookup("BACKUP_REQUEST_TIMEOUT_SECS"))?,
            max_header_bytes: required_positive(lookup("BACKUP_MAX_HEADER_BYTES"))?,
        })
    }

    /// Thin wrapper over `load` using the process environment.
    pub fn from_env() -> BackupConfigState {
        // A present but non-UTF-8 value must fail validation rather than count as absent.
        Self::load(|key| std::env::var_os(key).map(|value| value.into_string().unwrap_or_default()))
    }
}

/// F-39: true only for an IP literal in 127.0.0.0/8 or ::1.
pub fn is_loopback_bind(host: &str) -> bool {
    host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback())
}

/// Why a storage root was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageRootError {
    NotAbsolute,
    UnderForbiddenPath,
    ReparsePoint,
    /// Contains `..` or a verbatim/device prefix, so the real location cannot be compared.
    UnresolvedPath,
    /// Exists but is not a directory.
    NotDirectory,
    /// Metadata could not be read (and the error was not "missing").
    Inaccessible,
    /// Unix only: root not mode 0700 (TS-02 definition).
    NotPrivate,
}

fn is_reparse_point(meta: &std::fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        if meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return true;
        }
    }
    meta.file_type().is_symlink()
}

/// Compare path components as the platform file system would (case-insensitive on Windows).
fn is_under(root: &Path, forbidden: &Path) -> bool {
    #[cfg(windows)]
    {
        let fold = |p: &Path| -> Vec<String> {
            p.components()
                .map(|c| c.as_os_str().to_string_lossy().to_lowercase())
                .collect()
        };
        fold(root).starts_with(&fold(forbidden))
    }
    #[cfg(not(windows))]
    {
        root.starts_with(forbidden)
    }
}

/// CFG-04 check: absolute, free of `..` and verbatim prefixes, no reparse point or symlink on any
/// existing component, a private directory, and not equal to or beneath any `forbidden` entry.
pub fn validate_storage_root(root: &Path, forbidden: &[PathBuf]) -> Result<(), StorageRootError> {
    use std::path::{Component, Prefix};

    if !root.is_absolute() {
        return Err(StorageRootError::NotAbsolute);
    }
    for component in root.components() {
        match component {
            Component::ParentDir => return Err(StorageRootError::UnresolvedPath),
            Component::Prefix(p)
                if matches!(
                    p.kind(),
                    Prefix::Verbatim(_)
                        | Prefix::VerbatimUNC(..)
                        | Prefix::VerbatimDisk(_)
                        | Prefix::DeviceNS(_)
                ) =>
            {
                return Err(StorageRootError::UnresolvedPath)
            }
            _ => {}
        }
    }
    if forbidden.iter().any(|f| is_under(root, f)) {
        return Err(StorageRootError::UnderForbiddenPath);
    }
    // Every existing ancestor must be link-free; a missing one is the storage layer's to create.
    for path in root.ancestors() {
        match std::fs::symlink_metadata(path) {
            Ok(meta) => {
                if is_reparse_point(&meta) {
                    return Err(StorageRootError::ReparsePoint);
                }
                if path == root {
                    if !meta.is_dir() {
                        return Err(StorageRootError::NotDirectory);
                    }
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if meta.permissions().mode() & 0o777 != 0o700 {
                            return Err(StorageRootError::NotPrivate);
                        }
                    }
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => return Err(StorageRootError::Inaccessible),
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    type Env = HashMap<String, String>;

    fn test_root() -> String {
        crate::test_temp_dir()
            .join("makara-backup-config-test-root")
            .to_string_lossy()
            .into_owned()
    }

    /// Enabled, otherwise valid configuration with no bound keys set (defaults apply).
    fn valid_env() -> Env {
        [
            ("BACKUP_ENABLED", "true".to_string()),
            ("BACKUP_INSTANCE_ID", "local-1".to_string()),
            ("BACKUP_INSTANCE_LABEL", "Local instance".to_string()),
            ("BACKUP_STORAGE_ROOT", test_root()),
            (
                "BACKUP_ALLOWED_ORIGINS",
                "http://localhost:3000".to_string(),
            ),
            ("BACKUP_REQUEST_TIMEOUT_SECS", "30".to_string()),
            ("BACKUP_MAX_HEADER_BYTES", "16384".to_string()),
        ]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect()
    }

    fn load(env: &Env) -> BackupConfigState {
        BackupConfig::load(|k| env.get(k).cloned())
    }

    fn with(key: &str, value: &str) -> Env {
        let mut env = valid_env();
        env.insert(key.to_string(), value.to_string());
        env
    }

    fn without(key: &str) -> Env {
        let mut env = valid_env();
        env.remove(key);
        env
    }

    fn assert_invalid(env: &Env, what: &str) {
        assert_eq!(
            load(env),
            BackupConfigState::Disabled {
                reason: DisabledReason::InvalidConfig
            },
            "{what}"
        );
    }

    fn assert_enabled(env: &Env, what: &str) -> BackupConfig {
        match load(env) {
            BackupConfigState::Enabled(cfg) => cfg,
            other => panic!("{what}: expected Enabled, got {other:?}"),
        }
    }

    /// TS-01, Q-05, T-13: owner-approved values exactly (CFG-05..CFG-15).
    #[test]
    fn defaults_match_owner_values() {
        let b = BackupBounds::default();
        assert_eq!(b.staging_expiry_secs, 24 * 60 * 60);
        assert_eq!(b.orphan_grace_secs, 60 * 60);
        assert_eq!(b.sweep_interval_secs, 15 * 60);
        assert_eq!(b.cleanup_max_attempts, 3);
        assert_eq!(b.cleanup_backoff_secs, 60);
        assert_eq!(b.job_deadline_secs, 10 * 60);
        assert_eq!(b.max_concurrent_jobs, 1);
        assert_eq!(b.alloc_max_bytes, 256 * 1024 * 1024);
        assert_eq!(b.total_max_bytes, 1024 * 1024 * 1024);
        assert_eq!(b.retention_days, 30);
        assert_eq!(b.escalation_owner, "project_owner");

        // The same values must come out of the environment path when no bound key is set.
        let cfg = assert_enabled(&valid_env(), "defaults via load");
        assert_eq!(cfg.bounds, b);
    }

    fn not_configured() -> BackupConfigState {
        BackupConfigState::Disabled {
            reason: DisabledReason::NotConfigured,
        }
    }

    /// TS-02: enabled flag absent or not exactly `true` gives NOT_CONFIGURED; `true` enables.
    /// Strict matching [D]: `TRUE`, `1`, `yes`, empty and padded values are "not true".
    #[test]
    fn disabled_unless_backup_enabled_is_true() {
        assert_enabled(&valid_env(), "BACKUP_ENABLED=true");
        let mut envs = vec![without("BACKUP_ENABLED")];
        for v in ["false", "TRUE", "True", "1", "yes", "", " true", "true "] {
            envs.push(with("BACKUP_ENABLED", v));
        }
        for env in envs {
            assert_eq!(
                load(&env),
                not_configured(),
                "{:?}",
                env.get("BACKUP_ENABLED")
            );
        }
    }

    /// TS-02, FD F-01 main flow: NOT_CONFIGURED is decided first, before bind and key validation.
    #[test]
    fn not_configured_takes_precedence() {
        // (b) fully empty environment (default deployment): NOT_CONFIGURED, never INVALID_CONFIG.
        assert_eq!(
            BackupConfig::load(|_| None),
            not_configured(),
            "empty environment"
        );
        // (a) flag absent or false with a non-loopback HOST: F-01 order gives NOT_CONFIGURED.
        let mut absent = without("BACKUP_ENABLED");
        absent.insert("HOST".into(), "0.0.0.0".into());
        assert_eq!(load(&absent), not_configured(), "absent flag + 0.0.0.0");
        let mut falsy = with("BACKUP_ENABLED", "false");
        falsy.insert("HOST".into(), "0.0.0.0".into());
        assert_eq!(load(&falsy), not_configured(), "false flag + 0.0.0.0");
        // Flag false with an invalid bound is still NOT_CONFIGURED.
        let mut bad = with("BACKUP_ENABLED", "false");
        bad.insert("BACKUP_JOB_DEADLINE_SECS".into(), "601".into());
        assert_eq!(load(&bad), not_configured(), "flag false + invalid bound");
    }

    /// TS-02 / TS-08a [Q]: enabled + non-loopback HOST + invalid key. No design source orders
    /// these two reasons; this test pins NON_LOOPBACK_BIND first because the bind guard is a
    /// security control. Conductor/owner to confirm.
    #[test]
    fn non_loopback_bind_takes_precedence_over_invalid_config() {
        let mut env = with("BACKUP_JOB_DEADLINE_SECS", "601");
        env.insert("HOST".into(), "0.0.0.0".into());
        assert_eq!(
            load(&env),
            BackupConfigState::Disabled {
                reason: DisabledReason::NonLoopbackBind
            }
        );
    }

    /// TS-02: the closed set includes null; Enabled projects to None, each Disabled to its string.
    #[test]
    fn disabled_reason_projection_covers_null() {
        assert_eq!(load(&valid_env()).disabled_reason(), None);
        assert_eq!(
            load(&without("BACKUP_ENABLED")).disabled_reason(),
            Some("NOT_CONFIGURED")
        );
        assert_eq!(
            load(&with("HOST", "0.0.0.0")).disabled_reason(),
            Some("NON_LOOPBACK_BIND")
        );
        assert_eq!(
            load(&without("BACKUP_STORAGE_ROOT")).disabled_reason(),
            Some("INVALID_CONFIG")
        );
        // HOST present but empty is a non-loopback value (F-39), not the 127.0.0.1 default.
        assert_eq!(
            load(&with("HOST", "")).disabled_reason(),
            Some("NON_LOOPBACK_BIND")
        );
    }

    /// TS-02 / FD F-01 / IFC 8: invalid config never panics and never becomes an Err; it is
    /// always `Disabled`. Table-driven over every bad value. The gateway-still-starts and
    /// 503 BACKUP_DISABLED part (validate_config returns Ok) is deferred to WU-08/WU-10.
    #[test]
    fn invalid_config_always_yields_disabled_state() {
        let mut bad_envs: Vec<(String, Env)> = vec![];
        for (key, _, below, above) in BOUND_CASES {
            for v in [
                below.to_string(),
                above.to_string(),
                "abc".into(),
                "".into(),
                "0".into(),
            ] {
                bad_envs.push((format!("{key}={v:?}"), with(key, &v)));
            }
        }
        for key in [
            "BACKUP_STORAGE_ROOT",
            "BACKUP_INSTANCE_ID",
            "BACKUP_INSTANCE_LABEL",
            "BACKUP_ALLOWED_ORIGINS",
            "BACKUP_REQUEST_TIMEOUT_SECS",
            "BACKUP_MAX_HEADER_BYTES",
        ] {
            bad_envs.push((format!("{key} absent"), without(key)));
            bad_envs.push((format!("{key} empty"), with(key, "")));
        }
        for (what, env) in bad_envs {
            let state = std::panic::catch_unwind(|| load(&env))
                .unwrap_or_else(|_| panic!("{what}: load must not panic"));
            assert!(
                matches!(state, BackupConfigState::Disabled { .. }),
                "{what}: expected Disabled, got {state:?}"
            );
        }
    }

    /// TS-02 / TS-08a wiring: HOST defaults to 127.0.0.1 and non-loopback HOST disables.
    #[test]
    fn host_default_is_loopback_and_non_loopback_host_disables() {
        assert_enabled(&without("HOST"), "HOST absent defaults to loopback");
        assert_enabled(&with("HOST", "127.0.0.1"), "HOST 127.0.0.1");
        for host in ["0.0.0.0", "::", "localhost", "192.168.1.10"] {
            assert_eq!(
                load(&with("HOST", host)),
                BackupConfigState::Disabled {
                    reason: DisabledReason::NonLoopbackBind
                },
                "HOST={host}"
            );
        }
    }

    /// TS-02: closed disabled_reason set spelling.
    #[test]
    fn disabled_reason_closed_set_spelling() {
        assert_eq!(DisabledReason::NotConfigured.as_str(), "NOT_CONFIGURED");
        assert_eq!(
            DisabledReason::NonLoopbackBind.as_str(),
            "NON_LOOPBACK_BIND"
        );
        assert_eq!(DisabledReason::InvalidConfig.as_str(), "INVALID_CONFIG");
    }

    /// TS-02: CFG-02..CFG-04 required when enabled; storage root must be absolute.
    #[test]
    fn enabled_without_storage_root_refuses() {
        assert_invalid(&without("BACKUP_STORAGE_ROOT"), "CFG-04 missing");
        assert_invalid(&with("BACKUP_STORAGE_ROOT", ""), "CFG-04 empty");
        assert_invalid(
            &with("BACKUP_STORAGE_ROOT", "relative/dir"),
            "CFG-04 relative",
        );
        assert_invalid(&without("BACKUP_INSTANCE_ID"), "CFG-02 missing");
        assert_invalid(&without("BACKUP_INSTANCE_LABEL"), "CFG-03 missing");
        for (key, bad) in [
            ("BACKUP_INSTANCE_ID", ""),
            ("BACKUP_INSTANCE_ID", "   "),
            ("BACKUP_INSTANCE_LABEL", ""),
            ("BACKUP_INSTANCE_LABEL", "   "),
        ] {
            assert_invalid(&with(key, bad), &format!("{key}={bad:?} malformed"));
        }
        let cfg = assert_enabled(&valid_env(), "valid baseline");
        assert_eq!(cfg.storage_root, PathBuf::from(test_root()));
        assert_eq!(cfg.instance_id, "local-1");
        assert_eq!(cfg.instance_label, "Local instance");
    }

    /// Scratch directory removed on drop (also on panic). Links are removed before the tree
    /// so cleanup never follows them.
    struct Scratch {
        dir: PathBuf,
        links: Vec<PathBuf>,
    }

    impl Scratch {
        fn new(name: &str) -> Self {
            let dir = crate::test_temp_dir()
                .join(format!("makara-backup-wu01-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&dir);
            std::fs::create_dir_all(&dir).unwrap();
            Scratch { dir, links: vec![] }
        }

        fn link(&mut self, name: &str, target: &Path) -> PathBuf {
            let link = self.dir.join(name);
            make_dir_link(&link, target);
            self.links.push(link.clone());
            link
        }
    }

    impl Drop for Scratch {
        fn drop(&mut self) {
            for l in &self.links {
                let _ = std::fs::remove_dir(l);
                #[cfg(unix)]
                let _ = std::fs::remove_file(l);
            }
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }

    #[cfg(unix)]
    fn set_mode(p: &Path, mode: u32) {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(p, std::fs::Permissions::from_mode(mode)).unwrap();
    }

    /// Create a directory link (symlink on Unix, junction on Windows) at `link` to `target`.
    fn make_dir_link(link: &Path, target: &Path) {
        #[cfg(unix)]
        std::os::unix::fs::symlink(target, link).unwrap();
        #[cfg(windows)]
        {
            let out = std::process::Command::new("cmd")
                .args(["/C", "mklink", "/J"])
                .arg(link)
                .arg(target)
                .output()
                .unwrap();
            assert!(out.status.success(), "mklink /J failed");
        }
    }

    /// TS-02 definition, T-14: forbidden set is an explicit input.
    /// T-14 is only partially evidenced on Windows: no owner-only ACL check, so only the
    /// absolute / reparse-point / forbidden-set rules apply there.
    #[test]
    fn storage_root_rejects_forbidden_and_relative_paths() {
        let mut sc = Scratch::new("root");
        let t = sc.dir.clone();
        let repo = t.join("repo");
        let data = repo.join("data");
        let public_x = repo.join("apps").join("web").join("public").join("x");
        let private_root = t.join("private").join("root");
        let allowed_target = t.join("private").join("target");
        for d in [&data, &public_x, &private_root, &allowed_target] {
            std::fs::create_dir_all(d).unwrap();
        }
        #[cfg(unix)]
        {
            set_mode(&private_root, 0o700);
            set_mode(&allowed_target, 0o700);
        }
        let link_forbidden = sc.link("link-to-repo", &repo);
        let link_allowed = sc.link("link-to-allowed", &allowed_target);
        let forbidden = vec![repo.clone()];

        let results = (
            validate_storage_root(&repo, &forbidden),
            validate_storage_root(&data, &forbidden),
            validate_storage_root(&public_x, &forbidden),
            validate_storage_root(Path::new("relative/dir"), &forbidden),
            validate_storage_root(&link_forbidden, &forbidden),
            validate_storage_root(&private_root, &forbidden),
            validate_storage_root(&link_allowed, &forbidden),
        );

        assert_eq!(
            results.0,
            Err(StorageRootError::UnderForbiddenPath),
            "equal to forbidden"
        );
        assert_eq!(
            results.1,
            Err(StorageRootError::UnderForbiddenPath),
            "beneath forbidden"
        );
        assert_eq!(
            results.2,
            Err(StorageRootError::UnderForbiddenPath),
            "web/public child"
        );
        assert_eq!(results.3, Err(StorageRootError::NotAbsolute), "relative");
        assert!(
            matches!(
                results.4,
                Err(StorageRootError::ReparsePoint) | Err(StorageRootError::UnderForbiddenPath)
            ),
            "link to forbidden dir must be rejected as reparse point or forbidden, got {:?}",
            results.4
        );
        assert_eq!(results.5, Ok(()), "private root accepted");
        assert_eq!(
            results.6,
            Err(StorageRootError::ReparsePoint),
            "link/junction to an allowed directory is still a reparse point"
        );
    }

    /// TS-02 definition, Unix only: "private" means the root is mode 0700.
    #[cfg(unix)]
    #[test]
    fn storage_root_must_be_private_on_unix() {
        let sc = Scratch::new("mode");
        let root = sc.dir.join("root");
        std::fs::create_dir_all(&root).unwrap();
        set_mode(&root, 0o755);
        assert_eq!(
            validate_storage_root(&root, &[]),
            Err(StorageRootError::NotPrivate)
        );
        set_mode(&root, 0o700);
        assert_eq!(validate_storage_root(&root, &[]), Ok(()));
    }

    /// (key, default, below, above) for CFG-05..CFG-14.
    const BOUND_CASES: [(&str, u64, u64, u64); 10] = [
        ("BACKUP_STAGING_EXPIRY_SECS", 86_400, 86_399, 86_401),
        ("BACKUP_ORPHAN_GRACE_SECS", 3_600, 3_599, 3_601),
        ("BACKUP_SWEEP_INTERVAL_SECS", 900, 899, 901),
        ("BACKUP_CLEANUP_MAX_ATTEMPTS", 3, 2, 4),
        ("BACKUP_CLEANUP_BACKOFF_SECS", 60, 59, 61),
        ("BACKUP_JOB_DEADLINE_SECS", 600, 599, 601),
        ("BACKUP_MAX_CONCURRENT_JOBS", 1, 0, 2),
        (
            "BACKUP_ALLOC_MAX_BYTES",
            268_435_456,
            268_435_455,
            268_435_457,
        ),
        (
            "BACKUP_TOTAL_MAX_BYTES",
            1_073_741_824,
            1_073_741_823,
            1_073_741_825,
        ),
        ("BACKUP_RETENTION_DAYS", 30, 29, 31),
    ];

    /// TS-02: non-numeric, empty, negative, fractional or zero bound values refuse the capability.
    #[test]
    fn invalid_numeric_env_refuses_capability() {
        for (key, _, _, _) in BOUND_CASES {
            for bad in ["abc", "", "-1", "1.5", "0"] {
                assert_invalid(&with(key, bad), &format!("{key}={bad:?}"));
            }
        }
    }

    /// TS-02 [D]: only canonical decimal digits are accepted (no sign, no leading zero, no
    /// padding), and overflow is invalid. Spelling variants of the default are NOT the default.
    #[test]
    fn non_canonical_and_overflowing_numbers_are_invalid() {
        for (key, default, _, _) in BOUND_CASES {
            for bad in [
                format!("+{default}"),
                format!("0{default}"),
                format!(" {default}"),
                format!("{default} "),
                "18446744073709551616".to_string(),
                "4294967296".to_string(),
            ] {
                assert_invalid(&with(key, &bad), &format!("{key}={bad:?}"));
            }
        }
        for key in ["BACKUP_REQUEST_TIMEOUT_SECS", "BACKUP_MAX_HEADER_BYTES"] {
            for bad in ["+30", "030", " 30", "30 ", "18446744073709551616"] {
                assert_invalid(&with(key, bad), &format!("{key}={bad:?}"));
            }
        }
    }

    /// TS-02 / Q-05 [Q]: CFG-15 is the owner decision `project_owner`; the environment must
    /// not override it. Either the key is ignored (owner stays project_owner) or the config is
    /// INVALID_CONFIG; any other outcome fails.
    #[test]
    fn escalation_owner_cannot_be_overridden() {
        match load(&with("BACKUP_ESCALATION_OWNER", "someone_else")) {
            BackupConfigState::Enabled(cfg) => {
                assert_eq!(cfg.bounds.escalation_owner, "project_owner")
            }
            BackupConfigState::Disabled { reason } => {
                assert_eq!(reason, DisabledReason::InvalidConfig)
            }
        }
    }

    /// TS-02, FD F-01 bounds rule: only the owner-approved default is valid, per key.
    #[test]
    fn bounds_other_than_default_disable_with_invalid_config() {
        for (key, default, below, above) in BOUND_CASES {
            let cfg = assert_enabled(&with(key, &default.to_string()), &format!("{key} default"));
            assert_eq!(
                cfg.bounds,
                BackupBounds::default(),
                "{key} default keeps owner bounds"
            );
            assert_invalid(
                &with(key, &below.to_string()),
                &format!("{key} below default"),
            );
            assert_invalid(
                &with(key, &above.to_string()),
                &format!("{key} above default"),
            );
        }
        // CFG-15 is fixed to project_owner.
        assert_eq!(
            assert_enabled(&valid_env(), "baseline")
                .bounds
                .escalation_owner,
            "project_owner"
        );
    }

    /// TS-02: CFG-17 and CFG-18 have no approved default; absent or malformed is INVALID_CONFIG.
    #[test]
    fn ingress_limits_required() {
        for key in ["BACKUP_REQUEST_TIMEOUT_SECS", "BACKUP_MAX_HEADER_BYTES"] {
            assert_invalid(&without(key), &format!("{key} absent"));
            for bad in ["", "0", "abc", "-1", "1.5"] {
                assert_invalid(&with(key, bad), &format!("{key}={bad:?}"));
            }
        }
        let cfg = assert_enabled(&valid_env(), "valid ingress limits");
        assert_eq!(cfg.request_timeout_secs, 30);
        assert_eq!(cfg.max_header_bytes, 16_384);
    }

    /// TS-02, T-04: CFG-16 required, exact origins, no wildcard.
    #[test]
    fn allowed_origins_required_and_well_formed() {
        assert_invalid(&without("BACKUP_ALLOWED_ORIGINS"), "missing");
        for bad in [
            "",
            "*",
            "http://*.example.com",
            "localhost:3000",
            "not an origin",
            "http://localhost:3000/some/path",
            "http://localhost:3000,*",
            "http://localhost:3000/",
        ] {
            assert_invalid(
                &with("BACKUP_ALLOWED_ORIGINS", bad),
                &format!("origins={bad:?}"),
            );
        }
        let one = assert_enabled(&valid_env(), "single origin");
        assert_eq!(
            one.allowed_origins,
            vec!["http://localhost:3000".to_string()]
        );
        let two = assert_enabled(
            &with(
                "BACKUP_ALLOWED_ORIGINS",
                "http://localhost:3000,http://127.0.0.1:3000",
            ),
            "two origins",
        );
        assert_eq!(
            two.allowed_origins,
            vec![
                "http://localhost:3000".to_string(),
                "http://127.0.0.1:3000".to_string()
            ]
        );
    }

    /// TS-08a, AC-005 S1, T-04, F-39: loopback IP literals pass; everything else fails.
    /// One test holds both halves so it is red against any permissive or restrictive stub.
    /// Pinned [D]: `[::1]` (bracketed) and `::ffff:127.0.0.1` (IPv4-mapped) are not bare
    /// 127.0.0.0/8 or ::1 literals, so they are non-loopback.
    #[test]
    fn is_loopback_bind_accepts_only_loopback_ip_literals() {
        for host in [
            "127.0.0.1",
            "127.0.0.2",
            "127.255.255.255",
            "::1",
            "0:0:0:0:0:0:0:1",
        ] {
            assert!(is_loopback_bind(host), "{host} must be loopback");
        }
        for host in [
            "0.0.0.0",
            "::",
            "localhost",
            "example.com",
            "192.168.1.10",
            "10.0.0.1",
            "8.8.8.8",
            "2001:db8::1",
            "",
            "not-an-ip",
            "127.0.0.1:8080",
            "[::1]",
            "::ffff:127.0.0.1",
        ] {
            assert!(!is_loopback_bind(host), "{host:?} must not be loopback");
        }
    }

    /// Unix needs the root private for validation to get past the mode check.
    fn make_private_dir(p: &Path) {
        std::fs::create_dir_all(p).unwrap();
        #[cfg(unix)]
        set_mode(p, 0o700);
    }

    /// CFG-04, T-14 (behaviour 1): a link on an ANCESTOR of the root is refused, not only on the leaf.
    #[test]
    fn storage_root_rejects_link_on_ancestor() {
        let mut sc = Scratch::new("ancestor");
        let real = sc.dir.join("real");
        make_private_dir(&real.join("sub"));
        let link = sc.link("lnk", &real);
        let existing_child = link.join("sub");
        let missing_child = link.join("missing").join("deeper");
        assert_eq!(
            validate_storage_root(&existing_child, &[]),
            Err(StorageRootError::ReparsePoint),
            "existing root under a linked ancestor"
        );
        assert_eq!(
            validate_storage_root(&missing_child, &[]),
            Err(StorageRootError::ReparsePoint),
            "missing root under a linked ancestor"
        );
        assert_eq!(validate_storage_root(&real.join("sub"), &[]), Ok(()));
    }

    /// CFG-04, T-14 (behaviour 2): `..` makes the real location unknowable.
    #[test]
    fn storage_root_rejects_parent_dir_component() {
        let sc = Scratch::new("parentdir");
        let root = sc.dir.join("a").join("..").join("b");
        assert_eq!(
            validate_storage_root(&root, &[]),
            Err(StorageRootError::UnresolvedPath)
        );
    }

    /// CFG-04, T-14 (behaviour 2, Windows): verbatim prefixes are not comparable.
    #[cfg(windows)]
    #[test]
    fn storage_root_rejects_verbatim_prefix_on_windows() {
        let sc = Scratch::new("verbatim");
        let root = PathBuf::from(format!(r"\\?\{}", sc.dir.join("root").display()));
        assert!(root.is_absolute());
        assert_eq!(
            validate_storage_root(&root, &[]),
            Err(StorageRootError::UnresolvedPath)
        );
    }

    /// CFG-04, T-14 (behaviour 3, Windows): forbidden comparison ignores case.
    #[cfg(windows)]
    #[test]
    fn forbidden_comparison_is_case_insensitive_on_windows() {
        let forbidden = vec![PathBuf::from(r"C:\Repo")];
        assert_eq!(
            validate_storage_root(Path::new(r"c:\repo\data"), &forbidden),
            Err(StorageRootError::UnderForbiddenPath)
        );
        assert_eq!(
            validate_storage_root(Path::new(r"C:\REPO"), &forbidden),
            Err(StorageRootError::UnderForbiddenPath)
        );
        assert_eq!(
            validate_storage_root(Path::new(r"c:\repository\data"), &forbidden),
            Ok(()),
            "a sibling sharing a prefix is not under the forbidden path"
        );
    }

    /// CFG-04, T-14 (behaviour 4): an existing regular file cannot be a root.
    #[test]
    fn storage_root_rejects_existing_file() {
        let sc = Scratch::new("file");
        let file = sc.dir.join("not-a-dir");
        std::fs::write(&file, b"x").unwrap();
        assert_eq!(
            validate_storage_root(&file, &[]),
            Err(StorageRootError::NotDirectory)
        );
    }

    /// CFG-04, T-14 (behaviour 5): a metadata error other than NotFound is Inaccessible.
    /// An interior NUL byte makes the metadata call fail with InvalidInput on every platform.
    #[test]
    fn storage_root_metadata_error_is_inaccessible() {
        let sc = Scratch::new("nul");
        let root = sc.dir.join("bad\0name");
        assert!(root.is_absolute());
        assert_eq!(
            validate_storage_root(&root, &[]),
            Err(StorageRootError::Inaccessible)
        );
    }

    /// AC-003 S3, T-11 (behaviour 6): padded required text is INVALID_CONFIG, via injected lookup.
    #[test]
    fn padded_required_text_is_invalid_config() {
        for key in [
            "BACKUP_INSTANCE_ID",
            "BACKUP_INSTANCE_LABEL",
            "BACKUP_STORAGE_ROOT",
        ] {
            for (pre, post) in [(" ", ""), ("", " "), (" ", " "), ("\t", "\n")] {
                let core = if key == "BACKUP_STORAGE_ROOT" {
                    test_root()
                } else {
                    "local-1".to_string()
                };
                let value = format!("{pre}{core}{post}");
                assert_invalid(&with(key, &value), &format!("{key}={value:?}"));
            }
        }
        assert_enabled(&with("BACKUP_INSTANCE_ID", "local-1"), "unpadded control");
    }

    /// Probe run in a child process by `from_env_non_utf8_value_fails_validation` (inert alone).
    #[test]
    fn from_env_child_probe() {
        let Ok(mode) = std::env::var("MAKARA_CFG_CHILD") else {
            return;
        };
        let state = BackupConfig::from_env();
        match mode.as_str() {
            "invalid" => assert_eq!(
                state,
                BackupConfigState::Disabled {
                    reason: DisabledReason::InvalidConfig
                }
            ),
            "valid" => assert!(matches!(state, BackupConfigState::Enabled(_)), "{state:?}"),
            other => panic!("unknown mode {other}"),
        }
        println!("CHILD_PROBE_RAN {mode}");
    }

    /// AC-003 S3, T-11 (behaviour 6): a present but non-UTF-8 value is invalid, not absent.
    /// Runs `from_env` in a child process so this process's environment is never mutated.
    #[test]
    fn from_env_non_utf8_value_fails_validation() {
        use std::ffi::OsString;
        #[cfg(windows)]
        let bad: OsString = {
            use std::os::windows::ffi::OsStringExt;
            OsString::from_wide(&[0x61, 0xD800, 0x62])
        };
        #[cfg(unix)]
        let bad: OsString = {
            use std::os::unix::ffi::OsStringExt;
            OsString::from_vec(vec![0x61, 0xFF, 0x62])
        };
        let run = |mode: &str, instance_id: &OsString| {
            let mut cmd = std::process::Command::new(std::env::current_exe().unwrap());
            cmd.arg("--exact")
                .arg(format!(
                    "{}::from_env_child_probe",
                    module_path!().split_once("::").unwrap().1
                ))
                .arg("--nocapture")
                .arg("--test-threads=1")
                .env_remove("HOST")
                .env("MAKARA_CFG_CHILD", mode);
            for (k, v) in valid_env() {
                cmd.env(k, v);
            }
            cmd.env("BACKUP_INSTANCE_ID", instance_id);
            let out = cmd.output().unwrap();
            let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
            assert!(
                out.status.success() && stdout.contains(&format!("CHILD_PROBE_RAN {mode}")),
                "child {mode} failed: {stdout}{}",
                String::from_utf8_lossy(&out.stderr)
            );
        };
        run("valid", &OsString::from("local-1"));
        run("invalid", &bad);
    }

    /// AC-003 S3, T-11, T-14 (behaviour 7): `load` refuses a link or file as storage root.
    #[test]
    fn load_refuses_link_or_file_storage_root() {
        let mut sc = Scratch::new("loadroot");
        let real = sc.dir.join("real");
        make_private_dir(&real);
        let link = sc.link("lnk", &real);
        let under_link = link.join("sub");
        let file = sc.dir.join("a-file");
        std::fs::write(&file, b"x").unwrap();
        let root = |p: &Path| p.to_string_lossy().into_owned();
        assert_enabled(
            &with("BACKUP_STORAGE_ROOT", &root(&real)),
            "real dir control",
        );
        assert_invalid(&with("BACKUP_STORAGE_ROOT", &root(&link)), "link root");
        assert_invalid(
            &with("BACKUP_STORAGE_ROOT", &root(&under_link)),
            "root beneath a link",
        );
        assert_invalid(&with("BACKUP_STORAGE_ROOT", &root(&file)), "file root");
    }

    /// AC-003 S3, T-11 (behaviour 8): origin port must be canonical and within 1..=65535.
    #[test]
    fn origin_port_bounds_are_enforced() {
        for bad in [
            "http://localhost:0",
            "http://localhost:65536",
            "http://localhost:99999",
            "http://localhost:",
            "http://localhost:00080",
            "http://localhost:-1",
        ] {
            assert_invalid(&with("BACKUP_ALLOWED_ORIGINS", bad), bad);
        }
        for good in ["http://localhost:1", "https://localhost:65535"] {
            assert_enabled(&with("BACKUP_ALLOWED_ORIGINS", good), good);
        }
    }

    /// AC-003 S3, T-11 (behaviour 8): host labels may not start or end with '-'.
    #[test]
    fn origin_host_label_hyphen_edges_refused() {
        for bad in [
            "http://-a.example",
            "http://a-.example",
            "http://a.-b.example",
            "http://a.b-.example",
        ] {
            assert_invalid(&with("BACKUP_ALLOWED_ORIGINS", bad), bad);
        }
        assert_enabled(
            &with("BACKUP_ALLOWED_ORIGINS", "http://a-b.ex-ample:8080"),
            "inner hyphen control",
        );
    }
}
