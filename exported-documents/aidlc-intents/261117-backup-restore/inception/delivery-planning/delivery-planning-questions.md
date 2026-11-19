# Câu hỏi lập kế hoạch delivery — Backup & Restore

## Q1. [Delivery] Chiến lược ưu tiên các phần xây dựng

Một Bolt là một lượt xây dựng qua các stage Construction cho một hay nhiều Unit, kết thúc bằng một kết quả có thể kiểm chứng. Unit đầu tiên đã được xác định là lát tích hợp localhost nhưng chỉ tạo backup database chưa đầy đủ. Bạn muốn ưu tiên thế nào giữa giá trị, rủi ro chưa biết và tích hợp end-to-end?

A. Skeleton-first cho U1, đồng thời đặt cổng dừng rõ ràng: kiểm chứng API/job/archive path nhưng không gọi đó là backup hoàn chỉnh; sau đó xử lý rủi ro Auth/Storage sớm trong U2
B. Risk-first: xây capability Auth/Storage và giới hạn archive trước, tích hợp end-to-end sau
C. Value-first: ưu tiên luồng backup ZIP đầy đủ trước các phần restore
D. Kết hợp: nêu rõ cách áp dụng cho từng Unit
X. Other (please specify)

[Answer]: A — Skeleton-first + risk

## Q2. [Delivery] Mô hình chấm điểm kinh tế

DAG chỉ xác định phụ thuộc U1 → U2 → U3 → U4; thứ tự Bolts cần cân nhắc giá trị/rủi ro/khối lượng riêng. Có nên dùng một mô hình chấm điểm định lượng không?

A. Không dùng điểm số; ghi rationale risk-first/walking-skeleton-first và giả thuyết cần kiểm chứng cho từng Bolt
B. Dùng WSJF (Weighted Shortest Job First: giá trị + khẩn cấp + giảm rủi ro chia cho effort); cùng xác nhận trọng số
C. Dùng bảng điểm định tính value/risk/effort, không quy đổi công thức
X. Other (please specify)

[Answer]: A — Không dùng điểm số; rationale và giả thuyết

## Q3. [Delivery] Kích thước một Bolt

Vì bốn Unit có DAG tuyến tính và mỗi Unit cần design/test gate, kích thước Bolt nào giúp review và xác minh rõ nhất?

A. Một Unit cho mỗi Bolt; không gộp các Unit phụ thuộc vào cùng một Bolt
B. Gộp nhiều Unit liên tiếp thành một Bolt lớn
C. Chia thành lát mỏng cắt ngang Unit boundaries
X. Other (please specify)

[Answer]: A — Một Unit mỗi Bolt

## Q4. [Delivery] Thực thi đồng thời hay tuần tự

Nhóm xác nhận chỉ thử nghiệm localhost và không có bước release riêng; DAG hiện không có Unit độc lập. Các Bolt nên thực thi thế nào?

A. Tuần tự; các Unit đã phụ thuộc tuyến tính và không có cơ hội parallel giữa Unit
B. Song song ngay khi có thể, dù cần phối hợp các dependency
X. Other (please specify)

[Answer]: A — Tuần tự

## Q5. [Delivery] Phụ thuộc bên ngoài

Các capability GoTrue, schema database, Storage và object store chưa được xác minh đầy đủ; không có bằng chứng về một team bên ngoài đang chịu trách nhiệm hay thời hạn cụ thể. Cách ghi nhận nào phù hợp?

A. Ghi chúng là cổng xác minh/cấu hình kỹ thuật trước khi dùng dữ liệu thật, chủ sở hữu là project/platform owner chưa chỉ định; không bịa ETA hoặc hand-off
B. Xem như đã được hỗ trợ và không tạo dependency
C. Đặt ETA tạm thời và mặc định một team owner
X. Other (please specify)

[Answer]: A — Cổng xác minh, owner/ETA chưa xác định

## Q6. [Delivery] Rủi ro cần xử lý sớm

Rủi ro nào nên điều khiển sequence sau lát tích hợp U1?

A. Xác minh khả năng export/restore GoTrue/Auth và chính sách password/session; nếu không hỗ trợ thì dừng claim backup hoàn chỉnh
B. Xác minh scope/version/cách dump database My-CMS
C. Xác minh tính toàn vẹn, paging, limits và metadata của Storage/object store/archive
D. Ưu tiên rủi ro khác (nêu cụ thể)
X. Other (please specify)

[Answer]: A, B, C — Xác minh Auth, database và Storage/archive

## Q7. [Delivery] DoD và giả thuyết của Bolt đầu tiên

U1 được đặt làm lát tích hợp mỏng, nhưng không được cung cấp như backup hoàn chỉnh. Điều kiện nào chứng minh lát này hữu ích mà không gây hiểu lầm?

A. Admin được phân quyền tạo job localhost; metadata/job bền vững đi qua API→worker→archive/checksum; kiểm thử chứng minh thiếu Auth/Storage khiến package không được đánh dấu complete/downloadable
B. Chỉ unit-test từng crate, chưa chạy flow tích hợp
C. Chỉ demo UI không cần API/worker thật
X. Other (please specify)

[Answer]: A — Tích hợp + fail-closed

## Q8. [Delivery] Nhóm sở hữu và verification command

Đây là phạm vi `classic`, team-formation không chạy; hướng dẫn yêu cầu Bolts do AI developer agent thực hiện. Construction có checkpoint và cần một lệnh kiểm tra thật, bao gồm test và coverage theo posture đã ghi nhận. Bạn muốn dùng lựa chọn nào?

A. Để delivery plan ghi `aidlc-developer-agent`; đề xuất xác minh bằng lệnh test/coverage thực tế sau khi kiểm tra scripts hiện có, rồi hỏi duyệt riêng tại checkpoint verification-command
B. Tự đặt một lệnh test chung ngay trong Delivery Planning mà không kiểm tra scripts
C. Chọn lệnh verify sau, nhưng bỏ coverage khỏi checkpoint
X. Other (please specify)

[Answer]: A — Kiểm tra scripts trước
