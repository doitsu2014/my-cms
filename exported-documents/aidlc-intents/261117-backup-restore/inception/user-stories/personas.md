# Personas — Sao lưu và khôi phục My-CMS

## Primary persona: Quản trị viên My-CMS

- **Vai trò:** Admin được phân quyền thao tác với backup/restore trong admin UI.
- **Mục tiêu:** Sao lưu instance đã cấu hình thành package tải được; upload và preflight một package; khôi phục an toàn vào target tương thích.
- **Nhu cầu:** Nhận diện rõ môi trường nguồn/đích; hiểu phạm vi package và tác động replace; thấy trạng thái job và lỗi có thể hành động.
- **Điểm đau:** Backup không đầy đủ, restore sai môi trường, file hỏng, hoặc lỗi giữa chừng có thể gây mất dữ liệu và giảm khả năng phục hồi.
- **Bối cảnh:** Đợt đầu phát triển/kiểm thử localhost với Supabase self-hosted; production restore chưa được hỗ trợ.
- **Ưu tiên:** Persona chính.

## Supporting persona: Người vận hành

- **Vai trò:** Theo dõi job và xử lý sự cố/recovery khi backup hoặc restore thất bại.
- **Mục tiêu:** Biết job thành công/thất bại ở thành phần nào, target đang ở trạng thái nào, và có cần can thiệp/rollback không.
- **Nhu cầu:** Job ID, trạng thái, timestamps, lỗi an toàn, audit và recovery outcome.
- **Điểm đau:** Một phần restore thất bại nhưng hệ thống báo thành công, hoặc thiếu bằng chứng để xác định dữ liệu cần phục hồi.
- **Bối cảnh:** Hỗ trợ admin; không nhất thiết là người bắt đầu thao tác.
- **Ưu tiên:** Persona hỗ trợ.

## Affected users: Người dùng CMS

Người dùng CMS không trực tiếp thao tác feature. Dữ liệu tài khoản, nội dung và media của họ nằm trong phạm vi được bảo vệ; restore có thể ảnh hưởng tới dữ liệu/session tùy phương thức nền tảng, hiện còn là điểm cần xác minh.
