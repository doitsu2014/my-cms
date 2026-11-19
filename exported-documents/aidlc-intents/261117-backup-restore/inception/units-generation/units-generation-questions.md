# Câu hỏi phân rã Units — Backup & Restore

## Q1. [Architect] Ranh giới unit và story-first slice

Nhóm đã chọn một lát cắt tối thiểu chạy đầu-cuối trước và làm theo `test-after`. Component model hiện có API, orchestrator, worker/persistence, adapters và archive logic. Nên chia implementation unit thế nào?

A. Chia theo lát cắt dọc theo hành trình: foundation/metadata + một luồng localhost end-to-end, rồi mở rộng backup, preflight/restore và recovery
B. Chia theo từng adapter/lớp kỹ thuật trước (DB, Auth, Storage, ZIP, UI) rồi tích hợp cuối
C. Gộp toàn bộ backend feature thành một unit và admin UI thành unit riêng
X. Other (please specify)

[Answer]: A — Lát cắt dọc

## Q2. [Architect] Độ hạt và đơn vị deploy

Đợt đầu chỉ chạy và kiểm thử localhost, không có release riêng; topology deploy sẽ quyết định ở stage sau. Mức độ hạt nào phù hợp để units độc lập kiểm thử và tích hợp?

A. Coarse-grained units theo khả năng end-to-end; worker/API/adapter có thể nằm chung unit nếu cùng thay đổi
B. Fine-grained unit cho mỗi component trong catalogue để tối đa hóa parallelism
C. Chưa chốt deployment boundary; chia theo interface/component nhưng ghi rõ đây chưa phải deployable service
X. Other (please specify)

[Answer]: A — Coarse-grained

## Q3. [Developer] Runtime worker và persistence cho localhost

Domain Design đề xuất Tokio task trong gateway process và durable metadata store, nhưng việc restart/lease còn để Functional Design. Đợt đầu cần chọn giả định nào để phân rã unit?

A. Local Tokio task + bảng metadata trong PostgreSQL hiện hữu; khả năng restart/lease phải được thiết kế riêng trước implementation
B. Hàng đợi bên ngoài ngay từ đầu, bất kể chưa chạy production
C. In-memory job state vì chỉ localhost; không cần metadata bền vững
X. Other (please specify)

[Answer]: A — Tokio + PostgreSQL metadata

## Q4. [Architect] Thứ tự topology và song song

DAG chỉ mô tả phụ thuộc để build/test pass, không chọn trình tự giá trị kinh tế. Ranh giới phụ thuộc nào gần với thực tế nhất?

A. Foundation/API metadata trước; archive/export backup slice sau; restore orchestration sau adapter capabilities; history/audit phụ thuộc job foundation
B. Tất cả unit độc lập; chỉ tích hợp cuối
C. Một unit tuần tự duy nhất
X. Other (please specify)

[Answer]: A — Foundation rồi adapters rồi restore
