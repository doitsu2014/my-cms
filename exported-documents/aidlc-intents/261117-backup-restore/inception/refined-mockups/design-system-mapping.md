# Design System Mapping — Backup & Restore

## Evidence boundary

CodeKB mô tả ứng dụng admin React/TypeScript nhưng không khảo sát sâu toàn bộ UI/component library. Các mapping dưới đây là semantic design requirements; component names cụ thể cần được đối chiếu với component library và theme hiện có trong `apps/web`.

## Semantic component mapping

| UX element | Existing admin pattern to reuse | Required behavior |
|---|---|---|
| Page heading and tabs | Admin page header and tab/navigation pattern | Hai tab rõ ràng; focus/active state không chỉ dựa vào màu sắc |
| Instance selector | Existing searchable/select input | Nhãn môi trường và non-secret identity; loading/empty/error states |
| File upload | Existing file input/upload pattern | Accepted ZIP, configured max size, progress, retry/error, keyboard operation |
| Job history | Existing table/list/card pattern | Newest-first, status text + icon, expandable details; responsive cards on narrow screens |
| Status badge | Existing badge/alert semantics | Có text label, icon/shape, màu tương phản; không dùng màu đơn độc |
| Preflight summary | Existing alert/card and definition-list pattern | Compatibility, scopes, integrity, warning và replace impact hiển thị đồng nhất |
| Destructive confirmation | Existing dialog/page confirmation pattern | Target và data impact được nêu rõ; primary action visually distinct nhưng không chỉ nhờ màu |
| Progress | Existing progress/status component if present | Text alternative cho screen reader; không biểu diễn phần trăm giả |
| Error feedback | Existing inline validation/toast/alert | Lỗi theo field hoặc component, hướng xử lý; giữ dữ liệu nhập an toàn |

## Tokens and content rules

- Dùng token màu nền, text, border, spacing, typography và focus ring từ admin theme hiện tại; không tạo bảng token mới nếu hệ thống có sẵn.
- Status phải có text: Queued, Running, Succeeded, Failed, Recovery required (dịch qua localization hiện có).
- Cảnh báo nhất quán `best-effort` và thao tác `replace` phải có text rõ, không chỉ icon/màu.
- Không dùng public/editor brand token từ `editor-prose`; feature thuộc admin shell.
- Không đưa object contents, credential hay raw database row vào UI.
