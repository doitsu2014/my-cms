# Đặc tả nghiệp vụ: Sao lưu và khôi phục My-CMS

## 1. Tổng quan

### 1.1 Mục tiêu

Cho phép quản trị viên sao lưu dữ liệu của một instance My-CMS/Supabase thành một tệp ZIP có thể tải xuống, sau đó tải ZIP lên từ giao diện admin để khôi phục vào một instance đích được cấu hình sẵn.

Feature phải hỗ trợ chọn instance nguồn khi sao lưu và chọn instance đích khi khôi phục. Cả thao tác tạo backup và upload/import để restore đều phải thực hiện được từ UI admin.

### 1.2 Đối tượng sử dụng

- **Quản trị viên hệ thống:** chọn instance, tạo/tải backup, tải ZIP lên, xem kết quả kiểm tra và xác nhận restore.
- **Người vận hành:** theo dõi tiến trình, xử lý lỗi và thực hiện quy trình khôi phục khi có sự cố.
- **Người dùng CMS:** đối tượng được bảo vệ dữ liệu; không trực tiếp thao tác feature này.

### 1.3 Thuật ngữ

| Thuật ngữ | Ý nghĩa |
| --- | --- |
| Instance | Một môi trường My-CMS/Supabase đã được cấu hình và được phép chọn trong admin, ví dụ staging hoặc production. |
| Backup package | Tệp ZIP chứa dữ liệu backup, thông tin kiểm tra tính toàn vẹn và hướng dẫn/công cụ restore tương thích. |
| Restore | Khôi phục dữ liệu từ backup package vào instance đích. |
| Preflight | Bước kiểm tra package, khả năng tương thích, phạm vi dữ liệu và tác động lên instance đích trước khi restore. |

## 2. Phạm vi nghiệp vụ

### 2.1 Dữ liệu cần sao lưu

Một backup hoàn chỉnh phải bao gồm các nhóm dữ liệu thuộc phạm vi được phê duyệt cho instance:

1. **Database:** dữ liệu My-CMS trong PostgreSQL, gồm các schema/bảng và quan hệ thuộc hệ thống.
2. **GoTrue/Auth:** người dùng và dữ liệu định danh cần thiết để khôi phục tài khoản theo phương thức được Supabase hỗ trợ.
3. **Storage:** toàn bộ object/blob bytes thuộc các bucket trong phạm vi backup, cùng bucket, đường dẫn và metadata cần thiết để nhận diện object.
4. **Thông tin gói backup:** manifest, phiên bản định dạng, danh mục thành phần, dung lượng và checksum.
5. **Restore:** hướng dẫn và restore command/tool tương thích với định dạng package.

Không được tuyên bố package bao gồm dữ liệu hoặc cấu hình mà quy trình thực tế không thể xuất và khôi phục. Khả năng xuất/khôi phục GoTrue, schema do Supabase quản lý, cấu hình Auth và các thành phần nền tảng phải được xác minh theo phiên bản và mô hình hosting được hỗ trợ.

### 2.2 Ngoài phạm vi mặc định

Trừ khi được bổ sung và phê duyệt riêng, package không bao gồm project settings, secrets, cấu hình nhà cung cấp đăng nhập, edge functions, cấu hình triển khai hoặc các thiết lập Supabase không nằm trong dữ liệu database/Storage đã thống nhất.

### 2.3 Quy tắc khôi phục mặc định

Restore thay thế dữ liệu thuộc phạm vi đã chọn trên instance đích; không phải thao tác merge. Trước khi chạy, admin phải thấy rõ instance đích và tác động thay thế, sau đó xác nhận riêng. Không được thay đổi dữ liệu đích trong bước upload hoặc preflight.

## 3. Sơ đồ quy trình người dùng

```mermaid
flowchart TD
    A[Admin mở Backup & Restore] --> B{Chọn thao tác}
    B -->|Tạo backup| C[Chọn instance nguồn]
    C --> D[Yêu cầu tạo backup]
    D --> E[Hệ thống thu thập database, Auth và Storage]
    E --> F{Đủ thành phần và kiểm tra hợp lệ?}
    F -->|Không| G[Đánh dấu thất bại, hiển thị lỗi an toàn]
    F -->|Có| H[Đóng gói ZIP và xác minh checksum]
    H --> I[Admin tải ZIP đã xác minh]
    B -->|Khôi phục| J[Chọn instance đích và tải ZIP lên]
    J --> K[Preflight: kiểm tra package và tương thích]
    K --> L{Preflight đạt?}
    L -->|Không| M[Từ chối restore, không thay đổi dữ liệu đích]
    L -->|Có| N[Hiển thị nguồn, đích và tác động thay thế]
    N --> O{Admin xác nhận?}
    O -->|Không| P[Hủy, không thay đổi dữ liệu đích]
    O -->|Có| Q[Tạo điểm phục hồi và chạy restore]
    Q --> R{Restore và kiểm tra sau restore đạt?}
    R -->|Không| S[Rollback hoặc báo cần can thiệp]
    R -->|Có| T[Hoàn tất và ghi audit]
```

## 4. User stories và tiêu chí chấp nhận

### US-01 — Chọn instance nguồn để tạo backup

**Là** quản trị viên, **tôi muốn** chọn một instance nguồn đã được cấu hình, **để** sao lưu đúng môi trường cần bảo vệ.

**Acceptance criteria**

- **Given** tôi đã đăng nhập với quyền quản trị viên và có ít nhất một instance được phép backup, **When** tôi mở trang Backup & Restore, **Then** hệ thống hiển thị danh sách instance được phép chọn cùng nhãn môi trường dễ nhận biết.
- **Given** tôi không có quyền quản trị viên, **When** tôi truy cập trang hoặc gọi trực tiếp chức năng liệt kê instance, **Then** hệ thống từ chối và không tiết lộ danh sách instance.
- **Given** không có instance nào được cấu hình hoặc khả dụng, **When** trang được tải, **Then** hệ thống thông báo không thể tạo backup và không cho bắt đầu job.

### US-02 — Tạo và tải backup ZIP

**Là** quản trị viên, **tôi muốn** tạo backup từ UI và tải xuống một ZIP hoàn chỉnh, **để** lưu trữ hoặc dùng cho khôi phục.

**Acceptance criteria**

- **Given** tôi đã chọn instance nguồn hợp lệ, **When** tôi yêu cầu tạo backup, **Then** hệ thống tạo một job và UI hiển thị mã job cùng trạng thái.
- **Given** job đang chạy, **When** tôi xem trang trạng thái, **Then** UI hiển thị trạng thái hiện tại, thời điểm và tiến độ nếu đo được; lỗi được diễn đạt an toàn, không lộ dữ liệu nhạy cảm.
- **Given** một thành phần bắt buộc như database, Auth hoặc object Storage không thể thu thập đầy đủ, **When** job kết thúc, **Then** job được đánh dấu thất bại và package không được hiển thị như một backup có thể restore.
- **Given** mọi thành phần bắt buộc đã được thu thập và xác minh, **When** job hoàn tất, **Then** package ZIP chứa các thành phần đã thống nhất, manifest, checksum và restore command/tool hoặc hướng dẫn tương thích.
- **Given** package chưa hoàn tất hoặc checksum không hợp lệ, **When** tôi yêu cầu tải xuống, **Then** hệ thống từ chối tải package đó như một bản backup hợp lệ.
- **Given** package đã được xác minh, **When** tôi tải ZIP, **Then** hệ thống cung cấp đúng package gắn với job và instance nguồn đã chọn.

### US-03 — Tải ZIP lên và kiểm tra trước restore

**Là** quản trị viên, **tôi muốn** upload package và chạy preflight trước khi restore, **để** biết package có thể được áp dụng an toàn vào instance đích hay không.

**Acceptance criteria**

- **Given** tôi có quyền quản trị viên, **When** tôi chọn instance đích và tải ZIP từ UI admin, **Then** hệ thống tiếp nhận package trong vùng lưu trữ riêng tư và hiển thị trạng thái upload/kiểm tra.
- **Given** ZIP bị lỗi, thiếu manifest/thành phần bắt buộc, checksum sai, định dạng không được hỗ trợ hoặc package không tương thích với target, **When** preflight chạy, **Then** hệ thống từ chối package, giải thích lý do ở mức an toàn và không thay đổi dữ liệu đích.
- **Given** ZIP vượt giới hạn dung lượng/số lượng entry hoặc có đường dẫn không an toàn, **When** package được kiểm tra, **Then** hệ thống từ chối package trước khi thực hiện restore.
- **Given** preflight thành công, **When** kết quả hiển thị, **Then** admin thấy được nguồn package, instance đích, phạm vi thành phần, khả năng tương thích, tác động thay thế và cảnh báo liên quan.

### US-04 — Xác nhận và thực hiện restore

**Là** quản trị viên, **tôi muốn** xác nhận rõ instance đích trước khi restore, **để** tránh ghi đè nhầm môi trường.

**Acceptance criteria**

- **Given** preflight đã thành công, **When** hệ thống yêu cầu xác nhận, **Then** màn hình nêu rõ tên/môi trường instance đích, package được dùng và dữ liệu nào sẽ bị thay thế.
- **Given** tôi chưa xác nhận hoặc hủy thao tác, **When** tôi rời bước xác nhận, **Then** restore không chạy và dữ liệu đích không bị thay đổi.
- **Given** tôi xác nhận restore, **When** hệ thống bắt đầu công việc, **Then** hệ thống ghi nhận job/audit và tạo hoặc xác nhận có điểm phục hồi đủ để xử lý lỗi trước khi thay đổi dữ liệu đích.
- **Given** restore hoàn tất, **When** kiểm tra sau restore thành công, **Then** job được đánh dấu thành công và kết quả kiểm tra được ghi nhận.
- **Given** một thành phần restore hoặc kiểm tra sau restore thất bại, **When** job kết thúc, **Then** hệ thống không báo thành công; hệ thống rollback hoặc hiển thị rõ trạng thái cần khôi phục/can thiệp thủ công.

### US-05 — Ngăn truy cập và thao tác trái phép

**Là** chủ sở hữu hệ thống, **tôi muốn** các thao tác backup/restore chỉ dành cho quản trị viên được cấp quyền, **để** bảo vệ dữ liệu và bí mật của các instance.

**Acceptance criteria**

- **Given** người dùng không có quyền quản trị viên, **When** họ gọi trực tiếp bất kỳ chức năng liệt kê instance, tạo job, upload, preflight, download hoặc restore nào, **Then** hệ thống từ chối ở phía server.
- **Given** người dùng không được phép truy cập một job/package, **When** họ đoán hoặc sử dụng ID của job/package đó, **Then** hệ thống không trả dữ liệu hay tệp của tài nguyên đó.
- **Given** backup hoặc restore được thực hiện, **When** audit được ghi, **Then** audit ghi actor, instance nguồn/đích, package/job, thời điểm và kết quả nhưng không ghi credentials, token, mật khẩu hay nội dung object/database.
- **Given** package được tạo hoặc upload, **When** dữ liệu được truyền hoặc lưu tạm/lưu trữ, **Then** dữ liệu được bảo vệ theo chính sách mã hóa và quyền truy cập đã phê duyệt.

### US-06 — Theo dõi kết quả và xử lý lỗi

**Là** quản trị viên hoặc người vận hành, **tôi muốn** xem trạng thái cuối cùng và hướng xử lý khi job thất bại, **để** biết dữ liệu đang an toàn hay cần can thiệp.

**Acceptance criteria**

- **Given** backup hoặc restore đang chạy, **When** tôi xem job, **Then** hệ thống hiển thị trạng thái và mã tham chiếu có thể dùng để tra cứu.
- **Given** job thất bại, **When** kết quả được hiển thị, **Then** thông báo nêu thành phần/bước thất bại ở mức an toàn và trạng thái rollback hoặc yêu cầu can thiệp.
- **Given** job đã kết thúc, **When** người có quyền xem audit, **Then** họ có thể xác định ai thực hiện, instance liên quan, thời điểm, kết quả và package/job tương ứng.

## 5. Quy tắc nghiệp vụ

| ID | Quy tắc |
| --- | --- |
| BR-01 | Chỉ instance được cấu hình phía server và được phép theo quyền người dùng mới xuất hiện trong UI. |
| BR-02 | Thông tin kết nối, database credentials, service-role key và endpoint tùy ý không được gửi từ browser để lựa chọn instance. |
| BR-03 | Một backup chỉ được coi là hoàn tất khi mọi thành phần bắt buộc đã được thu thập, đóng gói và xác minh theo tiêu chuẩn đã công bố. |
| BR-04 | Restore mặc định thay thế dữ liệu thuộc phạm vi được hỗ trợ trên instance đích; không âm thầm merge. |
| BR-05 | Upload và preflight không được thay đổi dữ liệu instance đích. Chỉ job restore sau xác nhận tường minh mới được phép thay đổi dữ liệu. |
| BR-06 | Không thực thi script hoặc command do ZIP tải lên cung cấp. Restore phải dùng công cụ đã được hệ thống phê duyệt và hỗ trợ phiên bản package đó. |
| BR-07 | Khi restore lỗi, hệ thống phải rollback hoặc báo trạng thái recovery-required; không được báo thành công một phần. |
| BR-08 | Chặn job restore đồng thời có khả năng xung đột trên cùng target; tránh tạo job trùng do submit lặp. |
| BR-09 | Package backup chứa dữ liệu nhạy cảm, không chứa credentials/secrets và phải tuân thủ chính sách lưu trữ, mã hóa, retention và xóa đã được phê duyệt. |

## 6. Yêu cầu phi chức năng và kiểm soát

- **Bảo mật:** phân quyền server-side cho mọi thao tác; bảo vệ dữ liệu truyền và lưu; che giấu thông tin nhạy cảm trong log, lỗi, audit và metric.
- **An toàn ZIP:** từ chối path traversal, đường dẫn tuyệt đối, symlink, entry trùng/xung đột, archive bomb, file quá lớn và package vượt giới hạn cấu hình.
- **Tính toàn vẹn:** kiểm tra checksum trước khi restore và xác minh dữ liệu sau restore theo phạm vi đã định nghĩa.
- **Tính nhất quán:** công bố phương thức và giới hạn nhất quán của backup. Không gọi là point-in-time nếu chưa chứng minh được sự nhất quán giữa database, Auth và object storage.
- **Khả năng phục hồi:** có quy trình rollback/khôi phục, giới hạn thời gian lưu bản tạm và bài kiểm tra restore định kỳ trước khi bật production.
- **Vận hành:** ghi nhận thời lượng, số byte/object, kết quả theo thành phần và thời gian phục hồi; metric không chứa thông tin định danh cá nhân hoặc bí mật.

## 7. Điều kiện phát hành

Không bật restore cho production trước khi các điều kiện sau được xác minh và phê duyệt:

1. Phạm vi schema/bảng và phương thức khôi phục GoTrue/Auth được xác nhận cho phiên bản Supabase hỗ trợ.
2. Phương thức lấy/khôi phục object Storage, bucket metadata và liên kết dữ liệu được kiểm chứng.
3. Mức nhất quán được công bố; giới hạn và ảnh hưởng lên session người dùng được nêu rõ.
4. Có mã hóa và quản lý khóa, giới hạn upload/package, chính sách retention/xóa.
5. Quy trình tạo điểm phục hồi, rollback và xử lý restore thất bại đã được kiểm thử.
6. Restore drill thành công trên môi trường đại diện; có tiêu chí thời gian phục hồi và mức mất dữ liệu chấp nhận được.
7. Với production, quyền xác nhận bổ sung/second-person approval được quyết định và áp dụng nếu chính sách yêu cầu.

## 8. Giả định và câu hỏi cần BA/Product xác nhận

1. Những môi trường Supabase nào được hỗ trợ: self-hosted, Supabase hosted hay cả hai? Phiên bản nào?
2. “Toàn bộ dữ liệu Supabase” có nghĩa là dữ liệu My-CMS cùng GoTrue và Storage, hay bao gồm cả schema/dữ liệu hệ thống do Supabase quản lý? Cần chốt danh sách bao gồm/loại trừ.
3. Có yêu cầu giữ password hash để user đăng nhập bằng mật khẩu cũ sau restore không? Phương thức khôi phục được nền tảng hỗ trợ là gì?
4. Tạo backup có được phép tạm dừng ghi dữ liệu không? Nếu không, mức độ nhất quán chấp nhận được là gì?
5. Instance đích có thể khác instance nguồn không? Có cần ánh xạ ID, URL, domain hoặc bucket giữa hai môi trường không?
6. Giới hạn dung lượng ZIP, thời gian giữ backup và nơi lưu package là gì?
7. Ai được restore vào production? Có cần người thứ hai phê duyệt không?
8. Mục tiêu RPO/RTO cần đạt là bao nhiêu và tần suất restore drill ra sao?

## 9. Hiện trạng triển khai

Gateway hiện có CLI skeleton tại `apps/api/gateway/src/backup_cli.rs`, nhưng lệnh backup/list chưa có S3 transfer adapter và restore đang từ chối chạy vì quy trình phục hồi đầy đủ cho Supabase chưa được xác minh. Hiện chưa có luồng ZIP hoàn chỉnh, UI admin, upload/import restore hoặc chọn instance. Vì vậy, đây là đặc tả mục tiêu; không nên coi các tiêu chí trên là đã được triển khai.
