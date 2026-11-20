## Review

**Verdict:** NOT-READY
**Reviewer:** aidlc-architecture-reviewer-agent
**Date:** 2026-11-19T05:50:12Z
**Iteration:** 1
**Verdict Dictated:** no

### Findings

| ID | Severity | Location | Finding | Required action | Status |
|---|---|---|---|---|---|
| R-01 | Major | aidlc/spaces/default/intents/261117-backup-restore/inception/domain-design/components.md > BackupWorker.dependents BackupAdminApi | BackupWorker khai báo BackupAdminApi là dependent (style async), nhưng BackupAdminApi.depends_on không có BackupWorker. Catalogue YAML không đối xứng; sơ đồ mermaid cũng không có cạnh này. Đây không phải lời gọi thành phần thật: API đọc trạng thái job đã persist, không gọi worker. | Xóa cạnh giả BackupWorker → BackupAdminApi, hoặc thêm depends_on tương ứng nếu API thực sự gọi worker. Giữ depends_on/dependents đối xứng và khớp sơ đồ. | New |
| R-02 | Major | aidlc/spaces/default/intents/261117-backup-restore/inception/domain-design/components.md > External Dependencies / BackupPersistence.behaviour | US1.3/FR3 yêu cầu lưu package đã xác minh trong object store dài hạn. BackupPersistence tuyên bố không lưu object bytes; không thành phần nào khai báo kho object cho package/staging (ngoài Supabase Storage nguồn). ArchiveIntegrity chỉ fixture cho đến khi vượt cổng staging. Người triển khai không biết bytes package sống ở đâu. | Gắn kho object (private staging và bản lưu dài hạn) vào external_dependencies của thành phần sở hữu, hoặc ghi rõ U1 không persist bytes và thành phần nào sẽ sở hữu kho khi cổng TLS/mã hóa/cleanup đạt. | New |
| R-03 | Major | aidlc/spaces/default/intents/261117-backup-restore/inception/domain-design/decisions.md > ADR-002 Decision | ADR-002 vẫn nói BackupRestore sở hữu RestoreJob, trong khi catalogue chỉ có BackupJob (kèm operation), BackupPackage, BackupAuditEvent. Restore (US2.1/US2.2) không có thực thể/thành phần riêng; sơ đồ cũng không có Domain → DatabaseBackupAdapter dù YAML có depends_on đó. | Thống nhất: hoặc thêm RestoreJob với owner, hoặc sửa ADR-002/rationale để restore dùng BackupJob.operation. Khớp mermaid với mọi cạnh depends_on (Domain → DatabaseBackupAdapter). | New |

### Validation Tool Results

| Tool | Result | Interpretation |
|---|---|---|
| aidlc engine sensor-traceability --stage domain-design | PASS: gaps/orphans/invalid_targets rỗng; mọi US1.1–US3.2 map tới tên thành phần trong catalogue | Truy vết tên thành phần hợp lệ; không bắt lỗi đối xứng YAML hay thiếu kho package. |
| Kiểm tra thủ công YAML depends_on/dependents | FAIL: BackupWorker.dependents ↔ BackupAdminApi.depends_on | Xác nhận finding Major đầu. Đồ thị gọi còn lại không tự-phụ thuộc và không có chu trình thật. |
| Đối chiếu hướng đã chấp nhận (crate domain_backup_restore in-process, không service riêng, không sở hữu dữ liệu nguồn, U1 không staging dữ liệu thật) | Khớp ADR-004, behaviour BackupRestoreDomain/BackupWorker, rationale | Hướng decompositon mới đứng; NOT-READY do catalogue/ADR chưa đóng, không phải do chọn crate. |

### Summary

Hướng `domain_backup_restore` trong cùng process gateway, metadata/migration riêng, dữ liệu nguồn giữ chủ cũ, và cổng U1 không staging là đúng và có ADR. Catalogue chưa đóng vì cạnh YAML không đối xứng, thiếu chỗ lưu bytes package, và ADR-002 vẫn nói RestoreJob trong khi YAML chỉ có BackupJob.
