# Quyết định tech stack — u1-backup-foundation

## Decisions and constraints

| Area | Decision / constraint | Rationale and evidence status |
|---|---|---|
| API/runtime | Gắn U1 vào API gateway hiện có; API chỉ commit job bền vững rồi trả job ID/status. Worker localhost dự kiến là Tokio task trong gateway process. | Phù hợp component/domain design và `my-cms-api` gateway ownership. Tokio task chưa phải durable queue; lease, crash recovery và restart semantics cần NFR/Functional Design riêng trước khi claim resume. |
| Persistence | Dùng PostgreSQL hiện hữu cho U1 job/package/audit metadata thông qua persistence boundary; không tạo database riêng hoặc migration crate mới. | Project contract giữ sole API gateway, domain migrations và data ownership hiện hữu. Exact schema/migration owner phải theo gateway/domain source verification. |
| Authorization/config | Tích hợp với gateway's existing admin authorization và server-side configured instance IDs; không nhận URL/credentials từ UI. | Exact middleware/session/claim integration cần xác minh source; không phát minh auth subsystem mới. |
| Frontend | Dùng React admin hiện có trong `apps/web`, localization, component/theme/accessibility patterns hiện hữu. | Mockup/contract mô tả Admin UI; tên component cụ thể cần đối chiếu source trong Code Generation. |
| Archive | Package ZIP/versioned manifest/SHA-256 concept theo contract; implementation library chưa chọn ở Functional Design. | Chọn dependency sau khi kiểm tra crate hiện có, archive security review, limits và API support; không thêm thư viện khi chưa được phê duyệt. |
| Staging | Private server-side storage với encryption-at-rest của platform; U1 chỉ synthetic fixture cho đến khi bounds, expiry và crash-safe cleanup được implement/test. | Người dùng chấp nhận staging private/bounded; provider/key mechanism chưa được xác định. |
| Observability | Reuse logging/metrics/tracing stack hiện có; structured safe events, correlation ID; không bật external exporter mặc định. | Gateway OTLP behavior là unverified; không giả định exporter/config sẵn sàng. |
| Testing | Test-after theo layer; dùng cargo test/API, UI tests theo project scripts; đạt coverage 80% theo policy nhưng CI workflow hiện marked temporarily disabled. | `README.md` liệt kê commands; coverage runner config nằm trong disabled workflow. Verify/restore mechanism trước khi gọi CI gate hoạt động. |

## Explicit non-decisions

- Không chọn broker bên ngoài; U1 localhost task không được mô tả là durable queue.
- Không chọn database dump tool/schema scope trước khi xác minh existing gateway/domain migrations và supported My-CMS tables.
- Không chọn GoTrue export/import API; capability/password/session semantics cần platform verification.
- Không chọn object-store product, key-management system, retention values, size limits, timeout numbers hoặc numeric performance SLO.
- Không đưa implementation-specific code/SQL vào Functional Design; selection/detail belongs in NFR Design and Code Generation with evidence.
