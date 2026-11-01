**LLMOps Toolkit — Fully Rebranded, Independent Rust OSS Repository.**

Phạm vi: branch/PR của REBRAND-001, chưa merge vào main. Không phát hành mới.

1. **Danh tính kho.** hzjanuary/llmops-toolkit, repository ID 1245373914, default
   main. About/topics do chủ sở hữu duyệt đã khớp nên không sửa. URL kho cũ vẫn
   resolve cùng ID. Baseline main a3df205abc5d9bd9d665f0a9072032bbd2145978.
2. **Cargo/CLI/TUI.** Package và executable llmops-toolkit, mô tả Cargo đúng chuỗi
   owner yêu cầu. Help/health/user-agent dùng identity mới; heading chính xác
   LLMOps Toolkit local FinOps monitor. Cargo tái tạo lock đúng; chỉ record root
   đổi tên/vị trí, toàn bộ 249 record dependency registry giữ nguyên.
3. **Biến môi trường.** LLMOPS_DB_PATH/PROXY_TOKEN/SCAN_TOKEN là tên chuẩn; alias
   RAGOPS_* cũ deprecated vẫn hoạt động. Tên mới ưu tiên khi có, cặp bằng nhau
   được chấp nhận, cặp xung đột fail closed trước I/O kể cả --db. Không in values,
   không nạp .env. OPENAI_API_KEY giữ nguyên. 4 unit test resolver độc lập process
   environment cùng smoke subprocess kiểm new/legacy/equal/conflict/missing/
   invalid/non-Unicode/redaction; không sửa environment thật của operator.
4. **SQLite.** Default ragops_harness.sqlite3 cố ý giữ nguyên; --db override env
   khi không xung đột. Không move/rename hoặc đổi schema/rows/timestamps/receipts.
   src/db.rs và migrations không đổi. Smoke tạo snapshot synthetic dưới tên cũ,
   so JSON/bytes trước/sau, kiểm .env không nạp và không tạo default tên mới.
   Input docs/specs/ và ignored operator/evidence cũ không stage/ghi đè/xóa.
5. **Các tệp nghỉ dùng.** Đúng 17 tệp trong bảng dưới. Với mỗi tệp,
   docs/legal/RETIREMENT_RECORD.json có original path, Git blob SHA, SHA-256,
   source commit, actual removal commit và recoverable_from_git_history=true.
   Source snapshot chung a3df205abc5d9bd9d665f0a9072032bbd2145978; removal commit
   2920e526f1e4870637ec45f434c1724fc4b22beb. Không rewrite history.
6. **Tool giữ lại.** verify.py: toàn bộ local gates/SHA-log; runtime_smoke.py:
   synthetic loopback/SQLite/PTY/headless, signals/backup/alias/DB/redaction;
   verify_public_archives.py: exact ZIP/tar content/mode/path + negative cases;
   source_gate.py: Git archive/private paths/blob/mode/reproducibility;
   license_inventory.py: MIT và toàn bộ dependency SPDX/checksum;
   install_security_tools.py: Gitleaks/cargo-audit pinned bên ngoài checkout.
   Cả sáu vẫn được CI dùng, nay Git mode100755. Thêm rebrand_gate.py kiểm identity,
   retirement/history/protected files/links, cũng executable. mock_script.py là
   wrapper eval offline do project sở hữu, được giữ và smoke so output với eval.
7. **Tài liệu/pháp lý.** Current README/SECURITY/CONTRIBUTING/NOTICE/AGENTS,
   architecture/product/self-hosting/threat model/index/test matrix đã dùng tên
   mới, không installer/kho workflow ngoài. DEVELOPMENT.md mô tả build/test/gates
   thật; MAINTENANCE.md giữ invoice reconciliation, budget persistence, platform
   proof, security review, evidence retention, privacy và evaluation follow-ups.
   CHANGELOG thêm Unreleased; ADR0011 giải thích compatibility. LICENSE MIT
   byte-identical, Copyright (c) 2026 Hoang Nguyen, SHA-256
   4d7986336b20626e0f4b37bdcd8010f718856bdf0fecb11f40dfdd5f705d49e1.
   Tuyên bố sở hữu độc lập được ghi từ owner, không suy từ username/URL; không
   nhận mọi generic wording lịch sử là forensic proof tác giả độc lập. Đủ nghĩa
   vụ third-party/dependency và former inventory được giữ, không vendored source.
8. **Lịch sử.** 121 tệp được bảo vệ khớp hash baseline: product stories (trừ task
   copy bị retire), ADR0004–0010, SPEC-001/OSS-001, validation/failures, provenance
   tables, former dependency inventory, MIT, DB/migrations và notes v0.1.0.
   Không renumber ADR. SPEC-001 giữ nguyên 37 PASS /1 NOT_RUN và SHA riêng.
   Tag object1007928827bf2a1490e25c8775c3dc43737bd366 peel baseline a3df205;
   release430400350 và body không thay đổi. Tải ẩn danh cả archive của tag sau
   rename khớp192 file/content/mode. GitHub tự đổi prefix/compression bytes khi
   repo đổi tên; đây không phải source/tag/release bị sửa bởi REBRAND-001.
9. **Local/security.** Full verify.py chạy source
   0178ea96657b4f2c3765abea6cfd15f0dc9628f3, exit0, 10/10 gate PASS: fmt,
   strict Clippy, 32 ordinary tests +1 helper ignored được gọi riêng và PASS,
   build, runtime, license, rebrand/history/link, source archive, negative archive
   checks và clean archive build. Lệnh thực tế
   python3 scripts/verify.py --output /tmp/llmops-rebrand-local-0178ea9;
   command arrays/exits/log hashes trong local manifest. Literal cargo fmt --check
   và git diff --check cũng PASS. Gitleaks8.30.0 full-history/tree không finding/
   suppression; cargo-audit0.22.2 không vulnerability/warning/ignored advisory,
   RustSec2295466c4e9df26647fd67d8fa1c705ac2fc7f81 (1395 advisory).
   Candidate bec04674eb26a94feabc140c03c59d132bab2c49 có prepush checks riêng
   cho current docs/legal/history/exact archive privacy. Không gán CI cũ cho SHA mới.
10. **GitHub CI/artifact.** [Source CI41295320982](https://github.com/hzjanuary/llmops-toolkit/actions/runs/41295320982)
    đã PASS đủ offline-source/security/verified-source trên đúng
    bec04674eb26a94feabc140c03c59d132bab2c49. Đã tải ZIP và kiểm API digest,
    từng log thường/raw, exact source tar, SHA256SUMS và zero security findings.
    Offline artifact13079016982 SHA-256
    7ab3a887b512f8d1bde0c143d57881a0e0b8b5df57f6505e3ade878ffb7c80e6;
    security artifact13078998589 SHA-256
    27fd478398e0c7e76cabecdf8fb1b869145d92bbc27e9cc35c046ad77b3adea8.
    Snapshot bất biến ở rebrand-001/github-run-41295320982.json. Matrix ghi
    26 PASS tại nguồn có chứng cứ; commit bổ sung handoff/chứng cứ phải có CI
    riêng. Run/SHA/artifact cuối nằm trong PR metadata và báo cáo bàn giao cuối,
    không gán kết quả của bec0467 cho một SHA chưa thực sự chạy.
    Checkpoint kế tiếp [CI41302837995](https://github.com/hzjanuary/llmops-toolkit/actions/runs/41302837995)
    cũng PASS trên a0c5ea9233df203f12c34f025131455ff3abac12; snapshot và digest
    đầy đủ ở rebrand-001/github-run-41302837995.json. Chỉnh navigation tài liệu
    sau đó cần CI riêng trên HEAD cuối, không dùng lại kết quả này.
11. **Draft PR và HEAD.** [PR #2](https://github.com/hzjanuary/llmops-toolkit/pull/2),
    base main, draft=true, merged=false. Final exact HEAD và run/artifact mới nhất
    được xác minh trong PR metadata và báo cáo cuối; source doc không giả tự-SHA.
    Không merge/tag/release hoặc đổi settings trong initiative này.
12. **Giới hạn.** Không có real/paid inference, provider/judge thật, external
    target scan, model download, binary/package/Docker publication hay deploy.
    Mock không chứng minh live compatibility/certification; heuristic eval/scan
    không ground truth hoặc vulnerability certification. Không streaming; giá là
    estimate, budget theo process reset khi restart, unknown price/usage là null.
    Linux x86_64/Rust1.99/Python3.12+ được kiểm; Windows/macOS/lower MSRV chưa xác
    minh. Settings/ruleset proposals vẫn chưa áp dụng. Không sửa/yêu cầu quyền
    repo Harness bên ngoài. Cần owner review Draft PR, không có release mới.
13. **Workspace.** chore/llmops-toolkit-rebrand-cleanup theo dõi origin cùng tên;
    main giữ baseline, v0.1.0 giữ nguyên. Tracked tree sạch sau commit cuối;
    chỉ input docs/specs/ có trước còn untracked, không stage. Local evidence mới
    ở ignored local-evidence/rebrand-001; chứng cứ cũ giữ nguyên checksum.

| Đường dẫn đã xóa | SHA-256 nội dung gốc | Lý do |
| --- | --- | --- |
| `docs/legal/archive/legacy-scripts-readme.txt` | `0373e9f1cd1c8c27c0b0c029446c5c9488789555fb04c25ad3d1e194153f047d` | Obsolete external installer attachment; no installer required |
| `docs/HARNESS.md` | `656e68fdd1910065f9d03f432647b9ee985f526c48d2e12e5521b894daa5c3ba` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/HARNESS_BACKLOG.md` | `01cbf7d23e3a32c7a7f149eda4051bfd644f4fc302c2b910a0c1c0b58168e267` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/FEATURE_INTAKE.md` | `771043e61e3147abb13210f0205769f96fe08a26aa44f1ae9a92e43ae5d66b59` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/GLOSSARY.md` | `b39fdbf3e9997998cc53ba4089226d0ae7c2d5666776d9a990ade60868592ee0` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/templates/decision.md` | `e92ba7a5b7bc10ce6c213af75a191619fc305c3945cdd55d4dc6bac72dcd42c1` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/templates/high-risk-story/design.md` | `87412d8d8302c45b09369024be6cba734748ec428d5ebd03d73d3ce835ece2d3` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/templates/high-risk-story/execplan.md` | `03f9bb61f2e5b80f9658decd632ddbd695faed85f0b9906fa467f79ffef62b92` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/templates/high-risk-story/overview.md` | `94eaa234fd7c7f22a73dd5498c5188d3635eb780e1231b652886cf7cc5dd1a6b` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/templates/high-risk-story/validation.md` | `cdda39e3d7466a9d8a76ce5e7f804377a59c4b5bcc2651fc2112d99381d8aa81` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/templates/spec-intake.md` | `763726bcb6c42ae8e439c6dcc2ce2359d30f1192a4735ac5305cbda3a6bd7bb6` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/templates/story.md` | `7ea845bf5aab39ffd28785394d501d2d43d6d1a1177428714e009e7662c07228` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/templates/validation-report.md` | `0fb538321ebb09f17e81c9390caf7263ad9c12acb3025149bfe667d3911cddb7` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/decisions/0001-harness-first-development.md` | `8f329fe4ef318ffef2506724d689aa0d3280a6c7b4f50733e2bec35c07e66b6c` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/decisions/0002-post-spec-product-lifecycle.md` | `2f2ac9ae255d0f387204b4de67eee7ed97e7b2081b4891bc6f7d74cd14e0f212` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/decisions/0003-generic-spec-intake-harness.md` | `c17753144f544f1c9017d688ace26ff0a3a1ee5854135d2a82432320ef905f77` | Obsolete generic development workflow; useful actual commands/maintenance extracted into product-specific docs |
| `docs/stories/epics/E04-dashboard/US-043-unified-live-tui-monitor/legacy-agent-requirement.md` | `f73300ca6700712cd28a799c4271c6d9c59003f84003b723662e7958e5a480dc` | Redundant copied US-043 task prompt; technical story design/validation retain product requirements |

Bảng chi tiết Git blob/source/removal: [RETIREMENT_RECORD](../legal/RETIREMENT_RECORD.json). [Ma trận REBRAND-001](REBRAND-001-acceptance.md) tách khỏi kết quả SPEC-001.
