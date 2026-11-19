# Mô hình thực thể — u1-backup-foundation

## Phạm vi và nguồn

Thiết kế này chỉ bao quát các thực thể cần cho Unit U1 `u1-backup-foundation`, theo [`unit-of-work.md`](../../../inception/units-generation/unit-of-work.md) và phân ranh trách nhiệm tại [`unit-of-work-story-map.md`](../../../inception/units-generation/unit-of-work-story-map.md). U1 có thể giữ metadata cho job và artifact staging database-only nhưng không tạo BackupPackage hoàn chỉnh và không nhận quyền tải xuống. Các acceptance criteria của US1.2 ngoài phần U1 được thiết kế như `Deferred` cho U2 hoặc Unit sau.

Các cấu trúc dưới đây là mô hình logic, không quyết định schema vật lý hay migration. Metadata ở database hiện hữu; source rows tiếp tục thuộc domain sở hữu. Không lưu credentials, raw database rows hoặc object bytes trong các metadata entities.

```yaml
entities:
  - name: BackupJob
    description: Durable record of a single authorized backup attempt and its truthful terminal outcome.
    attributes:
      - name: job_id
        type: UUID
        required: true
        unique: true
      - name: operation
        type: enum
        required: true
        allowed_values: [backup]
      - name: status
        type: enum
        required: true
        allowed_values: [queued, running, incomplete, failed]
      - name: actor_id
        type: string
        required: true
        constraints: authenticated administrator identity; never a credential or token
      - name: source_instance_id
        type: string
        required: true
        references: ConfiguredInstance.id
      - name: idempotency_key_digest
        type: string
        required: true
        unique: true
        constraints: opaque-key digest scoped to actor and operation; never persist raw key
      - name: request_digest
        type: string
        required: true
      - name: state_version
        type: integer
        required: true
        min: 1
      - name: created_at
        type: timestamp
        required: true
      - name: started_at
        type: timestamp
        required: false
      - name: completed_at
        type: timestamp
        required: false
      - name: consistency_mode
        type: enum
        required: true
        allowed_values: [best_effort]
      - name: terminal_reason_code
        type: enum
        required: false
        allowed_values: [required_component_unavailable, capability_unverified, source_read_failed, archive_validation_failed, cancelled, internal_error]
      - name: safe_message_key
        type: string
        required: false
        constraints: localization-safe key; no raw upstream error
      - name: artifact_ref
        type: string
        required: false
        constraints: private staging reference only; never a download authorization
      - name: artifact_digest
        type: string
        required: false
        constraints: SHA-256 of staging artifact bytes if materialized
      - name: audit_reference_id
        type: UUID
        required: false
        references: BackupAuditReference.audit_id
    entity_constraints:
      - Terminal states are immutable except for an explicitly modeled recovery/cleanup correction.
      - status incomplete or failed is never downloadable and never treated as a complete backup.
      - status succeeded is not a state available to U1's database-only slice.
      - Artifact references resolve only in private staging storage and are not exposed in the API DTO.
  - name: ConfiguredInstance
    description: Server-side allowlisted instance identity projected to administrators without connection details.
    attributes:
      - name: id
        type: string
        required: true
        unique: true
      - name: display_label
        type: string
        required: true
      - name: environment_label
        type: string
        required: true
      - name: enabled
        type: boolean
        required: true
        default: false
    entity_constraints:
      - Connection endpoints and credentials are held only by server-side configuration/secret facilities, not this UI-facing projection.
      - No client-supplied endpoint, connection string, service-role key, or credential can create or modify this entity.
  - name: BackupStagingArtifact
    description: Private temporary database-only artifact created for the U1 integration demonstration; it is explicitly incomplete.
    attributes:
      - name: artifact_id
        type: UUID
        required: true
        unique: true
      - name: job_id
        type: UUID
        required: true
        unique: true
        references: BackupJob.job_id
      - name: component_name
        type: enum
        required: true
        allowed_values: [cms_database]
      - name: completeness
        type: enum
        required: true
        allowed_values: [incomplete]
      - name: byte_count
        type: integer
        required: true
        min: 0
      - name: sha256
        type: string
        required: true
        constraints: digest of staged bytes, not proof of complete backup
      - name: created_at
        type: timestamp
        required: true
      - name: expires_at
        type: timestamp
        required: true
      - name: private_storage_ref
        type: string
        required: true
        constraints: internal-only reference, never emitted as a download URL
    entity_constraints:
      - This artifact is for localhost integration evidence only and cannot be assembled as a complete backup package.
      - A missing Auth or Storage component prevents any promotion to a complete package state.
      - Cleanup follows the configured staging expiry; absent/invalid cleanup configuration prevents staging creation.
  - name: BackupAuditReference
    description: Non-sensitive audit metadata for the U1 backup job lifecycle.
    attributes:
      - name: audit_id
        type: UUID
        required: true
        unique: true
      - name: job_id
        type: UUID
        required: true
        references: BackupJob.job_id
      - name: actor_id
        type: string
        required: true
      - name: source_instance_id
        type: string
        required: true
        references: ConfiguredInstance.id
      - name: occurred_at
        type: timestamp
        required: true
      - name: outcome
        type: enum
        required: true
        allowed_values: [requested, started, incomplete, failed]
      - name: digest
        type: string
        required: false
      - name: reason_code
        type: string
        required: false
    entity_constraints:
      - Never record credentials, bearer tokens, passwords, raw database rows, object contents, raw idempotency key, or unsanitized adapter errors.
relationships:
  - from: ConfiguredInstance
    to: BackupJob
    cardinality: one-to-many
    direction: ConfiguredInstance.id is referenced by BackupJob.source_instance_id.
  - from: BackupJob
    to: BackupStagingArtifact
    cardinality: one-to-zero-or-one
    direction: A U1 job may create one private incomplete artifact; a job may also fail before artifact creation.
  - from: BackupJob
    to: BackupAuditReference
    cardinality: one-to-many
    direction: Lifecycle events reference the owning job.
```

## Entity summary

U1 persists an authorized backup attempt and its non-sensitive audit references. An optional staging record represents a private database-only artifact, always marked incomplete, and is never a downloadable BackupPackage. Configured instances are server-owned identifiers and safe display labels; connection details remain outside the entity and browser-facing API.
