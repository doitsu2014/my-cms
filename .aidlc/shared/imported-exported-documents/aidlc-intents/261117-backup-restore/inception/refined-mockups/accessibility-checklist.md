# Accessibility Checklist — Backup & Restore

## Target

WCAG 2.1 AA cho các thao tác cốt lõi, theo lựa chọn của người dùng. Đây là checklist yêu cầu thiết kế; cần xác minh bằng kiểm thử accessibility ở giai đoạn implementation.

## Keyboard and focus

- [ ] Tab order đi theo thứ tự đọc: tabs → form → primary action → history/details.
- [ ] Mọi tab, select, file input, detail disclosure, confirmation và action đều thao tác được bằng bàn phím.
- [ ] Focus indicator rõ và không bị che; khi tab chuyển, focus không bị mất bất ngờ.
- [ ] Dialog/confirmation giữ focus trong lúc mở, đóng bằng thao tác phù hợp và trả focus về control gọi dialog.
- [ ] Khi upload/job kết thúc, focus không bị tự ý giật; trạng thái mới được thông báo.

## Screen reader and semantics

- [ ] Dùng heading hierarchy, labels và descriptions có quan hệ ngữ nghĩa.
- [ ] Tên instance, tên file, trạng thái job và lỗi được đọc cùng ngữ cảnh.
- [ ] Upload progress và job status được công bố như status updates không gây thông báo lặp quá mức.
- [ ] Bảng lịch sử có header/caption; responsive cards vẫn giữ tên trường và giá trị rõ.
- [ ] Lỗi validation được gắn với field và có mô tả cách khắc phục.
- [ ] Xác nhận restore nêu tên target, nguồn package và hành động thay thế trong accessible name/description.

## Visual and responsive

- [ ] Contrast đạt WCAG 2.1 AA cho text và UI controls theo loại nội dung.
- [ ] Trạng thái không chỉ dựa vào màu: có text và/hoặc icon/shape.
- [ ] Nội dung reflow tại viewport hẹp, zoom và text scaling không làm mất thao tác; không buộc scroll ngang trang.
- [ ] Focus, error, warning, success và disabled states phân biệt được cả trong theme hiện có.
- [ ] Không truyền đạt progress chỉ qua animation; hỗ trợ reduced motion nếu animation được dùng.

## Destructive action and safety

- [ ] Confirmation cảnh báo rõ dữ liệu target nào sẽ bị thay thế.
- [ ] Người dùng có thể hủy/quay lại bằng bàn phím mà không thay đổi target.
- [ ] Nút restore không chỉ phân biệt bằng màu; tên thao tác cụ thể và target được nêu.
- [ ] Lỗi/recovery-required có thể được đọc và hiểu mà không cần nhận diện màu sắc.
