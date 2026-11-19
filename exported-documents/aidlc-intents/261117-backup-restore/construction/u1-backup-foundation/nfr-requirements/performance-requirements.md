# Yêu cầu hiệu năng — u1-backup-foundation

## Phạm vi và nguyên tắc

Không có dataset size, target throughput hoặc latency SLO đã được phê duyệt. Không tạo cam kết số tùy ý. U1 chạy localhost và chứng minh correctness/integration, không tối ưu throughput. Mọi thời hạn/giới hạn tài nguyên phải hữu hạn, cấu hình được và được validate trước staging.

| ID | Yêu cầu | Tiêu chí xác minh |
|---|---|---|
| NFR6.1 | API create-job chỉ thực hiện authentication, allowlist/config validation, idempotency và durable job commit; capture/archive chạy ngoài HTTP request. Thời hạn request lấy từ cấu hình hữu hạn. | Test xác nhận API trả sau durable commit và không chờ export/archive; cấu hình deadline thiếu/sai bị reject. Numeric latency SLO chờ benchmark/owner decision. |
| NFR6.2 | Worker tuân thủ configured finite whole-job, per-operation, temp-byte, archive-byte, concurrency và staging-expiry limits. Không dùng “unbounded localhost” làm fallback. | Boundary tests tại dưới/bằng/trên limit; invalid/absent required limit không ghi dữ liệu. |
| NFR6.3 | Progress chỉ phát khi có số liệu đo thực; không giả lập phần trăm để cải thiện cảm nhận hiệu năng. | UI/API tests: absent progress stays null/unknown; completed/total nhất quán và bounded. |
| NFR6.4 | Tạo backup U1 chỉ đọc/copy phần cần thiết cho database slice; không giữ toàn bộ raw rows trong process hoặc job metadata. | Memory/resource test trên fixture được chọn khi có dataset; không ghi raw contents vào metadata/log. |

## Đo lường và chốt target

Sau khi database scope và fixture size được xác nhận, benchmark local API acceptance latency, database export duration, archive/checksum duration, peak resident memory và staging bytes. Chủ sở hữu dự án phê duyệt numeric targets trước khi đưa vào NFR Design; cho đến khi đó các số đo là baseline quan sát, không phải SLO.
