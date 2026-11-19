# Kế hoạch và câu hỏi User Stories — Sao lưu và khôi phục My-CMS

## Q1. [Product manager] Persona và người dùng chính

Tài liệu yêu cầu xác định quản trị viên thao tác trên admin UI và người vận hành theo dõi kết quả; người dùng CMS là đối tượng được bảo vệ dữ liệu nhưng không thao tác trực tiếp. Ai nên là persona chính của stories trong đợt đầu?

A. Quản trị viên My-CMS thực hiện backup/restore; người vận hành là persona hỗ trợ
B. Người vận hành là persona chính; quản trị viên chỉ khởi tạo thao tác
C. Cả quản trị viên và người vận hành là persona chính với stories riêng
X. Other (please specify)

[Answer]: A — Admin chính; chia theo hành trình.


## Q2. [Product manager] Cách chia nhỏ stories

Yêu cầu bao gồm tạo ZIP, tải package, upload/preflight, xác nhận target, thực hiện restore và xem kết quả. Để stories có thể xây theo lát cắt chạy xuyên suốt mà nhóm đã chọn, nên chia backlog theo cách nào?

A. Theo hành trình người dùng: tạo/tải backup, rồi upload/preflight/restore
B. Theo persona: toàn bộ tác vụ quản trị viên, sau đó toàn bộ tác vụ vận hành
C. Theo thành phần kỹ thuật: database/Auth/Storage/UI thành stories riêng
X. Other (please specify)

[Answer]: A — Admin chính; chia theo hành trình.

