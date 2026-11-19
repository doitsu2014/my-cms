# Câu hỏi thiết kế hợp đồng — Backup & Restore

## Q1. [Architect] Ranh giới API được đặc tả

Các Units có bề mặt admin HTTP dùng bởi `apps/web` và các hợp đồng nội bộ giữa các Unit. Để không biến chi tiết triển khai nội bộ thành API công khai, phạm vi nào cần OpenAPI?

A. Đặc tả API HTTP admin được UI tiêu thụ; các boundary nội bộ dùng Rust interface/type contract trong cùng ứng dụng
B. Đặc tả mọi boundary nội bộ như API HTTP độc lập
C. Chỉ ghi hợp đồng prose, không cần spec máy đọc
X. Other (please specify)

[Answer]: A — OpenAPI admin; interface nội bộ A — OpenAPI admin; interface nội bộ

## Q2. [Architect] Cơ chế giao tiếp giữa API và worker

Job backup/restore có thể kéo dài và cần trạng thái bền vững; runtime đầu tiên là localhost với Tokio task trong gateway. Hợp đồng nào nên pin giữa API và worker?

A. API đồng bộ tạo job bền vững rồi worker claim/execute qua interface nội bộ; API trả job ID/status, UI đọc trạng thái qua API
B. Giữ HTTP request mở tới khi backup/restore hoàn tất
C. Thêm message broker bên ngoài ngay trong phiên bản localhost đầu tiên
X. Other (please specify)

[Answer]: A — Job bền vững + worker A — Job bền vững + worker

## Q3. [Architect] Ownership và versioning

Các Units phụ thuộc tuần tự, còn format package phải hỗ trợ preflight tương thích. Cách sở hữu/versioning nào phù hợp?

A. Unit cung cấp sở hữu spec boundary; package manifest có format version tách biệt API version; chỉ thêm field tương thích ngược, breaking change cần cập nhật version và kiểm tra consumer
B. Một spec trung tâm do mọi Unit cùng sở hữu; không cần version vì tất cả phát hành cùng nhau
C. Version từng API nhưng không version manifest/package
X. Other (please specify)

[Answer]: A — Provider sở hữu spec; manifest version riêng A — Provider sở hữu spec; manifest version riêng

## Q4. [Architect] Lỗi, timeout và retry

Restore là thao tác thay thế dữ liệu trên nhiều hệ thống, nên retry mù có thể lặp side effect; yêu cầu cũng đòi outcome trung thực và rollback/recovery state. Chính sách boundary nào nên áp dụng?

A. Lỗi trả mã/kiểu ổn định, thông điệp an toàn và correlation/job ID; timeout giới hạn theo cấu hình; retry chỉ cho thao tác transient có idempotency; restore lỗi chuyển rollback hoặc `recovery-required`, không tự retry side effect không an toàn
B. Retry mọi lỗi một số lần cố định rồi trả lỗi tổng quát
C. Để mỗi Unit tự chọn lỗi và retry, không cần hợp đồng chung
X. Other (please specify)

[Answer]: A — Retry có kiểm soát; lỗi an toàn, timeout cấu hình, rollback/recovery-required A — Retry có kiểm soát; lỗi an toàn, timeout cấu hình, rollback/recovery-required
