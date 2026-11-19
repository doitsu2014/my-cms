# User Stories — Sao lưu và khôi phục My-CMS

## Hành trình 1 — Tạo và lấy backup

### US1.1 — Chọn instance nguồn

**As a** Quản trị viên My-CMS, **I want** chọn một instance Supabase self-hosted đã cấu hình, **so that** backup được tạo từ đúng môi trường.

**Priority:** Must Have  
**Traces to:** FR1, FR1.1, FR1.2

#### Acceptance criteria

- **AC1.1.1** — **Given** admin có quyền và có ít nhất một instance được phép, **When** admin mở Backup & Restore, **Then** hệ thống hiển thị các instance được phép với nhãn môi trường rõ ràng.
- **AC1.1.2** — **Given** không có instance khả dụng, **When** admin mở trang, **Then** hệ thống nêu rõ chưa cấu hình instance và không cho bắt đầu backup/restore.
- **AC1.1.3** — **Given** người không có quyền admin, **When** họ gọi trực tiếp API liệt kê instance, **Then** server từ chối và không tiết lộ danh sách.
- **AC1.1.4** — **Given** request từ browser, **When** request chứa connection string, credential, service-role key hoặc endpoint tùy ý, **Then** server không chấp nhận các giá trị đó để chọn/kết nối instance.

**Dependencies:** none.  
**INVEST:** Independent, Valuable, Estimable, Small, Testable. Negotiable: nhãn/các thông tin nhận diện hiển thị.

### US1.2 — Tạo backup ZIP đã kiểm tra

**As a** Quản trị viên My-CMS, **I want** tạo backup cho instance đã chọn và nhận ZIP có manifest/checksum, **so that** tôi có thể lưu và dùng package để phục hồi dữ liệu trong phạm vi hỗ trợ.

**Priority:** Must Have  
**Traces to:** FR2, FR2.1, FR2.2, FR2.3, FR2.4, FR2.5, NFR3

#### Acceptance criteria

- **AC1.2.1** — **Given** admin chọn instance hợp lệ, **When** admin bắt đầu backup, **Then** hệ thống tạo job và hiển thị job ID cùng trạng thái.
- **AC1.2.2** — **Given** backup thành công, **When** package được tạo, **Then** ZIP chứa các thành phần database My-CMS, GoTrue/Auth và Storage đã được xác nhận/khả năng khôi phục; manifest nêu rõ phạm vi, timestamp, mức nhất quán best-effort, phiên bản và inventory.
- **AC1.2.3** — **Given** package được tạo, **When** hệ thống ghi checksums, **Then** mỗi thành phần được ghi digest, kích thước và được xác minh trước khi job chuyển sang succeeded.
- **AC1.2.4** — **Given** một thành phần bắt buộc không thu thập được hoặc kết quả bị thiếu, **When** job kết thúc, **Then** job là failed và ZIP không được cung cấp như backup hoàn chỉnh.
- **AC1.2.5** — **Given** có lỗi khi đọc database, Auth hoặc Storage, **When** UI hiển thị lỗi, **Then** lỗi nêu thành phần/bước ở mức an toàn, không lộ secret hoặc dữ liệu người dùng.
- **AC1.2.6** — **Given** backup diễn ra khi dữ liệu nguồn có thể đang thay đổi, **When** package hoàn tất, **Then** package công bố best-effort consistency và không tự nhận là point-in-time consistent.

**Dependencies:** US1.1.  
**INVEST:** Independent, Valuable, Estimable, Testable. Small: cần xác định khả năng chia theo dung lượng/object và nền tảng ở thiết kế. Negotiable: chi tiết định dạng manifest trong giới hạn contract.

### US1.3 — Tải và giữ backup package

**As a** Quản trị viên My-CMS, **I want** tải package đã xác minh và có bản lưu ở kho object dài hạn, **so that** tôi có thể giữ package để dùng cho quy trình restore.

**Priority:** Must Have  
**Traces to:** FR3, FR3.1, FR3.2, FR3.3, NFR2

#### Acceptance criteria

- **AC1.3.1** — **Given** job backup succeeded và checksum hợp lệ, **When** admin chọn tải, **Then** hệ thống trả về package đúng job/instance nguồn.
- **AC1.3.2** — **Given** job chưa hoàn tất hoặc checksum không hợp lệ, **When** admin yêu cầu download, **Then** hệ thống từ chối download dưới trạng thái package hợp lệ.
- **AC1.3.3** — **Given** package được lưu trong object store, **When** policy về dung lượng, retention, mã hóa at-rest hoặc quyền truy cập chưa cấu hình hợp lệ, **Then** hệ thống từ chối xử lý dữ liệu thật và nêu cấu hình cần hoàn tất.
- **AC1.3.4** — **Given** admin tải package, **When** download hoàn tất, **Then** UI nhắc rõ package chứa dữ liệu nhạy cảm và admin chịu trách nhiệm bảo vệ file đã tải.
- **AC1.3.5** — **Given** người không được phép truy cập package biết package ID, **When** họ thử tải, **Then** server từ chối.

**Dependencies:** US1.2.  
**INVEST:** Independent, Valuable, Estimable, Testable. Negotiable: UX quản lý danh sách và retention policy cụ thể.

## Hành trình 2 — Upload, preflight và restore

### US2.1 — Upload package và preflight target

**As a** Quản trị viên My-CMS, **I want** upload ZIP và kiểm tra tương thích với target trước khi restore, **so that** package lỗi hoặc không phù hợp không thể làm thay đổi target.

**Priority:** Must Have  
**Traces to:** FR4, FR4.1, FR4.2, FR4.3, FR4.4, FR4.5, NFR4

#### Acceptance criteria

- **AC2.1.1** — **Given** admin đã chọn target self-hosted được cấu hình, **When** admin upload ZIP từ UI, **Then** hệ thống hiển thị tiến độ upload và tiếp nhận package vào vùng lưu riêng tư.
- **AC2.1.2** — **Given** package upload xong, **When** preflight chạy, **Then** kết quả nêu nguồn package, target, thành phần, checksum, version/compatibility và tác động replace mà không thay đổi dữ liệu target.
- **AC2.1.3** — **Given** package bị hỏng, thiếu manifest/thành phần, sai checksum hoặc version không được hỗ trợ, **When** preflight chạy, **Then** hệ thống từ chối restore và giữ target không đổi.
- **AC2.1.4** — **Given** archive có path tuyệt đối, `..`, symlink, entry xung đột, vượt số lượng/độ sâu/dung lượng/giải nén tối đa, **When** kiểm tra archive, **Then** hệ thống từ chối trước khi extract dữ liệu vào vị trí không an toàn hoặc thay đổi target.
- **AC2.1.5** — **Given** nguồn và target là hai instance khác nhau, **When** phiên bản/configuration không khớp danh sách được hỗ trợ hoặc cần ánh xạ tùy chỉnh ngoài phạm vi, **Then** preflight từ chối và nêu nguyên nhân.
- **AC2.1.6** — **Given** upload hoặc preflight thất bại, **When** UI nhận kết quả, **Then** UI hiển thị trạng thái lỗi, bước xử lý tiếp theo và không làm lộ package contents/credentials.

**Dependencies:** US1.1, US1.3.  
**INVEST:** Independent, Valuable, Estimable, Testable. Negotiable: chi tiết progress UI và metadata hiển thị.

### US2.2 — Xác nhận và chạy restore

**As a** Quản trị viên My-CMS, **I want** xác nhận rõ instance đích và phạm vi dữ liệu bị thay thế trước khi restore, **so that** tôi không ghi đè nhầm instance và hiểu tác động.

**Priority:** Must Have  
**Traces to:** FR5, FR5.1, FR5.2, FR5.3, FR5.4, FR5.5, FR5.6

#### Acceptance criteria

- **AC2.2.1** — **Given** preflight đạt, **When** UI yêu cầu xác nhận, **Then** UI nêu tên/môi trường target, package nguồn và dữ liệu bị replace.
- **AC2.2.2** — **Given** admin hủy hoặc chưa xác nhận, **When** bước xác nhận kết thúc, **Then** restore không chạy và target không thay đổi.
- **AC2.2.3** — **Given** admin xác nhận, **When** hệ thống chuẩn bị restore, **Then** hệ thống xác nhận có điểm rollback/recovery khả dụng; nếu không, từ chối bắt đầu trước khi thay đổi target.
- **AC2.2.4** — **Given** target được thay đổi, **When** restore chạy, **Then** hệ thống sử dụng implementation đã phê duyệt và không thực thi script/command từ ZIP.
- **AC2.2.5** — **Given** tất cả thành phần restore và post-restore validation đạt, **When** job kết thúc, **Then** hệ thống báo succeeded cùng kết quả đối chiếu integrity/counts đã hỗ trợ.
- **AC2.2.6** — **Given** một thành phần restore hoặc post-restore validation thất bại, **When** job kết thúc, **Then** hệ thống không báo succeeded; rollback hoặc recovery-required được ghi rõ.
- **AC2.2.7** — **Given** request restore lặp hoặc có job restore đang chạy trên cùng target, **When** request mới đến, **Then** hệ thống không tạo thao tác trùng/chồng lấn; trả trạng thái hiện có hoặc từ chối rõ ràng.

**Dependencies:** US2.1.  
**INVEST:** Independent, Valuable, Estimable, Testable. Small: khả năng rollback qua nhiều hệ thống cần được xác nhận ở thiết kế.

## Hành trình 3 — Theo dõi và audit

### US3.1 — Theo dõi job và xử lý lỗi

**As a** Người vận hành, **I want** xem trạng thái job backup/restore và kết quả từng thành phần, **so that** tôi biết dữ liệu đã an toàn hay cần xử lý phục hồi.

**Priority:** Should Have  
**Traces to:** FR6, FR6.1, FR6.3, FR6.4, NFR5

#### Acceptance criteria

- **AC3.1.1** — **Given** job queued/running, **When** operator xem lịch sử, **Then** UI hiển thị job ID, trạng thái, timestamp và tiến độ nếu đo được.
- **AC3.1.2** — **Given** job failed, **When** operator xem chi tiết, **Then** UI nêu bước/thành phần lỗi và trạng thái rollback/recovery-required, không hiển thị secrets hoặc dữ liệu thô.
- **AC3.1.3** — **Given** job bị dừng/crash giữa chừng, **When** hệ thống khôi phục trạng thái, **Then** job không được trình bày là succeeded và staging artifacts được dọn theo policy.
- **AC3.1.4** — **Given** operator xem job không thuộc phạm vi quyền, **When** họ truy cập bằng ID, **Then** hệ thống từ chối truy cập.

**Dependencies:** US1.2, US2.2.  
**INVEST:** Independent, Valuable, Estimable, Testable. Negotiable: mức chi tiết metrics và UX lịch sử.

### US3.2 — Audit các thao tác nhạy cảm

**As a** Chủ sở hữu hệ thống, **I want** các thao tác backup/restore được audit mà không ghi dữ liệu nhạy cảm, **so that** có thể truy vết người thực hiện và kết quả mà không làm rò rỉ thông tin.

**Priority:** Must Have  
**Traces to:** FR6.4, NFR1, NFR2

#### Acceptance criteria

- **AC3.2.1** — **Given** admin tạo backup, upload package, tải package hoặc xác nhận restore, **When** audit được ghi, **Then** actor, source/target label, job/package ID, digest, thời điểm và outcome được lưu.
- **AC3.2.2** — **Given** audit/log/metric được tạo, **When** payload kiểm tra, **Then** không có credential, service-role key, token, mật khẩu, object content hay raw database row.
- **AC3.2.3** — **Given** audit write thất bại, **When** thao tác nhạy cảm chưa bắt đầu, **Then** hệ thống từ chối thao tác hoặc thể hiện rõ chính sách audit failure; không âm thầm bỏ audit.

**Dependencies:** US1.2, US1.3, US2.2.  
**INVEST:** Independent, Valuable, Estimable, Testable.

## Traceability summary

Stories cover all functional requirements FR1–FR6 and their sub-requirements. NFR1–NFR7 require additional quality/design elaboration; NFR6 is reflected in the delivery/testing constraints and will be elaborated in downstream quality/build stages; NFR7 needs explicit accessible-UI verification in design/test planning.
