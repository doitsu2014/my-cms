# Mockups — Backup & Restore

## Evidence and scope

Thiết kế mục tiêu dựa trên stories/requirements đã duyệt. Màn hình được đặt trong Admin UI hiện có; CodeKB chỉ xác nhận ở mức hỗn hợp nên vị trí điều hướng chính xác, component library và theme tokens cần được đối chiếu lại với `apps/web` khi triển khai UI. Đây là mô tả wireframe/text, không phải ảnh chụp implementation hiện hành.

## Information architecture

- Admin navigation: **Backup & Restore**.
- Trang có hai tab: **Tạo backup** và **Khôi phục**.
- Khu vực lịch sử job hiển thị backup/restore, với chi tiết từng job mở rộng theo hàng.
- Ngôn ngữ/thuật ngữ theo hệ thống localization hiện có trong admin.

## Screen A — Tạo backup

```text
┌──────────────────────────────────────────────────────────┐
│ Backup & Restore                                          │
│ [Tạo backup] [Khôi phục]                                  │
├──────────────────────────────────────────────────────────┤
│ Tạo backup                                                │
│ Instance nguồn *  [Chọn instance self-hosted       ▾]    │
│ Nhãn môi trường: [Local / Staging label]                  │
│ Phạm vi: My-CMS DB · GoTrue/Auth* · Storage objects       │
│ Nhất quán: Best-effort — các nguồn có thể lệch thời điểm  │
│                                                          │
│ [Tạo backup]                                              │
├──────────────────────────────────────────────────────────┤
│ Lịch sử job                                               │
│ Thời gian       Loại      Instance    Trạng thái   Tác vụ │
│ 06:30           Backup    Local       Đang chạy    [Xem]  │
│ 05:50           Backup    Local       Thành công   [Tải]  │
└──────────────────────────────────────────────────────────┘
```

- Không có instance: empty state nêu “Chưa có instance khả dụng” và action cấu hình theo quyền hiện có.
- Khi job chạy: trạng thái và tiến độ theo component nếu có số đo; không giả lập phần trăm nếu không đo được.
- Khi job thành công: hàng có nút tải package và metadata (ngày, dung lượng, checksum/trạng thái integrity).
- Khi thất bại: hiển thị thành phần lỗi và hành động an toàn; không hiển thị secrets/raw data.

## Screen B — Upload và preflight restore

```text
┌──────────────────────────────────────────────────────────┐
│ Backup & Restore                                          │
│ [Tạo backup] [Khôi phục]                                  │
├──────────────────────────────────────────────────────────┤
│ Khôi phục                                                 │
│ Package ZIP *    [Chọn tệp...]  backup-2026....zip        │
│ Instance đích *  [Chọn instance self-hosted        ▾]    │
│                                                          │
│ [Tải lên và kiểm tra]                                     │
├──────────────────────────────────────────────────────────┤
│ Preflight                                                 │
│ Package: backup-2026....zip  Integrity: Đạt               │
│ Nguồn: Local-A     Đích: Local-B     Tương thích: Đạt      │
│ Phạm vi: DB / Auth / Storage   Hành động: THAY THẾ         │
│ Warnings: Best-effort snapshot; Auth/session caveat       │
│                                                          │
│ [Tiếp tục đến xác nhận]                                   │
└──────────────────────────────────────────────────────────┘
```

- Upload hiển thị progress thực tế, tên file đã chọn, giới hạn kích thước được cấu hình và lỗi theo field.
- Preflight phân biệt đạt/không đạt; nếu không đạt, khóa tiếp tục và nêu lý do có thể hành động.
- Không có target tương thích thì UI giữ package không áp dụng và giải thích nguyên nhân.

## Screen C — Xác nhận restore (destructive checkpoint)

```text
┌──────────────────────────────────────────────────────────┐
│ Xác nhận khôi phục                                        │
│                                                          │
│ Target: Local-B (self-hosted)                             │
│ Package: backup-2026....zip                               │
│ Hành động: THAY THẾ dữ liệu thuộc phạm vi hỗ trợ          │
│ Sẽ thay đổi: My-CMS DB / Auth / Storage objects           │
│ Mức nhất quán: Best-effort                                │
│                                                          │
│ Nhập tên target để xác nhận: [________________]           │
│ [Quay lại]                         [Bắt đầu khôi phục]    │
└──────────────────────────────────────────────────────────┘
```

Nút restore chỉ enabled sau khi target name khớp chính xác và preflight cùng recovery-point readiness đạt. Hủy/quay lại không sửa target. Production restore không hiển thị/khả dụng trong phiên bản đầu.

## Screen D — Job detail

```text
┌──────────────────────────────────────────────────────────┐
│ Job R-1042 · Khôi phục · Đang chạy                        │
│ Target: Local-B   Package: backup-2026....zip             │
│                                                          │
│ Preflight       Đạt                                       │
│ Recovery point  Đang chuẩn bị                             │
│ Database        Chờ                                       │
│ Auth            Chờ                                       │
│ Storage         Chờ                                       │
│ Validation      Chờ                                       │
│                                                          │
│ Kết quả cuối: Chưa có                                     │
└──────────────────────────────────────────────────────────┘
```

Thất bại: gắn trạng thái `Rollback completed` hoặc `Recovery required`; mô tả component, thời gian và bước tiếp theo. Thành công chỉ hiển thị sau post-restore validation.

## Responsive and states

- Desktop/tablet: hai tab giữ nguyên; lịch sử có bảng và expandable job details.
- Mobile: bảng chuyển thành cards với thứ tự trường rõ; action chính full-width; không yêu cầu scroll ngang.
- Loading, empty, error, success, warning và partial/recovery-required là trạng thái riêng biệt.
