# Câu hỏi Infrastructure Design — u1-backup-foundation

## Q1. [Platform] Phạm vi deployment cho U1

Scope được duyệt chỉ là phát triển/kiểm thử localhost, không có bước release/deploy riêng; Infrastructure Design được chạy theo Unit nhưng không có AWS/cloud target mới. Artifact nào phù hợp?

A. Đặc tả localhost trong topology hiện có; không tạo AWS resources hay production deployment. Nếu stack app không cần thay đổi, ghi rõ no new infrastructure và giữ CI-pipeline ở trạng thái test/verification instructions.
B. Thiết kế AWS production stack mới cho gateway và object storage dù chưa được phê duyệt.
C. Bỏ qua mọi thiết kế vận hành, không ghi prerequisites.
X. Other (please specify)

[Answer]: A — Localhost only; no new cloud infrastructure

## Q2. [Platform] Staging, TLS và secret boundary

NFR Design yêu cầu TLS, encryption-at-rest, private staging, finite limits và crash-safe cleanup; exact provider chưa được chọn. Hạ tầng/runtime nào nên mô tả?

A. Phụ thuộc existing local Supabase/configured storage boundary; secrets ở server-only config, TLS cho API/DB/storage; staging private/encrypted và chỉ enable real-data khi bounds/expiry/reconciliation được cấu hình/verified. Chưa chọn AWS service.
B. Thêm S3/KMS/Secrets Manager làm mặc định, dù chưa có target AWS.
C. Dùng filesystem mặc định, không cần encryption/TLS vì localhost.
X. Other (please specify)

[Answer]: A — Existing local boundary with TLS/private/finite cleanup gates; no AWS service

## Q3. [Platform] Monitoring và cảnh báo

Gateway OTLP/runtime hiện chưa được xác minh, và chưa có SLO/RPO/RTO số. Mức observability nào phù hợp ở U1?

A. Local structured logs/metrics qua abstraction hiện có; external exporter opt-in sau verification; cảnh báo operational về cleanup failure/config-disabled; không bịa SLO/threshold.
B. Bật CloudWatch/X-Ray hoặc OTLP exporter mặc định.
C. Chỉ log lỗi và bỏ audit/cleanup status.
X. Other (please specify)

[Answer]: A — Local safe telemetry; exporter opt-in; no invented SLO

## Q4. [Platform] CI/CD và môi trường

Team chọn localhost-only, không có release task; API CI workflow có test/coverage nhưng ghi TEMPORARY DISABLED. U1 nên làm gì với pipeline/infrastructure artifacts?

A. Không tạo deployment pipeline mới; document pipeline currently disabled, yêu cầu xác minh/khôi phục test+coverage gate trước merge; không claim CI-green. Rollback/smoke cho production là out of scope của U1.
B. Re-enable workflow hoặc thêm deploy step ngay, không cần approval.
C. Bỏ CI/coverage vì chỉ localhost.
X. Other (please specify)

[Answer]: A — Không tạo deployment pipeline; yêu cầu xác minh/khôi phục CI coverage, không claim CI-green

## Q5. [Platform] Quyết định shared infrastructure hay không

U1 là Unit đầu tiên trong DAG tuần tự; Units sau có thể dùng package/object storage nhưng chưa có shared resource ownership chi tiết. Có nên provision shared service ngay?

A. Chưa provision shared infrastructure ở U1; ghi dependency/config gates và defer selection tới Unit/design khi object-store/retention requirements được xác định.
B. Tạo S3 bucket dùng chung cho mọi Unit ngay.
C. Tạo database/schema mới riêng cho BackupRestore ngay.
X. Other (please specify)

[Answer]: A — Defer provisioning until store policy and ownership are known
