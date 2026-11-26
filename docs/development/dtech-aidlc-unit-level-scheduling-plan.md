# Plan: dtech-aidlc điều phối theo work unit, tự park unit bị blocked

- **Áp dụng cho:** dtech-aidlc 1.0.0 (binary Rust cài qua cargo; source không nằm trong repo này)
- **Nguồn bằng chứng:** lịch sử report của intent `use-dtech-aidlc-to-develop-the-backup-and-restor` (33 report ở `red-tests`, 16 ở `build-and-verify`, 6 ở `implementation-design`), kết quả `dtech-aidlc doctor` và `orchestrate next` ngày 2026-11-26
- **Giới hạn:** plan này suy ra từ hành vi quan sát được, chưa đọc source của CLI. Các điểm "cách CLI làm hiện nay" cần được xác nhận trên source trước khi thiết kế chi tiết.

## 1. Vấn đề gốc

| # | Hành vi hiện tại | Hậu quả đã thấy |
|---|---|---|
| A | Bộ chọn chạy theo stage rồi mới đến unit. Một unit blocked giữ `currentStage` đứng yên. | WU-06 blocked khiến cả intent đứng. `doctor` báo "currentStage build-and-verify is not the first unfinished stage". |
| B | `orchestrate report` chỉ có `--stage`. Unit hiện hành là ngầm định. | Không park hay skip chính xác một unit được. |
| C | `--needs` chỉ dành cho prerequisite môi trường như `docker`. Không có lý do blocked kiểu "chờ unit khác" hay "test của unit khác đang đỏ". | WU-06 blocked khoảng 20 lần vì chờ WU-16. Phải đảo `dependsOn` thành `WU-06 -> WU-16` để lách. |
| D | Regression là toàn workspace và tính cho từng unit. | 13 test (workspace thường) và 3 test (workspace ignored) của WU-16 chặn gate của WU-06. Lỗi của unit khác chặn unit này. |
| E | `intent rework` đưa cả intent về `implementation-design`; sau đó `red-tests` và `build-and-verify` reset mọi unit về `pending`. | WU-01 đến WU-05 đã green vẫn bị replay, ra khoảng 10 report "skip redundant". |
| F | Việc nạp `work-units.json` có thể rơi về placeholder 1 unit. | Report 2026-11-25 07:16 ghi `designReworkPending=true` và mảng 1 unit thay cho 16 unit. |
| G | Không có đếm lần thử theo từng unit và không phát hiện cùng blocker lặp lại. | `attempt: 17` mà vẫn tiếp tục chọn cùng một stage. |

## 2. Mục tiêu

Khi một unit bị blocked vì lý do mà retry không giải quyết được, CLI **park** unit đó và tự chọn unit kế tiếp không bị blocked. Khi mọi unit làm được đã xong, CLI quay lại chạy các unit đã park. Rework chỉ động tới đúng unit có thiết kế sai.

Ngoài phạm vi: thay đổi nội dung methodology của từng stage, hoặc thay đổi cách duyệt gate bởi người.

## 3. Thiết kế

### 3.1 State máy theo từng unit (thay cho stage-major)

- Mỗi unit có trạng thái: `pending → red → green → verified → done`, cộng `parked` và `blocked`.
- Unit parked lưu `kind`, `blockedBy` (unit, test id hoặc prerequisite), `signature` (băm của lý do) và `attempts`.
- Bộ chọn lấy unit đầu tiên, theo thứ tự dependency, mà mọi `dependsOn` đã `done` và bản thân không parked.
- Stage trở thành một bước trong vòng đời của unit. `currentStage` ở mức intent vẫn giữ cho gate và `doctor`, nhưng được tính từ trạng thái các unit.

### 3.2 Phân loại blocked, mỗi loại một cách xử lý

| Kind | Ví dụ | Xử lý |
|---|---|---|
| `env-prerequisite` | Docker không chạy | Giữ như hiện nay (`--needs`) |
| `dep-unit` | Chờ WU-16 | Park. Tự đưa lại hàng đợi khi `blockedBy` đạt `done` |
| `foreign-regression` | Test của WU-16 đang đỏ | Park phần regression của unit, xem 3.3 |
| `owner-decision` | CFG-17/18, RK-16 | Park. Liệt kê trong báo cáo cuối cho owner |
| `design-defect` | Test không thể red có ý nghĩa | Chỉ loại này mới cần `intent rework --unit` |

### 3.3 Regression có quy kết (giải quyết D)

- Dùng `ids.json` và bảng "work unit to test mapping" trong `work-units.md` để gán mỗi test thất bại cho một unit sở hữu.
- Gate của một unit: không có thất bại mới ngoài baseline thuộc unit này hoặc không có chủ. Thất bại thuộc unit khác được ghi vào `deferred-regressions` và không chặn unit này.
- Chạy một lần regression nghiêm ngặt toàn workspace ở sweep cuối, trước gate `release-candidate`. Thất bại ở sweep này mở lại đúng unit sở hữu nó.
- Test không tìm ra chủ được đánh dấu `unowned`: không chặn từng unit nhưng chặn ở sweep cuối.

### 3.4 Lệnh và cờ mới

| Lệnh | Mục đích |
|---|---|
| `orchestrate report --unit WU-xx ...` | Mọi kết quả gắn với đúng unit |
| `orchestrate park --unit WU-06 --kind dep-unit --blocked-by WU-16 --detail "..."` | Park một unit kèm lý do có cấu trúc |
| `orchestrate queue` | Liệt kê hàng đợi: ready, parked (kèm lý do) và done |
| `orchestrate revisit [--unit WU-xx]` | Đưa unit parked về hàng đợi. Tự chạy khi blocker đã `done` |
| Tự park theo signature | Cùng `signature` lặp quá N lần (mặc định 3) thì park và báo cáo, không retry tiếp (giải quyết G) |

### 3.5 Rework theo phạm vi (giải quyết E, F)

- `intent rework --unit WU-06[,WU-16]` chỉ sửa thiết kế của các unit đó.
- Mỗi unit lưu `inputFingerprint`: băm phần thiết kế, test và code nó phụ thuộc. Rework chỉ vô hiệu hoá unit có fingerprint đổi và các unit phụ thuộc trực tiếp. Unit không đổi giữ `done`, không cần report skip.
- `intent split WU-06 --into ...` để chia unit mà không phải rework cả thiết kế.
- Khi nạp `work-units.json`, nếu nạp lỗi hoặc gặp placeholder thì báo lỗi rõ ràng, không rơi về mảng 1 unit.

### 3.6 Cảnh báo sớm trong `doctor`

- Phát hiện dependency bị đảo, ví dụ `WU-06 dependsOn WU-16` trong khi code của WU-06 đã tồn tại.
- Phát hiện unit mà test đã có trong cây nhưng chưa gán cho unit nào đã xong.
- Cảnh báo ">6 unit" chuyển thành gợi ý chia thành đợt (wave), bên cạnh gợi ý tách intent.

### 3.7 Gate

- `release-candidate` chỉ mở khi: hàng đợi rỗng, không còn unit parked, và sweep regression nghiêm ngặt xanh so với baseline.
- Báo cáo cuối gom các `owner-decision` còn mở và `deferred-regressions`.

## 4. Lộ trình

| Giai đoạn | Nội dung | Tiêu chí hoàn thành |
|---|---|---|
| M1 | State theo unit, `--unit` trong `report`, migration state từ 1.0.0 | State cũ đọc được. `status` hiển thị đúng trạng thái từng unit |
| M2 | Phân loại blocked, `park`, `queue`, `revisit`, tự park theo signature | Mô phỏng WU-06 chờ WU-16 thì CLI tự chuyển sang WU-07, WU-11 |
| M3 | Quy kết regression, `deferred-regressions`, sweep cuối | 13 + 3 lỗi của WU-16 không còn chặn WU-06 |
| M4 | `intent rework --unit`, fingerprint, `split`, kiểm tra nạp `work-units.json` | Rework một unit không reset unit đã green |
| M5 | Cập nhật methodology (`develop.md`, `SKILL.md`), `doctor`, tài liệu | Các kịch bản ở mục 5 chạy end-to-end |

## 5. Kiểm thử và kịch bản chấp nhận

- **Replay kịch bản thật:** WU-05 xong, WU-16 chưa có, WU-06 chạy với test lifecycle đỏ. Kỳ vọng: WU-06 chuyển `parked(foreign-regression)`, CLI chọn WU-07, rồi WU-16, rồi tự mở lại WU-06.
- **Rework một unit:** WU-01 đến WU-05 giữ `done` và không sinh report skip.
- **Lặp cùng blocker:** lần thứ 3 tự park thay vì tăng `attempt`.
- **Tương thích:** `state.json` cũ nâng cấp được và không mất lịch sử audit.
- **Chỉ số:** số report trên mỗi unit và tỷ lệ report "skip redundant". Intent hiện tại có 33 report `red-tests` và 16 report `build-and-verify` trong khi mới chạy tới unit thứ 6 trong 16 unit.

## 6. Rủi ro và câu hỏi mở

- **Quy kết test sai chủ:** mặc định không chặn nhưng đánh dấu `unowned`, và chặn ở sweep cuối.
- **Unit ready nhưng phụ thuộc ngầm vào unit parked:** bắt buộc khai `dependsOn`. `doctor` cảnh báo khi code import chéo giữa các unit.
- **Unit parked bị quên:** `queue` luôn hiển thị parked và gate chặn nếu còn parked.
- **Cần xác nhận:**
  1. Vị trí repo source Rust của dtech-aidlc.
  2. Có giữ chế độ stage-major cũ làm lựa chọn `--strict-order` hay không.
  3. Ngưỡng N cho tự park (mặc định đề xuất là 3).

## 7. Việc có thể làm ngay cho intent hiện tại (không cần tool mới)

CLI hiện chọn `red-tests` cho WU-01 sau lần rework gần nhất, trong khi WU-01 đến WU-05 đã green từ ngày 22 đến 24/11. Cách đi tiếp trong khuôn khổ CLI 1.0.0:

1. Skip lần lượt WU-01 đến WU-05 ở `red-tests` và `build-and-verify`, mỗi lần kèm lý do cụ thể (khoảng 10 report).
2. Làm WU-16 qua red rồi green.
3. Chạy lại regression nghiêm ngặt cho WU-06.
