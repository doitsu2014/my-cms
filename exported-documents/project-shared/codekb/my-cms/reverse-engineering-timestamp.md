# Reverse Engineering Timestamp

## Run Record

- Date: 2026-11-17
- Commit: 22c668809535f0cbf5c297564e25524f721242e5

## Scope of Analysis

```yaml
scope_version: 1
kind: partial
intent: 261117-backup-restore
fingerprint: 586ab546e490c2ed668324497335a67d5a9e48b3
analyzed:
  paths:
    - apps/api/Cargo.toml
    - apps/api/gateway/Cargo.toml
    - apps/api/gateway/src/main.rs
    - apps/api/gateway/src/backup_cli.rs
    - apps/api/gateway/src/migrate_cli.rs
    - apps/api/domain_interface/Cargo.toml
    - apps/api/domain_posts/Cargo.toml
    - apps/api/domain_auth/Cargo.toml
    - apps/api/domain_media/Cargo.toml
    - apps/api/domain_user/Cargo.toml
    - apps/api/domain_seo/Cargo.toml
    - apps/api/test_helpers/Cargo.toml
    - apps/api/domain_posts/src/
    - apps/api/domain_auth/src/
    - apps/api/domain_media/src/
    - apps/api/domain_user/src/
    - apps/api/domain_seo/src/
    - apps/api/domain_interface/src/
    - apps/api/test_helpers/src/
    - apps/web/package.json
    - apps/ducth-dev-website/package.json
    - packages/editor-prose/package.json
    - docs/features/backup-restore.md
    - README.md
    - docs/README.md
    - docs/architecture/
    - docs/features/
    - .github/workflows/
    - .github/action-rs/
components:
  - gateway
  - domain_interface
  - domain_posts
  - domain_auth
  - domain_media
  - domain_user
  - domain_seo
  - test_helpers
  - admin_side
  - apps/ducth-dev-website
  - packages/editor-prose
  - deployments/docker-swarm
  - deployments/k8s
shallow:
  paths:
    - apps/web/src/
    - apps/ducth-dev-website/src/
    - packages/editor-prose/
    - deployments/docker-swarm/
    - deployments/k8s/
    - scripts/
    - .vscode/
    - remaining documentation, root configuration, and static assets
```
