# Thiết kế hiệu năng — u1-backup-foundation

## Design constraints

U1 chỉ chứng minh localhost integration path; không có numeric throughput, latency SLO hay dataset profile. Tránh cache vì job state cần authoritative và chưa có evidence cần read-heavy optimization. Ưu tiên memory/resource bounds, async request separation và measurable baseline hơn tối ưu giả định.

## Request and worker separation

- API validate auth/config/idempotency, commit `BackupJob`, rồi trả `202` sau transaction commit; không giữ HTTP request trong thời gian export/archive.
- Worker xử lý async trong runtime gateway localhost, dùng bounded concurrency semaphore từ config; Tokio task không tự mang durabilty guarantee.
- DB export stream/chunk nếu adapter/library hỗ trợ để tránh buffer toàn bộ dump trong memory; archive/checksum pipeline dùng bounded buffers. Nếu implementation library buộc buffering và vượt memory budget, reject before staging.
- UI lấy status bằng explicit refresh/current API behavior; không giả progress percentage, không poll vượt configured request budget. Poll cadence/UX phải đối chiếu với contract và existing app.

## Finite budgets

Typed configuration bắt buộc gồm max input bytes, max staging bytes, max duration per operation/job, max concurrent jobs, max archive entries/depth/expanded bytes và cleanup expiry. Validate trước bytes được ghi. Giá trị cụ thể chưa được benchmark/approve: thiếu/sai cấu hình thì disable data path; không lấy ví dụ trong pattern guide làm mặc định.

## Measurement plan

Khi fixture/database scope được xác nhận, đo API job-accept duration, export/checksum duration, peak RSS, bytes staged, concurrency queue delay và cleanup time. Ghi OS/runtime/config/fixture version. Đặt numeric budgets sau khi người phụ trách phê duyệt; không biến baseline một máy localhost thành SLO production.
