# Phân tích yêu cầu — Sao lưu và khôi phục My-CMS

## Intent analysis

Quản trị viên cần tạo một bản backup có thể mang đi và khôi phục dữ liệu My-CMS giữa các instance Supabase self-hosted đã được cấu hình, thông qua giao diện admin. Bản backup phải bao quát dữ liệu My-CMS, người dùng/định danh GoTrue và object Storage theo phạm vi được xác minh; không được trình bày dữ liệu không thể thu thập/khôi phục như đã được bảo vệ. Mục tiêu là hỗ trợ phục hồi dữ liệu với luồng có preflight, xác nhận rõ target, kiểm tra tính toàn vẹn và thông báo kết quả.

Phạm vi đợt đầu chỉ hướng tới phát triển và kiểm thử localhost. Không hỗ trợ khôi phục vào production ở đợt đầu. Nhóm xác nhận các thay đổi chỉ kiểm thử localhost và không có bước phát hành/release riêng.

**Imports:** `aidlc/spaces/default/codekb/my-cms/business-overview.md`; `aidlc/spaces/default/codekb/my-cms/architecture.md`; `aidlc/spaces/default/codekb/my-cms/code-structure.md`; `aidlc/spaces/default/intents/261117-backup-restore/inception/practices-discovery/team-practices.md`; `docs/features/backup-restore.md`; `aidlc/spaces/default/knowledge/aidlc-shared/auth-users-media.md`; `aidlc/spaces/default/knowledge/aidlc-shared/backend-domain-architecture.md`; `aidlc/spaces/default/knowledge/aidlc-shared/deployment-operations.md`.

**Evidence boundary:** CodeKB là scan có độ sâu hỗn hợp, không phải rà soát toàn diện. Tài liệu feature mô tả mục tiêu, không chứng minh implementation hiện tại. Gateway CLI hiện là skeleton; chưa có luồng ZIP/UI/upload/restore hoàn chỉnh. Các hành vi chưa được xác nhận bằng source hoặc kiểm thử được ghi là yêu cầu mục tiêu, không phải mô tả hiện trạng.

## Functional requirements

### FR1 — Chọn instance và phân quyền

Hệ thống shall cho phép quản trị viên được phân quyền chọn một instance Supabase self-hosted đã cấu hình làm nguồn backup hoặc target restore. Không nhận connection string, credentials, service-role key hoặc endpoint tùy ý từ trình duyệt.

- **FR1.1:** Khi admin mở Backup & Restore, hệ thống hiển thị các instance được phép cùng nhãn môi trường dễ phân biệt; không có instance khả dụng thì nêu rõ và vô hiệu hóa thao tác tương ứng.
- **FR1.2:** Mọi thao tác liệt kê instance, tạo job, upload, preflight, download, xem trạng thái và restore phải kiểm tra quyền ở server; người không có quyền admin bị từ chối.
- **FR1.3:** Mỗi lần đọc job/package phải kiểm tra quyền truy cập của người gọi; biết hoặc đoán ID không cấp quyền truy cập.

### FR2 — Tạo backup ZIP từ admin UI

Hệ thống shall cho phép admin chọn instance nguồn và bắt đầu backup từ admin UI; xử lý có thể chạy thành job nền và hiển thị trạng thái.

- **FR2.1:** Job chỉ được đánh dấu thành công khi tất cả thành phần bắt buộc đã được thu thập, đóng gói và kiểm tra theo phạm vi được khai báo.
- **FR2.2:** ZIP phải chứa database My-CMS trong phạm vi đã xác nhận, dữ liệu GoTrue/Auth được nền tảng hỗ trợ khôi phục, Storage object bytes và metadata cần tái lập bucket/path, manifest phiên bản hóa, danh mục thành phần và checksum.
- **FR2.3:** Package hoặc hướng dẫn đi kèm phải chỉ rõ restore tool/command tương thích; nội dung do người dùng tải lên không được thực thi như script.
- **FR2.4:** Manifest phải ghi backup ID, instance nguồn (không có bí mật), thời gian, phiên bản định dạng/tool, mức nhất quán best-effort, phạm vi dữ liệu, số lượng/dung lượng object và checksum theo thành phần.
- **FR2.5:** Backup phải ghi rõ thành phần không xuất được, cảnh báo và phạm vi bị loại trừ; không được đánh dấu backup thành công nếu thiếu thành phần bắt buộc.

### FR3 — Package, lưu trữ và tải xuống

Hệ thống shall lưu package hoàn tất trong kho object dài hạn được cấu hình và cho phép admin tải package đã xác minh.

- **FR3.1:** Package chỉ tải được sau khi job hoàn tất và mọi kiểm tra integrity bắt buộc đạt.
- **FR3.2:** Hệ thống lưu metadata về ID, instance nguồn, trạng thái, thời gian, dung lượng và checksum; UI không hiển thị nội dung nhạy cảm hoặc credentials.
- **FR3.3:** Chính sách dung lượng tối đa, retention, xóa và truy cập kho phải được cấu hình trước khi dùng dữ liệu thật; nếu cấu hình bắt buộc thiếu hoặc không hợp lệ, hệ thống từ chối tạo backup thay vì dùng mặc định không an toàn.

### FR4 — Upload và preflight restore

Hệ thống shall cho phép admin chọn target instance và upload ZIP từ admin UI, sau đó kiểm tra package trước khi thay đổi target.

- **FR4.1:** UI hiển thị tiến độ upload/kiểm tra và kết quả preflight gồm nguồn package, target, phạm vi thành phần, tương thích và tác động thay thế.
- **FR4.2:** Preflight xác minh manifest/version, checksum, thành phần bắt buộc, khả năng hỗ trợ theo phiên bản Supabase/PostgreSQL và tương thích nguồn/target.
- **FR4.3:** Lần phát hành đầu chỉ hỗ trợ Supabase self-hosted. Restore chéo instance chỉ được cho phép khi preflight xác nhận hai instance thuộc cấu hình/phiên bản được hỗ trợ; nếu cần ánh xạ không được hỗ trợ thì phải từ chối.
- **FR4.4:** ZIP hỏng, thiếu thành phần, sai checksum, không tương thích hoặc vượt giới hạn được cấu hình phải bị từ chối trước restore; không được thay đổi dữ liệu target.
- **FR4.5:** Bộ giải nén phải từ chối đường dẫn tuyệt đối, `..`, symlink, entry trùng/xung đột, archive bomb, độ sâu quá mức và số lượng/dung lượng vượt giới hạn.

### FR5 — Xác nhận và thực hiện restore

Hệ thống shall chỉ bắt đầu restore sau khi preflight đạt và admin xác nhận tường minh đúng target cùng tác động thay thế.

- **FR5.1:** Mặc định restore thay thế dữ liệu thuộc phạm vi được hỗ trợ trên target, không merge. Màn hình xác nhận phải nêu target, package và dữ liệu sẽ bị thay thế.
- **FR5.2:** Upload, preflight, hủy hoặc chưa xác nhận không làm thay đổi target.
- **FR5.3:** Trước khi thay đổi target, hệ thống phải xác nhận có điểm phục hồi/rollback phù hợp; nếu không thể thiết lập hoặc xác minh thì từ chối bắt đầu restore.
- **FR5.4:** Hệ thống chỉ chạy restore implementation đã được phê duyệt; không thực thi command/script lấy từ ZIP.
- **FR5.5:** Restore phải xác minh kết quả database/Auth/Storage theo phạm vi hỗ trợ và không báo thành công nếu một thành phần bắt buộc hoặc kiểm tra sau restore thất bại.
- **FR5.6:** Khi restore thất bại, hệ thống phải rollback hoặc đánh dấu rõ `recovery-required`, nêu liệu rollback hoàn tất hay cần can thiệp; không được báo thành công một phần.

### FR6 — Job, lịch sử và xử lý cạnh tranh

Hệ thống shall cung cấp trạng thái và lịch sử job cho backup/restore trong admin UI.

- **FR6.1:** UI hiển thị job ID, trạng thái queued/running/succeeded/failed, thời điểm, tiến độ nếu đo được và thông báo lỗi an toàn.
- **FR6.2:** Yêu cầu lặp do double-submit phải không tạo job trùng; restore xung đột trên cùng target phải được tuần tự hóa hoặc từ chối rõ ràng.
- **FR6.3:** Job bị dừng giữa chừng không được trình bày như hoàn tất; tài nguyên staging tạm phải được dọn theo chính sách đã cấu hình.
- **FR6.4:** Audit ghi actor, source/target, job/package ID, digest, thời gian, xác nhận và kết quả/recovery status; không ghi credentials, token, mật khẩu, object contents hoặc raw database rows.

## Non-functional requirements

### NFR1 — Bảo mật và quyền

Mọi endpoint và thao tác package/job phải thực thi kiểm tra xác thực và quyền quản trị viên phía server; credentials của instance chỉ được giữ phía server trong secret/config store có kiểm soát. Không tiết lộ secret trong UI, phản hồi lỗi, log, audit hoặc metric. Kết nối admin/API, database và object store phải dùng TLS.

### NFR2 — Bảo vệ package và dữ liệu nhạy cảm

Package được truyền qua kênh mã hóa. Theo quyết định nhóm, file sau khi tải xuống do admin tự bảo vệ; do đó UI và hướng dẫn phải cảnh báo package chứa dữ liệu cá nhân/nhạy cảm và yêu cầu lưu tại nơi được kiểm soát. Bản lưu dài hạn trong object store phải dùng cơ chế mã hóa at-rest do nền tảng/kho cung cấp và quyền truy cập tối thiểu. Không đưa credentials/secrets vào package. Trước khi xử lý dữ liệu thật, giới hạn kích thước, retention và chính sách xóa phải được cấu hình.

### NFR3 — Tính toàn vẹn và tính nhất quán

Mọi thành phần phải có checksum mật mã được kiểm tra trước restore; số lượng và dung lượng thành phần phải đối chiếu với manifest. Mức nhất quán được công bố là **best-effort**, không phải point-in-time: package phải ghi timestamp/phương thức chụp và cảnh báo rằng database, Auth, metadata và object bytes có thể phản ánh thời điểm khác nhau. Không được quảng cáo tính nhất quán xuyên hệ thống nếu chưa chứng minh.

### NFR4 — An toàn xử lý archive

Việc kiểm tra/extract phải có giới hạn cấu hình cho kích thước upload, kích thước giải nén, số entry, độ sâu, thời gian và dung lượng tạm. Mọi lỗi vượt giới hạn hoặc cấu trúc nguy hiểm phải kết thúc preflight an toàn trước khi target bị thay đổi.

### NFR5 — Khả năng phục hồi và vận hành

Phải ghi nhận thời lượng, số byte/object, kết quả từng thành phần, trạng thái rollback và thời gian phục hồi; không dùng dữ liệu nhạy cảm làm nhãn metric. RPO/RTO cụ thể chưa được xác nhận và phải được xác định ở giai đoạn thiết kế trước khi tuyên bố mục tiêu khôi phục.

### NFR6 — Khả năng kiểm thử và chất lượng

Theo thực hành nhóm, áp dụng `test-after`, triển khai rồi kiểm thử từng lớp, đặt mục tiêu bao phủ dòng 80% và yêu cầu CI xanh trước khi hợp nhất. Nhóm chỉ kiểm thử thay đổi trên localhost và không có bước release riêng; vì vậy phiên bản đầu không hỗ trợ restore production. CI hiện được ghi là tạm vô hiệu hóa; cơ chế kiểm tra coverage/CI phải được khôi phục hoặc xác minh trước khi tiêu chí này được coi là có thể thực thi.

### NFR7 — Giao diện và khả năng tiếp cận

Các trạng thái loading, empty, error và success phải phân biệt được; lỗi phải nêu bước tiếp theo mà không tiết lộ dữ liệu nhạy cảm. Mọi thao tác cốt lõi phải sử dụng được bằng bàn phím và trạng thái không chỉ được biểu đạt bằng màu sắc.

## Constraints

- Lần phát hành đầu chỉ hỗ trợ instance Supabase self-hosted đã được cấu hình; không nhận endpoint tùy ý từ client.
- Phạm vi dữ liệu là database My-CMS, GoTrue/Auth và Supabase Storage; không mặc định bao gồm project settings, secrets, provider configuration, edge functions, deployment config hoặc toàn bộ schema Supabase do dịch vụ quản lý.
- Restore mặc định là replace trong phạm vi; không merge.
- Tạo backup có mức nhất quán best-effort theo lựa chọn của người dùng; không khẳng định point-in-time.
- Restore chéo instance được yêu cầu, nhưng chỉ khi cùng nằm trong cấu hình/phiên bản tương thích và preflight xác nhận; ánh xạ tùy chỉnh không nằm trong đợt đầu.
- Backup dài hạn ở object store; file tải về do admin tự bảo vệ sau khi tải. Mô hình khóa/retention/dung lượng phải được cấu hình trước khi dùng dữ liệu thật.
- Giai đoạn đầu chỉ phát triển và kiểm thử localhost; không cho restore production.
- Làm việc trên nhánh mới và dùng pull request cho mọi thay đổi. Dùng formatter/linter cấu hình sẵn có; không thêm quy tắc riêng.

## Assumptions

| ID | Giả định | Lý do / chủ sở hữu cần xác nhận |
|---|---|---|
| AS1 | Dữ liệu Auth có thể được xuất/khôi phục theo cách nền tảng Supabase self-hosted hỗ trợ mà không làm lộ password hash. | Chưa xác nhận đường dẫn kỹ thuật; platform/backend owner xác minh. Nếu không hỗ trợ, cần thay đổi phạm vi trước triển khai. |
| AS2 | Password hash không được cam kết giữ nguyên; yêu cầu về việc người dùng tiếp tục đăng nhập bằng mật khẩu cũ chưa được trả lời riêng. | Câu trả lời chọn nhóm dữ liệu My-CMS/Auth/Storage nhưng không xác định chính sách password/session. Product owner xác nhận trước thiết kế Auth restore. |
| AS3 | Kho object dài hạn cung cấp mã hóa at-rest và quyền truy cập riêng tư có thể cấu hình. | Người dùng chọn kho object dài hạn; loại kho, khóa, retention và xóa chưa được chỉ định. Platform owner xác nhận trước khi dùng dữ liệu thật. |
| AS4 | Các instance self-hosted có thể được phân loại theo phiên bản/cấu hình để quyết định tương thích chéo. | Cần xác định ma trận hỗ trợ trước khi cho phép cross-instance restore. |
| AS5 | Quy trình best-effort sẽ phát hiện ít nhất các lỗi liệt kê/tải thiếu object và ghi rõ mức nhất quán, nhưng không bảo đảm cùng một thời điểm xuyên database/Auth/Storage. | Người dùng chấp nhận best-effort; mức phát hiện thay đổi giữa lúc backup cần được thiết kế và kiểm thử. |
| AS6 | Dung lượng tối đa, retention, RPO/RTO và thời gian timeout được xác định trong thiết kế/triển khai cấu hình trước khi chạy với dữ liệu thật. | Chưa có giá trị cụ thể được cung cấp; không tự đặt ngưỡng tùy ý trong yêu cầu. |

## Out of scope

- Khôi phục production trong phiên bản đầu.
- Supabase hosted và các instance ngoài danh sách self-hosted được cấu hình.
- Khôi phục project settings, secrets, nhà cung cấp Auth, edge functions, deployment configuration hoặc bảo đảm khôi phục toàn bộ schema Supabase do dịch vụ quản lý.
- Merge dữ liệu giữa nguồn và đích.
- Ánh xạ tùy chỉnh ID, URL, domain hoặc bucket cho cross-instance restore.
- Mã hóa ZIP bằng mật khẩu do người dùng nhập; phương thức bảo vệ file tải xuống do admin quản lý.

## Open questions

1. Những schema/table cụ thể nào thuộc My-CMS và schema nào bị loại khỏi dump? Phương thức được hỗ trợ để khôi phục GoTrue users/identities là gì?
2. Có cần giữ password hash để user đăng nhập bằng mật khẩu cũ không? Session/token sau restore được giữ, vô hiệu hóa hay yêu cầu đăng nhập lại?
3. Kho object dài hạn nào được dùng; ai sở hữu khóa; retention/xóa và giới hạn dung lượng package là bao nhiêu?
4. Ma trận phiên bản Supabase/PostgreSQL nào được hỗ trợ để cho phép restore chéo instance?
5. Quy trình best-effort xác định object/database thay đổi trong khi backup như thế nào và khi nào phải đánh dấu backup không thể restore?
6. Ai xác định RPO/RTO và giới hạn timeout/concurrency cho backup/restore?
7. Cổng CI xanh và 80% coverage đã được nhóm chọn, nhưng workflow hiện bị vô hiệu hóa. Cơ chế CI nào sẽ thực thi các tiêu chí đó?
8. Mặc định tổ chức là deploy-on-merge lên staging, còn thực hành nhóm được xác nhận là localhost-only không release riêng. Cơ chế promotion nào áp dụng khi feature này được đưa ra khỏi localhost sau đợt đầu?
