//! Configured single-instance registry (FN-02, IF-01, TS-45).

use serde::Serialize;

use super::config::BackupConfig;

/// U1 deliberately supports only one server-configured local instance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BackupInstance {
    pub id: String,
    pub label: String,
    pub environment: &'static str,
    pub operations: [&'static str; 1],
    pub capabilities: Capabilities,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Capabilities {
    pub create: bool,
    pub upload: bool,
    pub preflight: bool,
    pub restore: bool,
    pub download: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceRegistry {
    configured: BackupInstance,
}

impl InstanceRegistry {
    pub fn from_config(config: &BackupConfig) -> Self {
        Self {
            configured: BackupInstance {
                id: config.instance_id().to_owned(),
                label: config.instance_label().to_owned(),
                environment: "local",
                operations: ["backup_db_slice"],
                capabilities: Capabilities {
                    create: true,
                    upload: false,
                    preflight: false,
                    restore: false,
                    download: false,
                },
            },
        }
    }

    pub fn list(&self) -> [&BackupInstance; 1] {
        [&self.configured]
    }

    pub fn get(&self, id: &str) -> Option<&BackupInstance> {
        (self.configured.id == id).then_some(&self.configured)
    }

    pub fn configured(&self) -> &BackupInstance {
        &self.configured
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> BackupConfig {
        let root = std::env::temp_dir().join(format!("backup-instance-{}", std::process::id()));
        let state = BackupConfig::load(|key| {
            Some(match key {
                "BACKUP_ENABLED" => "true".into(),
                "BACKUP_INSTANCE_ID" => "local-1".into(),
                "BACKUP_INSTANCE_LABEL" => "Development instance".into(),
                "BACKUP_STORAGE_ROOT" => root.to_string_lossy().into_owned(),
                "BACKUP_ALLOWED_ORIGINS" => "http://localhost:3002".into(),
                "BACKUP_REQUEST_TIMEOUT_SECS" => "30".into(),
                "BACKUP_MAX_HEADER_BYTES" => "16384".into(),
                _ => return None,
            })
        });
        match state {
            crate::domain::config::BackupConfigState::Enabled(config) => config,
            crate::domain::config::BackupConfigState::Disabled { reason } => {
                panic!("test config must be enabled, got {reason:?}")
            }
        }
    }

    #[test]
    fn ts45_registry_exposes_only_the_configured_safe_local_instance() {
        let registry = InstanceRegistry::from_config(&config());
        let instances = registry.list();
        assert_eq!(instances.len(), 1);
        let instance = instances[0];
        assert_eq!(instance.id, "local-1");
        assert_eq!(instance.label, "Development instance");
        assert_eq!(instance.environment, "local");
        assert_eq!(instance.operations, ["backup_db_slice"]);
        assert_eq!(
            instance.capabilities,
            Capabilities {
                create: true,
                upload: false,
                preflight: false,
                restore: false,
                download: false,
            }
        );
        assert!(registry.get("local-1").is_some());
        assert!(registry.get("another-instance").is_none());
    }
}
