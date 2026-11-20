# Quy tắc nghiệp vụ — u1-backup-foundation

## Phạm vi

Các quy tắc dưới đây áp dụng riêng cho U1 và các acceptance criteria U1 chịu trách nhiệm theo [`unit-of-work-story-map.md`](../../../inception/units-generation/unit-of-work-story-map.md). Các capability đầy đủ của US1.2 (Auth/Storage, tải package hoàn chỉnh), lịch sử đầy đủ và restore được deferred tới các Unit sau. Quy tắc an toàn không được nới lỏng vì chỉ chạy localhost.

```yaml
rules:
  - id: BR1.1
    statement: Chỉ quản trị viên đã xác thực mới được truy cập thao tác U1.
    category: authorization
    applies_to: Configured instance list, backup job creation, job status, and staging metadata.
    trigger: Mỗi request tới API admin.
    logic: IF actor chưa xác thực hoặc không có administrator role THEN từ chối request trước khi đọc/tạo resource.
    violation_behaviour: Trả lỗi an toàn 401 hoặc 403; không tiết lộ instance/job/artifact ngoài quyền.
    source: FR1.2, NFR1, AC1.1.3
  - id: BR1.2
    statement: Client chỉ được chọn một ID từ danh sách instance do server cấu hình.
    category: validation
    applies_to: BackupJob.source_instance_id.
    trigger: Khi tạo backup job.
    logic: IF instance ID không tồn tại, disabled, hoặc request chứa endpoint/credential/client-controlled connection data THEN không tạo job.
    violation_behaviour: Từ chối an toàn với lỗi field/code; không ghi dữ liệu nhạy cảm vào log/audit.
    source: FR1, FR1.1, FR1.3, NFR1
  - id: BR1.3
    statement: Tạo job phải bền vững và idempotent theo actor, operation và request digest.
    category: constraint
    applies_to: BackupJob creation.
    trigger: Khi nhận yêu cầu bắt đầu backup.
    logic: IF cùng actor và operation dùng lại idempotency key với cùng request digest THEN trả về job hiện có; IF digest khác THEN từ chối xung đột; otherwise persist job before acknowledging acceptance.
    violation_behaviour: Trả conflict an toàn cho key reuse khác nội dung; lỗi persistence thì không báo job accepted.
    source: FR2, FR6.2, AC1.2.1
  - id: BR1.4
    statement: U1 chỉ tạo artifact staging database-only mang trạng thái incomplete.
    category: policy
    applies_to: BackupJob and BackupStagingArtifact.
    trigger: Worker bắt đầu lát tích hợp database-only.
    logic: IF required Auth or Storage capture is not implemented and verified in U1 THEN any staged database bytes remain private, completeness=incomplete, and the job terminal status is incomplete or failed; this U1 path cannot transition to succeeded.
    violation_behaviour: Không quảng bá artifact như backup hoàn chỉnh; xóa staging artifact theo policy nếu thiếu/invalid expiry hoặc khi terminal cleanup chạy.
    source: FR2.1, FR2.2, FR2.5, US1.2, AC1.2.2, AC1.2.4
  - id: BR1.5
    statement: Artifact incomplete không được tải xuống qua API như package backup.
    category: authorization
    applies_to: Download and package endpoints.
    trigger: Mọi request tải artifact/package liên quan tới job U1.
    logic: IF job status is incomplete or failed, or artifact completeness is incomplete THEN deny download regardless of caller knowledge of job/artifact ID.
    violation_behaviour: Trả lỗi not-available an toàn; không phát sinh URL tới private staging storage.
    source: FR3.1, FR1.3, AC1.2.4
  - id: BR1.6
    statement: Worker chỉ được chuyển trạng thái theo kỳ vọng hiện tại; không được suy diễn thành công khi kết quả không chắc chắn.
    category: constraint
    applies_to: BackupJob lifecycle.
    trigger: Khi claim job, ghi component result, timeout hoặc kết thúc worker.
    logic: IF state version/current status không khớp THEN reject transition; IF worker timeout/crash/unknown outcome THEN preserve non-success state and record a safe reason; IF database slice completes without required Auth/Storage THEN terminal state is incomplete or failed.
    violation_behaviour: Audit transition failure; không báo `succeeded`; lease/restart recovery implementation vẫn deferred.
    source: FR2.1, FR6.3, AC1.2.4, AC3.1.3
  - id: BR1.7
    statement: Staging được tạo chỉ khi có giới hạn và thời hạn dọn dẹp cấu hình hợp lệ.
    category: constraint
    applies_to: BackupStagingArtifact and archive work.
    trigger: Trước khi ghi bytes vào staging.
    logic: IF size, expiry, or storage policy required for the staging path is absent/invalid THEN refuse staging before writing; otherwise enforce the configured bounds and expire/delete the artifact at the configured terminal time.
    violation_behaviour: Từ chối an toàn, ghi reason code không chứa secret; không dùng mặc định không giới hạn.
    source: FR3.3, FR6.3, NFR2, NFR4
  - id: BR1.8
    statement: Audit chỉ giữ metadata cần truy vết, không giữ nội dung dữ liệu hoặc bí mật.
    category: policy
    applies_to: BackupAuditReference, logs, metrics, and safe error projections.
    trigger: Mỗi lần tạo/yêu cầu/chạy/kết thúc backup.
    logic: IF audit is written THEN include actor, configured source ID, job ID, timestamp, digest when available, and outcome; exclude credentials, tokens, passwords, raw rows, object contents, raw idempotency key and raw adapter errors.
    violation_behaviour: Nếu audit bắt buộc không ghi được trước khi thao tác nhạy cảm bắt đầu THEN từ chối thao tác; nếu chưa có chính sách an toàn thì không âm thầm bỏ audit.
    source: FR6.4, NFR1, NFR2, AC3.2.2, AC3.2.3
  - id: BR1.9
    statement: UI phản ánh rõ instance unavailable và job chưa hoàn tất.
    category: policy
    applies_to: U1 admin screens and status presentation.
    trigger: Khi danh sách instance rỗng, không khả dụng, hoặc job queued/running/incomplete/failed.
    logic: IF no configured instance is available THEN explain configuration is missing and disable job creation; IF job progress cannot be measured THEN do not invent a percentage; show incomplete/failed text distinctly from success.
    violation_behaviour: Không cho khởi tạo thao tác không hợp lệ; lỗi nêu bước tiếp theo ở mức an toàn.
    source: FR1.1, FR6.1, NFR7, AC1.1.2
```

## Tóm tắt quy tắc

| ID | Chủ đề | Kết quả bắt buộc |
|---|---|---|
| BR1.1–BR1.2 | Quyền và instance | Admin-only; chỉ ID server-configured, không nhận endpoint/secret từ browser |
| BR1.3 | Job/idempotency | Lưu bền vững trước accepted; duplicate cùng request trả job cũ, khác digest conflict |
| BR1.4–BR1.6 | Lát U1 chưa hoàn chỉnh | Chỉ incomplete/failed, không success; private staging không downloadable; stale/unknown outcome không thể thành success |
| BR1.7 | Resource bounds | Không staging nếu giới hạn/expiry bắt buộc chưa cấu hình |
| BR1.8 | Audit | Có thể truy vết mà không ghi secret/raw contents |
| BR1.9 | Admin UI | Empty/error/incomplete state rõ ràng; không giả tiến độ |
