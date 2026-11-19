# Yêu cầu khả năng mở rộng — u1-backup-foundation

## Phạm vi

U1 chỉ hỗ trợ localhost và một workflow backup skeleton. Chưa có growth projection, dataset size hoặc concurrency target được xác nhận; không tuyên bố horizontal scaling hay production capacity.

| ID | Yêu cầu | Tiêu chí xác minh |
|---|---|---|
| NFR7.1 | Số job worker đồng thời, staging bytes, entry/size nếu có archive, và DB resource use phải bị giới hạn bởi finite server configuration. | Config validation và tests xác nhận vượt ngưỡng bị từ chối an toàn, không tạo resource không giới hạn. |
| NFR7.2 | Job persistence và idempotency phải giữ semantics chính xác dưới duplicate requests và worker concurrency; chỉ một transition hợp lệ được commit cho cùng expected state/version. | Concurrent integration tests cho same key/same digest, same key/different digest, duplicate worker claim. |
| NFR7.3 | U1 không yêu cầu multi-process/multi-host coordination; local Tokio execution không được mô tả như scale-out hoặc crash recovery guarantee. | Architecture review xác nhận scope; khi cần nhiều API instances phải thiết kế shared lease/lock và staging coordination riêng. |
| NFR7.4 | Mọi giới hạn/throughput được benchmark trên dataset/fixture đại diện trước khi mở rộng; không hard-code throughput claim. | Benchmark report kèm hardware/config/data version trước khi đặt target tiếp theo. |

## Trigger để thiết kế lại

Nếu phải xử lý nhiều worker process/host, dataset vượt configured staging budget, hoặc nhiều instance đồng thời, phải bổ sung thiết kế concurrency/lease, object storage streaming, backpressure và recovery trước khi đổi cấu hình để tăng capacity.
