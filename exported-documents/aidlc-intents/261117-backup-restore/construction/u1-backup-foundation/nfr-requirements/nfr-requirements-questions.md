# Câu hỏi NFR Requirements — u1-backup-foundation

## Q1. [Security] Bảo vệ dữ liệu staging chưa hoàn chỉnh

U1 có thể tạo artifact database-only `incomplete`, nhưng đây vẫn có thể chứa dữ liệu nhạy cảm. Cơ chế nào được phép trước khi ghi bytes xuống staging?

A. Chỉ private server-side staging với access tối thiểu, encryption-at-rest theo platform, giới hạn bytes/expiry bắt buộc; nếu cleanup bền vững hoặc policy chưa cấu hình thì không ghi dữ liệu thật
B. Ghi vào thư mục tạm mặc định và xóa khi process bình thường kết thúc
C. Staging có thể tải về nếu admin biết job ID
X. Other (please specify)

[Answer]: Private server-side staging, encryption-at-rest per platform, finite limits/expiry; refuse if cleanup/policy absent

## Q2. [Security] Quyền truy cập API và tài nguyên

Mọi thao tác backup/job phải được phân quyền phía server, còn auth/session cụ thể chưa được xác minh hoàn toàn trong codebase. Mục tiêu nào phù hợp cho U1?

A. Mọi endpoint kiểm tra authenticated administrator; mọi job read kiểm tra resource authorization; resource không thuộc quyền trả safe denial; không ghi credentials/token/raw error
B. Chỉ ẩn nút UI; API tin cậy browser
C. Chấp nhận bất kỳ user token nào, miễn caller biết job ID
X. Other (please specify)

[Answer]: Server-side admin auth and per-resource authorization; safe denial and redaction

## Q3. [Reliability] Job crash, timeout và recovery semantics

Worker dự kiến local Tokio task nhưng lease/restart semantics chưa được xác定. NFR nào nên áp dụng cho U1 ngay lập tức?

A. Durable job transitions, finite configured timeout, stale/unknown result không bao giờ success; fail closed khi recovery/lease policy chưa defined; không nhận claim automatic restart-resume
B. Timeout cố ý không giới hạn để tránh job fail
C. Process restart có thể đánh dấu mọi job running là succeeded nếu staging file tồn tại
X. Other (please specify)

[Answer]: Durable transitions, finite timeout and non-success for unknown outcomes; recovery policy gates claims

## Q4. [Performance/limits] Giới hạn U1 localhost

Chưa có số liệu kích thước dữ liệu hoặc throughput, và group chưa chọn ngưỡng. Cách đặt yêu cầu nào an toàn và kiểm chứng được mà không bịa con số?

A. Các byte/time/temp/concurrency limits phải là cấu hình hữu hạn được validate trước khi staging; ghi giá trị theo deployment config và benchmark sau khi xác định dataset; absent/invalid thì refuse
B. Không giới hạn trong localhost; chỉ giới hạn khi lên production
C. Chọn tùy ý một số cố định làm cam kết cho mọi instance
X. Other (please specify)

[Answer]: Finite validated configuration; absent/invalid values refuse; benchmark after dataset known

## Q5. [Reliability] Timeout và lỗi xuyên boundary

Backup adapter/DB có thể timeout sau khi thao tác đã bắt đầu; consumer cần trạng thái trung thực. Hành vi nào phù hợp?

A. Deadline per operation/job từ config hữu hạn; retry chỉ transient và idempotent; timeout/unknown result giữ non-success, ghi reason code/correlation ID an toàn và yêu cầu reconciliation thay vì báo không có side effect
B. Retry vô hạn tới khi hoàn tất
C. Trả success nếu không nhận được lỗi trước khi HTTP response kết thúc
X. Other (please specify)

[Answer]: Bounded deadline; retry transient/idempotent only; unknown remains non-success and requires reconciliation

## Q6. [Observability] Log, audit và metric

Yêu cầu audit cần actor/source/job/digest/outcome nhưng cấm credentials, raw rows, object contents và dữ liệu nhạy cảm. Bộ observability nào phù hợp với U1?

A. Structured events cho state transition, duration, bytes, component result code và correlation/job ID; redact secrets và không dùng dữ liệu nhạy cảm làm label; audit failure trước side effect thì từ chối
B. Log toàn bộ adapter request/response để debug dễ hơn
C. Chỉ ghi log khi job thành công
X. Other (please specify)

[Answer]: Safe structured events; redact sensitive data; mandatory pre-side-effect audit failure refuses
