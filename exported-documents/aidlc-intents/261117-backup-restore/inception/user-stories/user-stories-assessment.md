# Đánh giá nhu cầu User Stories

## Quyết định

Execute.

## Lý do

Backup/restore là feature hướng người dùng, có nhiều persona liên quan, thao tác phá hủy cần xác nhận, nhiều trạng thái job và tích hợp database/Auth/Storage. User Stories giúp mô tả hành trình theo góc nhìn admin, giữ thứ tự lát cắt đầu-cuối mà nhóm yêu cầu, và tạo tiêu chí kiểm thử/traceability từ requirement tới acceptance criteria.

## Yếu tố đã xem xét

- Dự án brownfield; gateway hiện có CLI skeleton nhưng chưa có UI admin, ZIP pipeline hay upload/restore hoàn chỉnh.
- Admin thực hiện lựa chọn instance, tạo/tải backup và upload/xác nhận restore.
- Operator theo dõi job và xử lý recovery; đây là persona hỗ trợ.
- Restore target thay thế dữ liệu trong phạm vi được hỗ trợ và phải có preflight/confirmation.
- Scope đợt đầu self-hosted, localhost-only; production restore ngoài phạm vi.

## Khu vực giá trị stories

- Hành trình tạo và lấy package backup đã xác minh.
- Hành trình upload, kiểm tra preflight và xác nhận restore an toàn.
- Theo dõi job, xử lý lỗi, integrity và trạng thái recovery.
- Bảo đảm yêu cầu có nguồn truy vết tới FR/NFR và acceptance criteria có thể kiểm thử.

## Quyết định lập kế hoạch

- Persona chính: quản trị viên My-CMS.
- Persona hỗ trợ: người vận hành.
- Chia stories theo hành trình người dùng, ưu tiên lát cắt end-to-end thay vì tách theo lớp kỹ thuật.
