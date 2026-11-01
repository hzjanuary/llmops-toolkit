# Bàn giao kỹ thuật SPEC-001

Mục tiêu: giữ tính năng hữu ích của prototype Rust, tăng an toàn mặc định và khả
năng kiểm chứng; không biến dự án thành RAG engine, gateway tổng quát hay hệ điều
hành đa tác tử. Baseline cục bộ đã xác minh:
`d73849824cdf4eddaeb7898d375818f259450dec`.

## Thay đổi đã triển khai

- Proxy mặc định loopback, chưa cho phép gọi provider. Muốn chuyển tiếp phải bật
  rõ ràng, dùng token chủ sở hữu tách biệt API key, chọn model chính xác và chính
  sách giá. Giới hạn nội dung, thời gian, số yêu cầu, đồng thời và token đầu ra;
  lỗi provider được che nội dung. Không hỗ trợ streaming.
- SQLite giữ nguyên bảng/dữ liệu cũ; migration bổ sung phiên bản 1. Ghi receipt
  pending trước khi gửi; lỗi/mất usage/không biết giá không được giả thành chi phí
  bằng 0. Chi phí là ước tính theo cấu hình có nguồn/ngày/hash, không phải hóa đơn.
  Ngân sách chỉ theo tiến trình; khởi động lại làm reset. Cần hạn mức tài khoản
  provider độc lập, đặc biệt khi cho phép giá chưa biết.
- Eval mặc định heuristic so sánh từ, JSON tái lập; không gọi đó là faithfulness
  chuẩn. Judge thật yêu cầu đồng ý riêng tư, cấu hình provider/model/giá được duyệt
  và hạn mức rõ ràng; chỉ kiểm tra adapter bằng mock cục bộ.
- Scanner mặc định fixture offline. Kết quả chỉ là dấu hiệu từ chối, không kết
  luận an toàn/lỗ hổng. Live phải xác nhận quyền với đúng URL, IP literal, hạn mức,
  không redirect/DNS/metadata endpoint. Không scan mục tiêu ngoài.
- TUI nhận biết TTY, phục hồi khi SIGINT/SIGTERM/lỗi có thể xử lý, giám sát và chờ
  server dừng. Headless không phát escape sequence. SIGKILL/mất điện không thể bảo
  đảm phục hồi terminal.
- Tài liệu, threat model, vận hành, bảo mật, đóng góp, NOTICE và MIT proposal đã
  chuẩn bị. CI chỉ kiểm tra mã nguồn, không phát hành binary/package/image.

## Bằng chứng và giới hạn

Xem [ma trận nghiệm thu](SPEC-001-acceptance.md), [kiểm toán baseline](SPEC-001-baseline.md)
và evidence có source SHA, câu lệnh, exit code, SHA-256. Không đổi FAIL thành PASS:
baseline lỗi build do thiếu cache/OpenSSL và có advisory; bản sửa dùng Rust TLS,
cập nhật dependency và giữ lại lịch sử kết quả. Script kiểm thử tạo DB tổng hợp
mới trong thư mục tạm; không chạm DB/credentials/session của operator. File spec
untracked có sẵn được giữ nguyên và không tự ý đưa vào commit.

Trạng thái CI/PR và HEAD được cập nhật sau khi chạy thật trong hồ sơ nghiệm thu.
Không có suy luận LLM thật, cuộc gọi trả phí, scan mục tiêu bên ngoài hay tải model.
Mock PASS không chứng minh tương thích live hoặc chứng nhận bảo mật độc lập.

## Việc chủ sở hữu cần quyết định trước source-only v0.1.0

1. Xác minh quyền sở hữu mã/tài liệu harness kế thừa; duyệt danh tính bản quyền và
   MIT proposal. Hiện chưa có license grant hợp lệ được kích hoạt.
2. Review Draft PR/HEAD và artifacts CI thực tế, bảo vệ workflow/checks khỏi thay
   đổi làm yếu gate; cân nhắc đánh giá bảo mật độc lập.
3. Quyết định riêng ba ruleset cho main: chống xóa/force update, bắt buộc PR,
   bắt buộc verified-source. Các JSON chỉ là proposal disabled, chưa áp dụng.
4. Quyết định riêng việc bỏ crate Hello World rag-harness/ và tài liệu tác vụ cũ;
   lần này giữ nguyên cấu trúc và lịch sử, chỉ cập nhật lockfile có lỗ hổng.
5. Lưu evidence dài hạn trước khi artifacts GitHub hết retention. Chỉ cân nhắc
   source preview sau khi các blocker được giải quyết; chưa có quyền tạo tag,
   release, merge, deploy hay publish artifact thực thi.

Kết luận hiện tại: có thể review thay đổi mã nguồn trên Draft PR sau khi gate
thật thành công; **chưa sẵn sàng phát hành OSS** do quyền sở hữu/license và các
quyết định chủ sở hữu còn mở. Không yêu cầu thực hiện live test trong tác vụ này.
