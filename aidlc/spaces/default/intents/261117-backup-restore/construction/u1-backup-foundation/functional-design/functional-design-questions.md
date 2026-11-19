# Câu hỏi Functional Design — u1-backup-foundation

## Q1. [Architect] Ranh giới nghiệp vụ của U1

U1 là lát localhost chạy qua API → job bền vững → worker → archive, nhưng US1.2 còn yêu cầu Auth và Storage để được xem là backup hoàn chỉnh. U1 nên xử lý thế nào khi mới thu được database?

A. Cho phép tạo artifact thử nghiệm/staging nội bộ có trạng thái `incomplete`; API/UI không cung cấp download như backup hoàn chỉnh, không chuyển job sang `succeeded`, và artifact bị dọn theo policy
B. Không tạo archive nào nếu chưa có Auth và Storage; demo chỉ chứng minh API → job → worker với outcome `failed/incomplete`
C. Cho phép `succeeded` cho database-only nhưng luôn gắn cảnh báo rằng chưa có Auth/Storage
X. Other (please specify)

[Answer]: Incomplete artifact — internal staging only; never complete/downloadable

## Q2. [Architect] Phân ranh AC của story dùng chung

Story US1.2 được gán cho U1 và U2 nhưng U1 chỉ là lát database-only; US3.2 cũng trải qua các thao tác nhiều Unit. Để Functional Design và traceability của U1 không nhận trách nhiệm toàn bộ AC ngoài phạm vi, cách nào phù hợp?

A. Chỉ thiết kế các AC U1 thật sự thực hiện; đánh dấu rõ các AC còn lại `Deferred` tới U2 hoặc các Unit sau, giữ nguyên story tổng thể
B. Thiết kế toàn bộ AC của US1.2 và US3.2 ngay trong U1 dù phần implementation nằm ở Unit sau
C. Thay đổi/tách story và DAG trước khi tiếp tục Functional Design
X. Other (please specify)

[Answer]: AC theo Unit; defer các AC còn lại cho Unit phù hợp

## Q3. [Architect] UI và người sở hữu trong U1

US1.1 yêu cầu chọn instance và kiểm tra quyền; mockup đặt các màn hình trong Admin UI hiện có. U1 mang kind `service`, nên Functional Design không tạo artifact riêng `frontend-components.md`. UI scope nào cần đặc tả trong `functional-spec.md` của U1?

A. U1 sở hữu phần UI tối thiểu cho instance selection, empty/error states và bắt đầu backup; đặc tả interaction/API states trong `functional-spec.md` và dùng mockup hiện có làm nguồn
B. U1 chỉ thiết kế backend; UI để một Unit sau phụ trách, dù DAG hiện không có UI Unit riêng
C. Dừng và đề nghị sửa kind/DAG để tạo một Unit `ui` trước khi xác định giao diện
X. Other (please specify)

[Answer]: U1 owns minimal UI and specifies its workflows

## Q4. [Architect] Cấu hình instance localhost

API không được nhận endpoint/credential tùy ý từ browser; danh sách cấu hình và owner/secret store vẫn chưa được chốt. U1 cần mô hình hóa nguồn instance thế nào?

A. Chỉ tham chiếu ID ổn định trong danh sách cấu hình server-side; nếu chưa cấu hình instance thì UI/API trả trạng thái rõ và không tạo job; chi tiết secret/config loader để thiết kế kỹ thuật sau
B. Nhận endpoint và credential từ admin qua form, chỉ dùng trong runtime hiện tại
C. Dùng một instance hard-coded duy nhất, không có ID hoặc cấu hình
X. Other (please specify)

[Answer]: Chỉ ID từ server-side config; chưa cấu hình thì không tạo job

## Q5. [Architect] Trạng thái và cạnh tranh job U1

U1 cần chứng minh job bền vững qua API/worker, trong khi lease/restart semantics chưa được quyết định đầy đủ. Trạng thái và xử lý duplicate nào nên được đặc tả ở lát đầu?

A. Job có ID bền vững, `queued → running → failed` cho lát chưa đủ thành phần; idempotency key chống double-submit; stale/unknown outcome không được suy thành `succeeded`; lease/restart recovery chi tiết được giữ là câu hỏi mở cho Functional Design/NFR tiếp theo
B. Chỉ dùng trạng thái trong bộ nhớ và không xử lý duplicate trong U1
C. Có thể đưa job lên `succeeded` trước khi worker hoàn tất vì đây chỉ là localhost
X. Other (please specify)

[Answer]: Job bền vững + idempotency; incomplete không success; lease/restart để sau
