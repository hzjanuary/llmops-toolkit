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
- SQLite giữ nguyên bảng/dữ liệu cũ; migration bổ sung phiên bản 1 và 2, giữ receipt phiên bản 1; vẫn hiển thị
  timestamp/latency và lưu correlation ID hợp lệ, không chứa credential. Ghi receipt
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

Mã nguồn đã kiểm thử cục bộ: `5f3a2d5b57465dafdf1b18c105783b537e30f601`.
`python3 scripts/verify.py --output /tmp/ragops-spec001-retention-evidence` thành công;
28 test thông thường PASS và test setup được gọi riêng PASS; build sạch từ source
archive, Clippy, license metadata và secret/dependency gates PASS. Evidence giữ
nguyên lệnh, exit code và hash từng log. Trạng thái CI/PR và HEAD được cập nhật sau
khi chạy thật trong hồ sơ nghiệm thu. Run GitHub đầu tiên
[41272218134](https://github.com/hzjanuary/ragops-harness/actions/runs/41272218134)
đã PASS cả ba job ở SHA 794f749; artifact ZIP và hash log bên trong đã được tải
về kiểm chứng. Run này có trước bản giữ latency/correlation và migration 2.
HEAD cuối cùng và CI tương ứng nằm trong mô tả
[Draft PR #1](https://github.com/hzjanuary/ragops-harness/pull/1); không gán kết quả
run cũ cho HEAD mới.
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

## Bằng chứng GitHub cho bản triển khai đầy đủ

[Run 41272626554](https://github.com/hzjanuary/ragops-harness/actions/runs/41272626554)
PASS cả offline-source, security và verified-source tại
`f253ddda41d6a06e116e67ba288145ae3b646e5d`, gồm migration 2 và regression chống
credential echo trong metadata. Hai ZIP artifact đã tải về, đối chiếu digest,
source SHA, 28 test + helper và log; history/tree không có secret findings và hai
lockfile không có vulnerability/unsoundness warnings. Snapshot bất biến:
[evidence/github-run-41272626554.json](evidence/github-run-41272626554.json).
Commit bổ sung chỉ cập nhật tài liệu/evidence; HEAD cuối và run tương ứng được
ràng buộc trong mô tả Draft PR, không tự gán kết quả cũ cho commit mới.

## Cập nhật theo tuyên bố chủ sở hữu — Phase A của OSS-001

Chủ sở hữu đã xác nhận quyền đối với phần RAGOps-specific và làm rõ Harness là
môi trường workflow dùng để phát triển. NOTICE đã bỏ suy diễn quyền sở hữu từ
URL/username; audit từng file và tuyên bố nằm trong docs/legal/. Không khẳng định
mọi câu chữ workflow lịch sử đều được viết độc lập khi Git không chứng minh điều đó.

Đã kiểm tra 146 file tại HEAD 782dfab; giữ toàn bộ lịch sử/evidence/migration.
Crate Hello World chưa xóa vì CI và inventory đang tham chiếu; việc xóa cần thay
đổi đồng bộ trong commit riêng. AGENT_REQUIREMENT.md đã bị xóa trước lượt làm
việc này; trạng thái đó được giữ nguyên, không tự đưa vào commit kiểm toán.

Chưa có nguyên văn tên chủ thể bản quyền công khai. LICENSE.proposed vẫn giữ,
LICENSE chưa được cài và S30 vẫn BLOCKED (36 PASS / 1 BLOCKED / 1 NOT_RUN).
Chưa merge, tag hay phát hành; Phase D chưa bắt đầu. Kết quả CI cũ chỉ áp dụng
cho SHA cũ. Cần xác nhận dòng copyright theo mục A3 trước khi tiến tới các gate
phát hành. Không có inference thật, trả phí hay scan ngoài.
