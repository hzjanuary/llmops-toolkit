# RAGOps project verification scripts

These are local project tools. No external Harness installer, runtime,
orchestration service or update mechanism is needed by RAGOps.

| Script | Active responsibility |
| --- | --- |
| verify.py | Pinned format/Clippy/offline tests/build, runtime flow, legal inventory, exact archive and clean archive build with SHA-bound logs |
| runtime_smoke.py | Authenticated synthetic loopback mock -> SQLite -> real TTY/headless process flow; signals/backup/consent gates |
| verify_public_archives.py | Offline ZIP/tar.gz verification against an exact commit, with different archive prefixes and tamper rejection self-test |
| source_gate.py | Reproducible exact Git source archive; every blob/mode and private-path policy |
| license_inventory.py | Owner-confirmed exact MIT grant and declared license metadata for every retained root dependency |
| install_security_tools.py | Checksum-pinned Gitleaks and pinned cargo-audit, installed outside checkout |

See CONTRIBUTING.md for exact commands. Initial registry/tool/advisory downloads
use network access; application tests make no real inference or external scans.
No script publishes an executable, package, image or deployment.

The obsolete external installer examples are preserved verbatim as historical
text in docs/legal/archive/legacy-scripts-readme.txt. Do not execute them. The
unused nested Hello World crate was retired together with its CI/inventory
references; historical dependency data and evidence remain preserved.
