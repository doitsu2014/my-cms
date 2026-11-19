# Câu hỏi NFR Design — u1-backup-foundation

## Q1. [Architect] Storage protection và cleanup

NFR yêu cầu staging private, TLS, encryption-at-rest, giới hạn hữu hạn và cleanup sau crash; exact provider/key/cleanup service chưa được chọn. Design nào cân bằng fail-closed và khả năng kiểm chứng?

A. U1 dùng private staging adapter; mọi ghi cần security/config validation (TLS cho remote, at-rest protection, finite bounds/expiry); durable metadata + orphan reconciliation/retry phải chạy được sau restart. Nếu cleanup không đảm bảo hoặc không được quan sát thì disable dữ liệu thật.
B. Ghi vào local temp directory, best-effort delete khi app shutdown; chấp nhận orphan sau crash vì localhost.
C. Bật real-data staging trước, chọn cleanup/encryption sau khi triển khai.
X. Other (please specify)

[Answer]: A — Private adapter; finite config; durable orphan cleanup; disable real data if unverified

## Q2. [Architect] TLS và trust boundary

Yêu cầu nguồn đòi TLS cho admin/API, database và object store; C1 API contract ghi HTTPS. Môi trường đầu tiên là localhost, nhưng không có ngoại lệ TLS được chấp thuận. Design nào nên theo?

A. Không tạo loopback exception trong yêu cầu: remote/reachable admin, DB, object-store hops phải TLS; nếu gateway hiện chỉ plain HTTP loopback thì giữ real-data path disabled đến khi TLS/trust boundary được phê duyệt và cấu hình, không gắn secrets vào traffic không mã hóa.
B. Mặc định plain HTTP loopback cho mọi kết nối localhost; coi localhost tương đương TLS.
C. Chỉ dùng HTTPS ngoài loopback; DB/object-store traffic không cần TLS.
X. Other (please specify)

[Answer]: A — Giữ yêu cầu TLS

## Q3. [Architect] Job recovery và staging state

Worker hiện được mô tả là Tokio task trong gateway; lease/restart/cancellation còn chưa định nghĩa. Cơ chế nào nên thiết kế cho U1 trước khi bật real data?

A. Durable state machine với expected-version transitions; trạng thái `running` sau restart thành unknown/non-success; reconciliation kiểm tra metadata và staging bytes, cleanup orphan theo policy, không resume destructive work khi chưa chứng minh idempotency. Real-data path chờ lease/recovery tests.
B. Khi restart, scan staging directory và tự đánh dấu job thành công nếu có file.
C. Xóa mọi file temp khi khởi động mà không đối chiếu metadata/audit.
X. Other (please specify)

[Answer]: A — Durable states; reconcile/cleanup orphans; real-data awaits recovery proof

## Q4. [Architect] Observability và privacy

NFR chọn structured events, safe codes, correlation IDs; chưa có OTLP runtime verification. Cách triển khai observability nào phù hợp?

A. Instrument local structured events/metrics qua abstraction hiện có; default không gửi external exporter; whitelist attributes, redact trước emit; audit failure trước side effect từ chối; không có SLO numbers cho localhost.
B. Bật OTLP exporter bằng default vì project có nhắc `ENABLED_OTLP_EXPORTER`.
C. Ghi toàn bộ adapter payload và raw error vào log local.
X. Other (please specify)

[Answer]: A — Safe local instrumentation; redact; no external exporter by default

## Q5. [Architect] Resource bounds và retry strategy

Không có số throughput/size targets đã được xác nhận. NFR yêu cầu finite limits, bounded timeouts và retry chỉ cho transient idempotent operations. Design nào cụ thể?

A. Typed config bắt buộc cho byte/time/temp/concurrency limits; validate trước side effect; per-operation/job deadlines; bounded exponential backoff+jitter chỉ cho transient, idempotent operations; không retry mutation/unknown outcome.
B. Dùng giá trị mẫu của pattern guide (5 failures, 30s, 3 retries) làm production default mà không benchmark.
C. Retry mọi lỗi cho localhost vì không có production users.
X. Other (please specify)

[Answer]: A — Typed finite config; bounded deadlines/retries only for transient idempotent work
