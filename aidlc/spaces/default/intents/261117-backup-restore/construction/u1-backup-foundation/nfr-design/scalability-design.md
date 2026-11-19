# Thiết kế khả năng mở rộng — u1-backup-foundation

## Initial topology

U1 chỉ chạy một gateway/local worker trên localhost, serial theo DAG Unit và bounded concurrency theo config. Không thiết kế scale-out/multi-host trong scope đầu; không thêm broker/cache/partitioning khi chưa có tải đo được.

## Resource isolation

- Giới hạn số job active và queue depth bằng typed finite config; admission control trả lỗi safe khi capacity đầy.
- Mỗi job có byte/time/temp budgets; adapter/archiver không được chiếm buffer không giới hạn.
- Tách execution slots theo job operation nếu cần để tránh một archive task chiếm toàn bộ worker; con số cần benchmark và approval.
- PostgreSQL pool dùng gateway's existing pool, không tạo pool/DB độc lập; query/transaction scope ngắn cho state transitions, không giữ transaction mở qua export I/O.

## Concurrency correctness

- Idempotency digest scoped actor + operation + request digest.
- Optimistic state version cùng atomic claim ngăn hai task cùng xử lý một job; target lock/concurrency giữa restore jobs thuộc Unit sau.
- U1 restart/reconcile state chưa được coi là safe auto-resume; unknown job remains non-success until reconciled.

## Growth trigger

Nếu cần nhiều worker process/host, concurrent targets, large dataset streaming, hoặc load vượt bounds: trước hết hoàn thiện durable leases, shared locks, object-store staging and orphan cleanup; benchmark và thiết kế scaling mới. Không tăng limits tùy tiện.
