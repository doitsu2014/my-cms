# Bằng chứng và mức độ chắc chắn

## Phạm vi

Đây là tổng hợp cho intent `261117-backup-restore`, scope `classic`, loại dự án brownfield. CodeKB của lần reverse engineering được đánh dấu phân tích có độ sâu hỗn hợp; các quan sát không đại diện cho rà soát toàn diện. Không chạy build, test, linter, triển khai hay chương trình dự án trong bước này. `Collaborators: off`; không có tệp đóng góp độc lập.

## Nguồn đã kiểm tra

| Nguồn | Quan sát có thể hỗ trợ | Giới hạn |
|---|---|---|
| `aidlc/spaces/default/memory/{org,team,project}.md` | `team.md` và các phần thực hành liên quan trong `project.md` chưa có nội dung đã khẳng định. `org.md` có mặc định về làm việc, kiểm thử, triển khai và phong cách mã. | Mặc định tổ chức không chứng minh lựa chọn hay hành vi của nhóm. |
| `aidlc/spaces/default/intents/261117-backup-restore/aidlc-state.md` | Scope `classic`; Test Strategy `Standard`; cấu hình các tùy chọn workflow. | Cấu hình workflow không xác nhận thực hành của nhóm. |
| CodeKB: `code-structure.md`, `technology-stack.md`, `dependencies.md`, `code-quality-assessment.md` | Nhận diện workspace Rust/Cargo, frontend React/TypeScript/pnpm, vùng kiểm thử, ESLint/Prettier và workflow CI. | CodeKB ghi nhận độ sâu hỗn hợp/không đầy đủ; có khu vực frontend và deployment chỉ được khảo sát nông. Không phải rà soát toàn diện và không có kiểm tra thực thi trong bước này. |
| CodeKB: `architecture.md`, `business-overview.md` | Mô tả mục tiêu backup/restore và rủi ro dữ liệu nhạy cảm; tính năng chưa phải luồng sản phẩm hoàn chỉnh. | Mô tả mục tiêu không chứng minh hệ thống đang vận hành hay quy trình của nhóm. |
| `reverse-engineering-timestamp.md`, `developer-scan.md` | Phạm vi scan được ghi là `partial`. | Không suy rộng từ các vùng chưa đọc sâu. |
| `.github/workflows/ci-my-cms.yml` | Có cấu hình PR nhắm `main`, Cargo tests và báo cáo coverage. | Tệp ghi rõ workflow tạm thời bị vô hiệu hóa; chưa xác minh CI thực sự chạy hay đang chặn merge. Đây là khoảng trống vận hành so với cổng CI mà nhóm xác nhận. |
| Git history, checkout và workflow phát hành | Có commit gần đây trên `main`, một số workflow phát hành ảnh; checkout chứa thay đổi chưa commit thuộc người dùng. | Lịch sử không đủ để suy ra chính sách nhánh/merge; không sửa các thay đổi đang có. |
| `deployments/` và tài liệu triển khai | Có tài liệu/cấu hình liên quan Docker Swarm, Kubernetes và phát hành image. | Kiểm kê không xác nhận môi trường đang dùng, lịch phát hành hay triển khai thực tế. |
| `practices-discovery-questions.md` — câu trả lời con người | Q1: nhánh mới và PR cho mọi thay đổi. Q2: lát cắt đầu-cuối mỏng trước. Q3: `test-after`, mục tiêu 80% và CI xanh trước merge. Q4: chỉ localhost, không bước deploy/release riêng; phần làm rõ nói đây là thực hành thường trực cho mọi thay đổi. Q5: dùng formatter/linter hiện có, không thêm quy tắc. | Đây là quyết định thực hành do con người cung cấp; không biến chúng thành bằng chứng rằng repository hiện đang thực thi chúng. Không nêu kiểu merge, rebase/squash hay thời hạn nhánh. |

## Thực hành đã được xác nhận

- **Cách làm việc (Q1)**: Nhánh mới cho công việc và pull request cho mọi thay đổi. Không có câu trả lời về kiểu merge, rebase/squash hoặc tuổi thọ nhánh; không suy diễn.
- **Lát cắt đầu-cuối (Q2)**: Có, xây một lát cắt mỏng chạy đầu-cuối trước. Phạm vi khẳng định là backup/restore; chưa có căn cứ mở rộng thành quy tắc chung.
- **Kiểm thử (Q3)**: `test-after`, triển khai từng lớp có thể kiểm thử rồi viết/chạy kiểm thử lớp đó; mục tiêu coverage 80% và CI xanh trước khi hợp nhất. CI hiện bị ghi tạm vô hiệu hóa, nên điều kiện vận hành chưa đáp ứng/được xác minh. Đây là khoảng trống cần xử lý, không phải lý do xóa lựa chọn của nhóm.
- **Triển khai (Q4 và làm rõ)**: Chỉ kiểm thử localhost, không bước deploy/release riêng; nhóm xác nhận đây là thực hành thường trực cho mọi thay đổi. Điều này khác mặc định tổ chức deploy-on-merge lên staging. Cần xác minh cơ chế promotion để xử lý khác biệt; không âm thầm thay câu trả lời bằng mặc định tổ chức.
- **Phong cách mã (Q5)**: Dùng formatter/linter hiện có, không thêm quy tắc.

## Bất định còn lại

- Q1 không xác định kiểu hợp nhất, rebase/squash hay thời hạn nhánh; không ghi thành chính sách.
- Chưa rõ cơ chế CI hoạt động thay thế nào, nếu có, khi workflow trong repo đang bị vô hiệu hóa.
- Chưa xác minh cơ chế promotion, môi trường hoặc cách thực hành localhost-only tương tác với mặc định staging deploy-on-merge của tổ chức. Đây là khác biệt chính sách cần được xử lý qua cơ chế promotion, không phải suy luận rằng nhóm đã chọn staging.
- Độ sâu khảo sát CodeKB hỗn hợp và các kiểm tra thực thi không được chạy; các nhận định về cấu hình không khẳng định trạng thái runtime.
