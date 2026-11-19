# Yêu cầu độ tin cậy — u1-backup-foundation

## Phạm vi

U1 là localhost-only; không có availability SLO hoặc RPO/RTO số cụ thể được phê duyệt. Yêu cầu tập trung vào job state bền vững, không báo thành công sai, giới hạn tài nguyên, và fail-closed khi outcome chưa xác định.

| ID | Yêu cầu | Tiêu chí xác minh |
|---|---|---|
| NFR8.1 | Job được persist trước khi API trả accepted. API persistence failure không tạo response accepted hoặc job giả. | Transaction/integration tests inject database failure trước/sau commit; response/job state nhất quán. |
| NFR8.2 | Worker chỉ transition theo expected status/state version; idempotency chống duplicate submit; retry chỉ cho lỗi transient trên thao tác idempotent. | State-machine/property tests và concurrency tests xác nhận transition hợp lệ và bounded retries. |
| NFR8.3 | Timeout, crash hoặc kết quả adapter không rõ giữ job ở trạng thái non-success; không suy luận success từ file tồn tại/process exit. | Failure injection tại từng bước; mọi unknown outcome là failed/incomplete hoặc trạng thái reconciliation riêng đã định nghĩa, không succeeded. |
| NFR8.4 | Job backup U1 có thể kết thúc `incomplete` hoặc `failed`; không cung cấp succeeded cho database-only slice vì Auth/Storage bắt buộc chưa được capture. | Test missing required adapter/capability: no succeeded state, no download authorization, safe terminal reason. |
| NFR8.5 | Staging có finite expiry và durable cleanup/reconciliation có khả năng nhận diện orphan sau crash; cleanup retry có bounded policy và lỗi không được bỏ qua. Trước khi cơ chế này được thiết kế/tested, real-data staging bị disable. | Crash injection sau object write trước metadata commit, sau metadata trước terminal state, và khi delete lỗi; restart reconciliation xóa hoặc báo rõ orphan, không để silent expiry miss. |
| NFR8.6 | Audit failure trước thao tác nhạy cảm khiến thao tác bị từ chối; audit ghi outcome kể cả incomplete/failed mà không chứa nội dung nhạy cảm. | Inject audit write failure và kiểm tra operation không bắt đầu; kiểm tra log/audit redaction. |
| NFR8.7 | Recovery/lease/restart semantics chưa được chứng minh thì hệ thống không claim automatic job resume hoặc exactly-once processing. | API/docs/status không quảng cáo resume; design gate yêu cầu reconciliation policy trước khi bật automatic restart. |
| NFR8.8 | Numeric timeout/concurrency/RPO/RTO chỉ được đặt sau benchmark/owner approval; missing required operational values không được thay bằng unbounded defaults. | Configuration-validation tests từ chối absent/invalid values; target decision được ghi trong downstream design. |
