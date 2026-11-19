# Câu hỏi Domain Design — Backup & Restore

## Q1. [Architect] Ranh giới thành phần backup/restore

Feature cần phối hợp thao tác admin, điều phối job, database My-CMS, GoTrue/Auth, Storage và ZIP package. Để tránh phân tán quy tắc nghiệp vụ, nên đặt logic điều phối backup/restore ở đâu?

A. Tạo một thành phần Backup & Restore trong gateway/domain layer, điều phối adapter riêng cho database, Auth, Storage và package
B. Thêm logic backup vào từng domain hiện hữu (posts, user, media) để mỗi domain tự xuất dữ liệu
C. Đưa toàn bộ xử lý vào admin UI và gọi trực tiếp từng dịch vụ Supabase
X. Other (please specify)

[Answer]: A — Component Backup & Restore

## Q2. [Architect] Quyền sở hữu job và package metadata

Job, trạng thái, manifest metadata, checksum và audit liên quan nhiều dịch vụ nhưng có vòng đời riêng. Thành phần nào nên sở hữu các bản ghi điều phối này?

A. Thành phần Backup & Restore sở hữu job, package metadata, status và audit reference; payload lớn nằm trong private object store
B. `domain_posts` sở hữu job và package metadata vì dữ liệu chính của CMS nằm ở đó
C. Không lưu job/metadata bền vững; chỉ giữ trạng thái trong admin UI
X. Other (please specify)

[Answer]: A — Backup component sở hữu metadata backup

## Q3. [Architect] Ranh giới sở hữu dữ liệu nguồn

My-CMS data, GoTrue users và Storage objects có chủ sở hữu hiện hữu riêng. Khi backup/restore, cách nào giữ ownership rõ nhất?

A. Thành phần Backup & Restore điều phối các exporter/importer do từng domain/platform adapter cung cấp; ownership/schema gốc vẫn thuộc domain/platform
B. Thành phần Backup & Restore trở thành chủ sở hữu mọi bảng Auth, Storage và CMS entities
C. Gateway truy cập trực tiếp tất cả bảng/schema và không cần adapter/domain boundary
X. Other (please specify)

[Answer]: A — Backup component sở hữu job/package metadata; domain/platform giữ ownership dữ liệu nguồn.

## Q4. [Architect] Cách tích hợp giữa UI, API và worker

Backup/restore là job dài hạn; admin UI cần tiến độ và trạng thái nhưng không được giữ credential nguồn. Ranh giới tương tác nào phù hợp?

A. Admin UI gọi API được bảo vệ; API tạo job; worker/service phía server chạy adapter và UI chỉ đọc status/download qua API được phân quyền
B. Admin UI kết nối trực tiếp database/object store để thực hiện backup/restore
C. Một request HTTP đồng bộ giữ kết nối mở tới khi toàn bộ backup/restore hoàn tất
X. Other (please specify)

[Answer]: A — API tạo job, worker xử lý

## Q5. [Architect] Ranh giới tính nguyên tử và phục hồi

Database, Auth và Storage không có giao dịch chung; yêu cầu chấp nhận best-effort backup và đòi hỏi recovery/rollback khi restore thất bại. Cách nào nên dùng trong component model?

A. Orchestrator quản lý saga/state machine theo thành phần, ghi checkpoint/audit và giao rollback/recovery policy cho adapter; không tuyên bố atomic transaction xuyên hệ thống
B. Giả định một database transaction bao trùm database, Auth và object store
C. Cho mỗi adapter chạy độc lập, không có component nào sở hữu trạng thái tổng thể
X. Other (please specify)

[Answer]: A — Saga/state machine

## Q3 follow-up. [Architect] Phạm vi ownership

Câu trả lời ban đầu chọn “Backup component sở hữu hết”, nhưng hợp đồng kiến trúc hiện có giao dữ liệu CMS/Auth/Storage cho domain/platform sở hữu chúng. Phạm vi chính xác là gì?

A. Backup component sở hữu job, manifest/package metadata; domain/platform giữ ownership dữ liệu nguồn
B. Chuyển toàn bộ CMS/Auth/Storage entities sang component backup
X. Other (please specify)

[Answer]: A — Chỉ metadata backup; domain/platform hiện tại vẫn sở hữu dữ liệu nguồn.
