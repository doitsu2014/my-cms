# Câu hỏi UX — Backup & Restore

## Q1. [Designer] Cấu trúc màn hình admin

Admin cần tạo backup và upload/restore package từ cùng khu vực. Đợt đầu ưu tiên bố cục nào?

A. Một trang Backup & Restore có hai tab rõ ràng: Tạo backup và Khôi phục
B. Một wizard duy nhất đi tuần tự từ backup sang restore
C. Một trang tổng quan với hai panel riêng cho backup và restore
X. Other (please specify)

[Answer]:

## Q2. [Designer] Xác nhận hành động thay thế dữ liệu

Restore thay thế dữ liệu trong target. Cách xác nhận nào giúp admin nhận diện đúng môi trường và hiểu rõ tác động?

A. Trang preflight riêng, sau đó hộp xác nhận cuối cùng có tên target và nội dung sẽ bị thay thế
B. Một hộp thoại xác nhận ngay sau khi upload
C. Gõ lại tên target để xác nhận, không cần bước wizard riêng
X. Other (please specify)

[Answer]:

## Q3. [Designer] Trạng thái job

Backup và restore có thể chạy lâu; lỗi có thể xảy ra ở database, Auth hoặc Storage. Cách trình bày trạng thái nào phù hợp?

A. Bảng lịch sử job với trạng thái tổng thể và chi tiết theo thành phần khi mở từng job
B. Chỉ hiển thị tiến trình của job đang chạy; không cần lịch sử
C. Thông báo/toast khi xong, không cần trang chi tiết lỗi
X. Other (please specify)

[Answer]:

## Q4. [Designer] Responsive và accessibility

Màn hình admin có bảng trạng thái, upload file và thao tác destructive. Ngoài các tiêu chí cơ bản của NFR7, có breakpoint hoặc yêu cầu WCAG cụ thể nào cần áp dụng không?

A. Responsive desktop/tablet/mobile; mục tiêu WCAG 2.1 AA cho thao tác cốt lõi
B. Chỉ desktop vì thao tác quản trị chỉ dùng trên máy tính
C. Chưa đặt mục tiêu WCAG/breakpoint; thiết kế đề xuất giá trị phù hợp
X. Other (please specify)

[Answer]:

## Q5. [Designer] Ngôn ngữ và định dạng nhãn

Admin UI hiện có thể phục vụ người dùng Việt/Anh; feature cần hiển thị trạng thái và cảnh báo nhất quán. Cách làm nào phù hợp?

A. Dùng hệ thống dịch/ngôn ngữ hiện có trong admin, giữ thuật ngữ nhất quán
B. Đợt đầu chỉ hiển thị tiếng Anh
C. Đợt đầu chỉ hiển thị tiếng Việt
X. Other (please specify)

[Answer]:
