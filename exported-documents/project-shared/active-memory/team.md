<!-- AI-DLC keeps this copy in step with aidlc/spaces/default/memory/team.md. Edit that file; this copy is replaced. -->
# Team-Level Rules

> This team's affirmed practices and corrections. Loaded after `org.md` as
> strict-additive guidance; contradictions with broader policy are rejected.
> Populated by the practices-discovery affirmation gate. Edit at the gate,
> not directly.

## Way of Working

- Nhóm tạo nhánh mới để thực hiện công việc và dùng pull request cho mọi thay đổi.
- Câu trả lời không quy định kiểu hợp nhất, thời hạn nhánh hay quy trình rebase/squash; không suy diễn các chính sách này từ bằng chứng về `main`.

## Walking Skeleton

- Với phạm vi backup/restore này, nhóm muốn xây một lát cắt mỏng chạy đầu-cuối trước để chứng minh các phần kết nối được.
- Đây là quyết định được xác nhận cho backup/restore; chưa có bằng chứng để khái quát thành thông lệ cho mọi thay đổi.

## Testing Posture

- **Methodology**: test-after
- **Methodology evidence**: Nhóm xác nhận dùng `test-after` cho thay đổi này; workflow CI hiện được ghi rõ là tạm vô hiệu hóa, nên trạng thái vận hành hiện không chứng minh được cổng CI.
- **Ordering**: Triển khai từng lớp có thể kiểm thử, sau đó viết và chạy kiểm thử cho chính lớp đó.
- Nhóm đặt mục tiêu coverage 80% và yêu cầu CI xanh trước khi hợp nhất.
- Scope `classic` cũng có mặc định tổ chức về coverage 80% và CI trước merge; câu trả lời của nhóm xác nhận các mục tiêu này.
- Khoảng trống vận hành: cấu hình `.github/workflows/ci-my-cms.yml` có khai báo test/coverage nhưng đang tạm vô hiệu hóa; cần khôi phục hoặc xác minh cơ chế CI để cổng đã chọn có thể thực thi.

## Guard Policy

<!-- Affirmed by the team. Mode: strict, relaxed, or off. Strict here holds for every intent, whatever one person asks in chat; relaxed or off here applies to every intent whose policy came from its scope, unless project.md or the person sets another; changing this line changes it. A section under the retired Change Control heading, written by an earlier release, is still read. -->

## Deployment

- Nhóm chỉ kiểm thử trên localhost và không có bước triển khai/phát hành riêng; nhóm xác nhận đây là thực hành thường trực áp dụng cho mọi thay đổi.
- Điều này khác với mặc định tổ chức là triển khai staging khi hợp nhất. Cần xác minh cơ chế promotion/triển khai của môi trường để xử lý rõ khác biệt này; không thay câu trả lời của nhóm bằng mặc định tổ chức.

## Code Style

- Dùng cấu hình formatter/linter hiện có; không bổ sung quy tắc riêng.
## Forbidden

<!-- Team-specific forbidden patterns -->

## Mandated

<!-- Team-specific mandates -->

## Corrections

<!-- Self-learning loop appends here. -->
