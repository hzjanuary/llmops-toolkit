# LLMOps Toolkit verification tools

These project-owned tools are required for source validation. No external
workflow checkout, installer, template package or update service is needed.

| Script | Active responsibility |
| --- | --- |
| verify.py | Format/strict Clippy/offline tests/build, runtime, legal/identity/reference gates, exact archive and clean archive build with SHA-bound logs |
| runtime_smoke.py | Synthetic authenticated loopback proxy to SQLite to real TTY/headless; signals/backup/consent; new/legacy environment aliases and default DB compatibility |
| verify_public_archives.py | Offline exact-commit ZIP/tar.gz content/mode/path checks; prefix/tamper/unsafe-directory regressions |
| source_gate.py | Reproducible Git source archive, every blob/mode and private-path policy |
| license_inventory.py | Exact owner-confirmed MIT grant and declared license metadata for every retained root dependency |
| install_security_tools.py | Checksum-pinned Gitleaks and pinned cargo-audit, installed outside source |
| rebrand_gate.py | Identity/default-file, retirement/Git history, immutable evidence and Markdown-link integrity |

The root `mock_script.py` is retained separately as a product-owned optional
offline evaluation wrapper, exercised by runtime smoke. It is not an installer.
See [CONTRIBUTING](../CONTRIBUTING.md) for exact commands. Initial tool/registry/
advisory downloads use network; application tests use synthetic loopback fixtures
and no real inference, paid requests or external scans.

The obsolete installer attachment is removed from the current tree; only its
original path/blob/hash/source/removal identity remains in the retirement record
and Git. The toy crate stays retired. No script publishes a release, executable,
package, image or deployment.
