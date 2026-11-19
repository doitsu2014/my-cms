# Câu hỏi làm rõ yêu cầu — Sao lưu và khôi phục My-CMS

## Q1. [Product manager] Môi trường và phạm vi hỗ trợ

Đặc tả yêu cầu chọn instance nguồn/đích nhưng chưa chỉ rõ mô hình Supabase. Phạm vi database/Auth/Storage và cách khôi phục phụ thuộc vào loại hosting và phiên bản được hỗ trợ. Những loại môi trường nào cần hỗ trợ ở đợt đầu?

A. Chỉ Supabase self-hosted do nhóm quản lý
B. Supabase hosted do nhà cung cấp vận hành
C. Cả hai loại; preflight phải kiểm tra tương thích theo từng loại
D. Chỉ hỗ trợ một danh sách instance/phiên bản được xác nhận cụ thể
X. Other (please specify)

[Answer]: A — Chỉ Supabase self-hosted do nhóm quản lý.

## Q2. [Product manager] Phạm vi “toàn bộ dữ liệu” và Auth

Đặc tả yêu cầu My-CMS database, GoTrue users và Storage blobs, đồng thời ghi rằng schema/cấu hình Supabase được quản lý có thể không khôi phục được. Đợt đầu cần cam kết phạm vi nào? Việc giữ password hash để người dùng tiếp tục đăng nhập bằng mật khẩu cũ có phải bắt buộc không?

A. My-CMS schemas/tables, Auth users/identities và toàn bộ Storage objects; giữ password hash nếu được nền tảng hỗ trợ
B. Bao gồm cả schema hệ thống do Supabase quản lý; chỉ bật khi có phương thức được hỗ trợ và kiểm chứng
C. Chỉ dữ liệu My-CMS và Storage; Auth users được tạo/khôi phục riêng theo quy trình khác
D. Chưa chốt phạm vi; không cho restore production trước khi có xác nhận theo nền tảng
X. Other (please specify)

[Answer]: A — My-CMS, Auth và Storage

## Q3. [Operations engineer] Tính nhất quán và ghi dữ liệu trong lúc backup

Database, Auth và Storage không mặc định nằm trong một giao dịch chung. Khi backup đang chạy, hệ thống có thể tạm dừng ghi dữ liệu để thu được một ảnh nhất quán không?

A. Có thể tạm dừng ghi; backup chỉ thành công khi tất cả nguồn được chụp trong cửa sổ đó
B. Không dừng được; chấp nhận best-effort, nhưng phải ghi rõ thời điểm và giới hạn nhất quán
C. Chỉ cho phép phương thức snapshot nhất quán do nền tảng hỗ trợ; nếu không có thì từ chối backup
D. Chưa quyết định; yêu cầu đề xuất phương án và rủi ro trước khi chọn
X. Other (please specify)

[Answer]: B — Chấp nhận best-effort

## Q4. [Product manager] Restore sang instance khác

Đặc tả cho phép chọn instance đích nhưng chưa nói backup có thể khôi phục chéo giữa hai instance hay không. Restore chéo có thể cần ánh xạ ID, URL, domain và bucket.

A. Chỉ restore về cùng instance nguồn
B. Cho restore sang instance khác cùng loại/phiên bản, preflight phải xác minh tương thích
C. Cho restore sang instance khác và yêu cầu hỗ trợ ánh xạ ID/URL/bucket trong đợt đầu
D. Chưa chốt; từ chối cross-instance restore trong đợt đầu
X. Other (please specify)

[Answer]: B — Khác instance tương thích

## Q5. [Operations engineer] Package, dung lượng và lưu trữ

ZIP chứa dữ liệu cá nhân và object riêng tư, nhưng giới hạn dung lượng, nơi giữ package và thời gian retention chưa được xác định. Chọn yêu cầu ban đầu nào?

A. Tải trực tiếp xuống máy admin; không giữ package phía server sau khi tải thành công
B. Lưu tạm riêng tư phía server để download/upload; giới hạn dung lượng và retention sẽ được cấu hình trước khi bật tính năng
C. Cho phép lưu backup dài hạn ở kho object được mã hóa; retention được cấu hình theo môi trường
D. Chưa chốt; cần quyết định chính sách lưu trữ trước khi triển khai backup thực
X. Other (please specify)

[Answer]: C — Kho object dài hạn

## Q6. [Security engineer] Mã hóa và khóa

Backup chứa dữ liệu người dùng, thông tin xác thực và nội dung Storage. Package cần được mã hóa khi lưu/chuyển và phục hồi qua khóa nào?

A. Mã hóa khi truyền và khi lưu bằng khóa do hệ thống quản lý; UI không nhận khóa bí mật
B. Mã hóa khi truyền; file tải xuống do admin tự bảo vệ sau khi tải
C. Admin cung cấp mật khẩu để mã hóa ZIP; mật khẩu không được lưu/log
D. Chưa chốt; không tạo package chứa dữ liệu thật trước khi duyệt mô hình khóa
X. Other (please specify)

[Answer]: B — Mã hóa khi truyền; admin tự bảo vệ file sau khi tải

## Q7. [Operations engineer] Xác nhận production và mục tiêu phục hồi

Restore thay thế dữ liệu đích và có thể làm mất dữ liệu. Ai được xác nhận restore production, và có cần người thứ hai phê duyệt không? Nếu đã có mục tiêu RPO/RTO, hãy nêu giá trị; nếu chưa, có thể ghi để quyết định ở thiết kế.

A. Admin được phân quyền có thể tự xác nhận; mục tiêu RPO/RTO sẽ xác định trong thiết kế
B. Production cần hai người phê duyệt; mục tiêu RPO/RTO sẽ xác định trong thiết kế
C. Chưa cho restore production trong đợt đầu; chỉ cho môi trường thử nghiệm
D. Có chính sách hiện hữu; mô tả chính sách và mục tiêu RPO/RTO
X. Other (please specify)

[Answer]: A — Một admin xác nhận; RPO/RTO xác định trong thiết kế

## Q8. [Product manager] Ranh giới kiểm thử localhost-only

Thực hành nhóm đã xác nhận là mọi thay đổi chỉ kiểm thử trên localhost, không có bước triển khai/release riêng; đồng thời Restore phải không phá hủy dữ liệu và production bị khóa cho đến khi quy trình được xác minh. Cách áp dụng nào phù hợp?

A. Chỉ xây và kiểm thử localhost; không hỗ trợ restore production trong đợt đầu
B. Kiểm thử localhost cho phát triển; restore production vẫn bị khóa cho đến khi restore drill/điều kiện an toàn được xác minh, không yêu cầu release riêng trong scope
C. Localhost-only có nghĩa không chạy restore drill ở bất kỳ môi trường đại diện nào
D. Chưa chốt; cần làm rõ ngoại lệ kiểm chứng an toàn trước khi production
X. Other (please specify)

[Answer]: A — Chỉ kiểm thử localhost; không hỗ trợ restore production ở đợt đầu

## Consolidated Summary Confirmation

- Chỉ hỗ trợ Supabase self-hosted do nhóm quản lý.
- Phạm vi dữ liệu dự kiến gồm My-CMS, GoTrue/Auth và Storage; password hash/session chưa được xác nhận riêng.
- Backup chấp nhận best-effort và phải nêu rõ giới hạn nhất quán.
- Cho phép restore chéo sang instance khác nếu cấu hình/phiên bản tương thích và preflight xác nhận.
- Backup được lưu dài hạn ở object store; retention và giới hạn dung lượng cần cấu hình.
- Package mã hóa khi truyền; admin chịu trách nhiệm bảo vệ file sau khi tải xuống.
- Một admin được phân quyền xác nhận restore; mục tiêu RPO/RTO để giai đoạn thiết kế xác định.
- Phiên bản đầu chỉ kiểm thử localhost và không hỗ trợ restore production.
- Thực hành nhóm: test-after, mục tiêu coverage 80% và CI xanh trước merge; tuy nhiên workflow CI trong repository hiện ghi tạm vô hiệu hóa.
- Thực hành nhóm: nhánh mới và PR cho mọi thay đổi; dùng formatter/linter hiện có.
- Chưa xác nhận password hash, Supabase/Postgres versions, object-store encryption/key policy, retention/quota và RPO/RTO.

Does this all look correct before I generate the requirements artifact?

Looks correct
Request changes

[Answer]: Looks correct

