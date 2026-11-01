# LLMOps Toolkit product and architecture decisions

Retained decisions keep their original numbers and historical text. Current
product contracts and later decisions supersede older unsafe/default behavior.
Generic process decisions 0001–0003 were retired from the current source tree;
original blobs/source hashes remain in the
[retirement record](../legal/RETIREMENT_RECORD.json) and Git.

- [0004 Rust/Axum/SQLite core](0004-rust-axum-sqlite-core-proxy.md).
- [0005 CLI safety scanner](0005-cli-red-team-scanner.md).
- [0006 RAG evaluation](0006-cli-rag-evaluation-engine.md).
- [0007 Historical Tauri dashboard](0007-tauri-dashboard.md), superseded by
  [0008 CLI dashboard](0008-pure-cli-dashboard.md).
- [0009 Unified terminal monitor](0009-unified-live-tui-monitor.md).
- [0010 Secure local-first refactor](0010-secure-local-oss-refactor.md), supersedes
  unsafe prototype details; its earlier license/release uncertainty is historical.
- [0011 Product rebrand and compatibility](0011-product-rebrand-and-environment-compatibility.md).

Record a new numbered decision when a product or architecture contract changes
meaningfully; explain the concrete choice, alternatives, limits and evidence.
Do not renumber retained records. See [development](../DEVELOPMENT.md).
