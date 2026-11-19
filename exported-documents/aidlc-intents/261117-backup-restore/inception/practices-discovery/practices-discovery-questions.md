# Phỏng vấn thực hành của nhóm

## Q1. [Release engineer] Cách làm việc và hợp nhất

Lịch sử và cấu hình hiện có cho thấy `main` được dùng làm nhánh đích, nhưng chưa đủ để xác định cách nhóm làm việc. Nhóm thường tạo nhánh và hợp nhất thay đổi như thế nào? Với thay đổi backup/restore, có yêu cầu cụ thể về pull request, review hoặc squash merge không?

A. Làm trực tiếp trên `main`, kiểm tra thay đổi trước khi hợp nhất
B. Tạo nhánh ngắn hạn và hợp nhất qua pull request
C. Tạo nhánh phát hành dài hạn hoặc quy trình khác
X. Other (please specify)

[Answer]: B — Tạo nhánh mới để làm việc, sau đó dùng pull request cho toàn bộ thay đổi.

## Q2. [Architect] Lát cắt chạy xuyên suốt

Có nên xây trước một phiên bản tối thiểu chạy xuyên suốt để chứng minh các phần chính kết nối được trước khi bổ sung đầy đủ khả năng backup/restore không? Đây là cách kiểm chứng tích hợp sớm, không có nghĩa là bỏ qua yêu cầu an toàn trước khi restore production.

A. Có, bắt đầu bằng một lát cắt tối thiểu chạy đầu-cuối
B. Không, hoàn thiện từng lớp trước rồi tích hợp sau
C. Chưa quyết định; đề xuất cách tiếp cận khi lập thiết kế
X. Other (please specify)

[Answer]: A — Có, xây trước một lát cắt tối thiểu chạy đầu-cuối.

## Q3. [Quality engineer] Phương pháp và thứ tự kiểm thử

Đề xuất tạm thời là triển khai từng lớp rồi viết và chạy kiểm thử cho lớp đó (`test-after`). Nhóm có muốn dùng cách này cho backup/restore không? Mục tiêu bao phủ 80% và CI hiện được thấy trong cấu hình nhưng workflow CI ghi tạm vô hiệu hóa; nhóm muốn coi các mục nào là cổng bắt buộc trước khi hợp nhất?

A. Giữ `test-after`; yêu cầu mục tiêu 80% và CI xanh trước khi hợp nhất
B. Giữ `test-after`; dùng kiểm thử tập trung theo yêu cầu, chưa đặt ngưỡng bao phủ chung
C. Dùng phương pháp khác hoặc cổng chất lượng khác
X. Other (please specify)

[Answer]: A — Giữ `test-after`; yêu cầu mục tiêu bao phủ 80% và CI xanh trước khi hợp nhất.

## Q4. [Operations engineer] Phát hành và quay lui

Tài liệu có nhắc đến Docker Swarm, Kubernetes và quy trình phát hành ảnh, nhưng chưa xác định môi trường đang dùng. Riêng feature backup/restore, nhóm có yêu cầu kiểm thử/phát hành nào ngoài kiểm thử localhost không?

A. Chỉ cần kiểm thử localhost cho feature này; không đặt thêm bước phát hành riêng
B. Kiểm thử localhost, sau đó theo quy trình phát hành hiện hành
C. Chưa chốt; đề xuất quy trình trong thiết kế
X. Other (please specify)

[Answer]: A — Chỉ kiểm thử localhost, không thêm bước phát hành riêng; áp dụng đây là quy trình chung cho mọi thay đổi.

## Q5. [Developer] Quy ước mã và review

Đề xuất tạm thời là tuân theo formatter/linter và quy ước sẵn có của từng package, không thêm quy tắc riêng nếu chưa thống nhất. Nhóm có yêu cầu bổ sung nào về phong cách mã, review, bảo mật hoặc kiểm thử cho thay đổi này không?

A. Dùng formatter/linter sẵn có; không thêm quy tắc riêng
B. Dùng quy ước hiện có và bổ sung yêu cầu review/bảo mật cụ thể
C. Cần khảo sát thêm trước khi chốt quy ước
X. Other (please specify)

[Answer]: A — Dùng formatter/linter hiện có; không thêm quy tắc riêng.
