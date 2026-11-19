# Yêu cầu quan sát hệ thống — u1-backup-foundation

## Logging và audit

| ID | Yêu cầu | Tiêu chí xác minh |
|---|---|---|
| NFR9.1 | Structured events ghi job ID/correlation ID, actor ID, configured source ID, operation, from/to state, safe reason code, timestamp và digest/byte count nếu có. | Schema/event tests xác nhận field bắt buộc; failed/incomplete events có mặt. |
| NFR9.2 | Tuyệt đối không ghi credential, bearer/service-role token, password, connection string, raw DB row, object content, raw idempotency key hoặc upstream error body. | Secret/data canary scan trên logs, audit, metrics và API errors không thấy canary. |
| NFR9.3 | Audit ghi request/started/incomplete/failed outcome; nếu audit store bắt buộc không khả dụng trước side effect thì từ chối công việc. | Failure-injection tests xác nhận fail-closed. |
| NFR9.4 | Error projection dùng stable safe code/message key + correlation/job ID; raw adapter detail chỉ được giữ ở kênh bảo vệ nếu policy được phê duyệt, không hiển thị cho admin. | API contract tests và log access review. |

## Metrics

- Đếm số job theo operation và status; số lần component capability bị unavailable; job duration, component duration, bytes được staging, cleanup success/failure và configured-limit rejection.
- Không dùng actor ID, instance secret, object key, email, raw path hoặc data content làm metric label.
- Không báo progress percentage nếu không có counter completed/total đáng tin.
- Alerting/SLO thresholds chưa được đặt vì workload localhost và chưa có service SLO; trước production cần owner xác nhận SLI/SLO/RPO/RTO.

## Tracing

- Propagate correlation ID qua API, persistence, worker và adapter boundary.
- Span attributes chỉ dùng operation/component/status/code; không gắn payload, credential, object path hoặc raw SQL.
- Distributed tracing không bắt buộc cho localhost-only U1; có thể instrument local spans mà không gửi ra ngoài khi exporter chưa cấu hình.
