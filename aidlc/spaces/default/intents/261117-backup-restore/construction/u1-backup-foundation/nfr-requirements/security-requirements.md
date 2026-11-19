# Yêu cầu bảo mật — u1-backup-foundation

## Phạm vi

Các yêu cầu áp dụng cho lát localhost U1; chúng không chứng minh hiện trạng implementation hoặc xác nhận capability platform chưa kiểm tra. Thiết kế bị ràng buộc bởi [`requirements.md`](../../../inception/requirements-analysis/requirements.md), [`functional-spec.md`](../../u1-backup-foundation/functional-design/functional-spec.md), [`rules.md`](../../u1-backup-foundation/functional-design/rules.md), và [`contract-summary.md`](../../../inception/contract-design/contract-summary.md).

| ID | Yêu cầu | Tiêu chí xác minh |
|---|---|---|
| NFR1.1 | Mọi endpoint U1 yêu cầu actor xác thực và role administrator được kiểm tra server-side trước khi truy cập configuration/job/artifact metadata. | Integration tests cho unauthenticated và non-admin: 401/403; không trả instance list, job projection hoặc artifact reference. |
| NFR1.2 | Mỗi lần đọc job phải kiểm tra quyền trên resource, không chỉ xác thực ID. | Test actor khác/resource ID đoán được: safe denial và không tiết lộ tồn tại/chi tiết ngoài policy đã chọn. |
| NFR1.3 | API chỉ nhận configured instance ID từ server allowlist; reject/ignore client fields chứa endpoint, connection string, credential hoặc service-role key. | Request schema forbids unknown fields; tests chứng minh client-controlled endpoint không được đưa vào adapter/config. |
| NFR1.4 | Instance credentials và service-role secrets chỉ được resolve trong server-side secret/config boundary; không xuất hiện trong response, browser bundle, error, audit, log hoặc metric. | Secret canary scan trên API/UI/log/audit test outputs; không xuất hiện canary. |
| NFR1.5 | Backup staging chỉ được ghi vào private server-side storage với least-privilege access và platform encryption-at-rest; không có public URL/download route trong U1. | Access test từ non-owner/non-admin bị từ chối; storage policy inspection xác nhận private/encrypted; HTTP contract không trả `private_storage_ref`. |
| NFR1.6 | Staging write chỉ bắt đầu khi byte/time/temp limits, expiry, cleanup policy và storage protection đều có giá trị hữu hạn, hợp lệ. Nếu thiếu/sai thì từ chối trước khi byte đầu tiên được ghi. | Config validation tests cho absent/zero/negative/unbounded/invalid values; assert storage write count = 0. |
| NFR1.7 | Staging crash/cleanup không để dữ liệu nhạy cảm tồn tại quá hạn: mọi artifact có thể được phát hiện lại sau restart và dọn bằng cleanup/reconciliation bền vững; cleanup failure được báo và retry có bounded policy. | Failure-injection tests sau khi tạo object và trước khi commit metadata; restart/reconcile xóa orphan; expiry path xóa artifact; lỗi xóa không bị silent. Cho đến khi kiểm chứng, real-data staging bị disable. |
| NFR1.8 | Idempotency keys được lưu dưới dạng digest, scoped theo actor+operation; raw key không lưu/log. | Storage inspection và log scan xác nhận raw key absent; same request trả job hiện hữu, changed digest trả conflict. |
| NFR1.9 | Job state transitions dùng current status/state version guard; stale/unknown/timeout outcome không thể tạo `succeeded`. U1 DB-only flow chỉ terminal `incomplete` hoặc `failed`. | Concurrency/race tests và state-machine tests xác nhận không có illegal transition hoặc false success. |
| NFR1.10 | Adapter errors được ánh xạ thành stable safe reason/error code; raw upstream response, SQL rows, auth data, file paths chứa secrets và object bytes không đi vào caller projection. | Secret/raw-data canary assertions trên error, response, audit, logs và metrics. |
| NFR1.11 | Audit trước side effect nhạy cảm phải được ghi hoặc thao tác bị từ chối; audit chỉ chứa actor/source/job/time/digest/outcome/reason code, không chứa secrets/raw contents. | Inject audit store failure; assert action không bắt đầu. Kiểm tra schema/fixture logs không có các trường cấm. |
| NFR1.12 | Transport admin↔API, API↔database và API↔staging/object store phải dùng TLS/secure local configuration phù hợp; production assumptions không được áp vào localhost không mã hóa mà không ghi rõ boundary. | Configuration review và tests xác minh endpoint policy; local exception nếu cần chỉ giới hạn local loopback và không được mang sang remote deployment. |

## Threat considerations

- **Unauthorized job/resource access:** role check + resource authorization tại mọi read/write.
- **Credential injection từ browser:** input allowlist chỉ nhận opaque configured ID; không deserialize hoặc log credentials/endpoints.
- **Staging disclosure/orphaned sensitive data:** private storage, finite TTL/limits, durable orphan cleanup; real-data path disabled đến khi cleanup crash cases được chứng minh.
- **False backup assurance:** trạng thái U1 không có succeeded; staging artifact incomplete không downloadable.
- **Audit/log leakage:** structured safe reason codes, redaction và secret-canary tests.
- **Duplicate work/race:** idempotency digest, expected-state version, target/operation constraints theo contract.

## Out of scope / unresolved

- Không đặt tên thuật toán mã hóa cụ thể hoặc key-management service khi provider chưa chọn; yêu cầu chỉ định platform-provided encryption-at-rest và kiểm chứng configuration trước real data.
- TLS termination, secret-store mechanism, CSRF/session details và exact role claim syntax phải phù hợp với gateway hiện tại, cần source validation.
- Auth/Storage backup capabilities thuộc U2 trở đi; U1 không serialize Auth/Storage data.
- Cleanup service/trigger cụ thể, lease/restart, cancellation và reconciliation design phải được chốt ở NFR Design/Functional Design tiếp theo. Cho tới khi NFR1.7 pass, chỉ synthetic fixtures được phép vào staging.
