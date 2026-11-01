#!/usr/bin/env python3
"""Offline current identity, provenance, immutable-history and local-link gate."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tomllib
from urllib.parse import unquote, urlsplit

BASELINE = "a3df205abc5d9bd9d665f0a9072032bbd2145978"
TAG = "1007928827bf2a1490e25c8775c3dc43737bd366"


def git(*args):
    return subprocess.check_output(["git", *args])


tracked = git("ls-files", "-z").decode().split("\0")[:-1]
original = json.loads(Path("docs/validation/rebrand-001/baseline-source.json").read_text())
assert original["source_sha"] == BASELINE
record = json.loads(Path("docs/legal/RETIREMENT_RECORD.json").read_text())
removed = record["rebrand_001_retirements"]
assert len(removed) == 17
for entry in removed:
    path = entry["original_path"]
    assert path not in tracked and not Path(path).exists(), f"not retired: {path}"
    source = entry["original_source_commit"]
    blob = git("rev-parse", source + ":" + path).decode().strip()
    assert blob == entry["original_git_blob_sha"]
    assert hashlib.sha256(git("cat-file", "blob", blob)).hexdigest() == entry["content_sha256"]
    removal = entry["removal_commit"]
    assert re.fullmatch(r"[0-9a-f]{40}", removal or ""), f"unrecorded removal: {path}"
    assert git("diff", "--name-status", removal + "^", removal, "--", path).decode().strip() == "D\t" + path
    subprocess.run(["git", "merge-base", "--is-ancestor", removal, "HEAD"], check=True)
    assert entry["recoverable_from_git_history"] is True

immutable = []
for entry in original["tracked"]:
    path = entry["path"]
    protected = (
        path == "LICENSE" or path == "src/db.rs" or path.startswith("migrations/")
        or path.startswith("docs/validation/")
        or path.startswith("docs/stories/epics/")
        or path.startswith("docs/stories/initiatives/SPEC-001/")
        or path.startswith("docs/stories/initiatives/OSS-001-publication/")
        or path.startswith("docs/legal/file-provenance-782dfab.")
        or path == "docs/legal/archive/dependency-inventory-before-crate-retirement.json"
        or path == "docs/releases/v0.1.0.md"
        or bool(re.match(r"docs/decisions/000[4-9]-|docs/decisions/0010-", path))
    )
    if protected and path not in {e["original_path"] for e in removed}:
        assert hashlib.sha256(Path(path).read_bytes()).hexdigest() == entry["sha256"], f"historical/data change: {path}"
        immutable.append(path)
assert git("rev-parse", "v0.1.0").decode().strip() == TAG
assert git("rev-parse", "v0.1.0^{}").decode().strip() == BASELINE
assert git("merge-base", BASELINE, "HEAD").decode().strip() == BASELINE
old_matrix = git("show", BASELINE + ":docs/TEST_MATRIX.md").decode()
current_matrix = Path("docs/TEST_MATRIX.md").read_text()
marker = "## Historical baseline snapshots"
assert old_matrix[old_matrix.index(marker):] == current_matrix[current_matrix.index(marker):]

package = tomllib.loads(Path("Cargo.toml").read_text())["package"]
assert package["name"] == "llmops-toolkit"
assert package["description"] == "Local-first Rust CLI/TUI for OpenAI Chat Completions proxy telemetry, SQLite FinOps, offline RAG evaluation, and bounded LLM safety testing."
lock = tomllib.loads(Path("Cargo.lock").read_text())
old_lock = tomllib.loads(git("show", BASELINE + ":Cargo.lock").decode())
assert [p for p in lock["package"] if "source" in p] == [p for p in old_lock["package"] if "source" in p]
assert [p["name"] for p in lock["package"] if "source" not in p] == ["llmops-toolkit"]
assert "ragops_harness.sqlite3" in Path("src/environment.rs").read_text()
assert Path("README.md").read_text().startswith("# LLMOps Toolkit\n")
for name in ["verify", "runtime_smoke", "verify_public_archives", "source_gate", "license_inventory", "install_security_tools", "rebrand_gate"]:
    path = Path("scripts") / (name + ".py")
    assert path.is_file() and path.stat().st_mode & 0o111
assert not any(p.startswith(("rag-harness/", "docs/templates/")) for p in tracked)
assert "AGENT_REQUIREMENT.md" not in tracked
assert Path("mock_script.py").is_file()

# Historical prose/code examples retain their source identity. Check all actual
# Markdown links, including historical docs, without changing their bytes.
links = 0
for path in tracked:
    if not path.endswith(".md"):
        continue
    text = Path(path).read_text()
    text = re.sub(r"^```.*?^```[^\n]*$", "", text, flags=re.M | re.S)
    for destination in re.findall(r"\[[^\]]*\]\(([^\s)]+)(?:\s+\"[^\"]*\")?\)", text):
        parsed = urlsplit(destination.strip("<>"))
        if parsed.scheme or parsed.netloc or not parsed.path:
            continue
        target = Path(path).parent / unquote(parsed.path)
        assert target.exists(), f"broken Markdown destination: {path} -> {destination}"
        links += 1

# Do not exempt operational text from stale installer checks. Historical records
# are explicitly scoped; deleting current copies does not rewrite their evidence.
historical = ("docs/validation/", "docs/stories/epics/", "docs/stories/initiatives/SPEC-001/", "docs/stories/initiatives/OSS-001-publication/", "docs/releases/", "docs/legal/file-provenance-", "docs/legal/archive/dependency-inventory-")
external = re.compile(r"harness-experimental|hoangnb24/repository-harness|https?://[^\s]+install-harness|curl[^\n]*\|\s*(?:ba)?sh", re.I)
for path in tracked:
    if path.startswith(historical):
        continue
    # Patterns in this gate are policy, not installer commands.
    if path == "scripts/rebrand_gate.py":
        continue
    assert not external.search(Path(path).read_text()), f"obsolete operational installer reference: {path}"
print(f"PASS: LLMOps identity, 17 exact retirements, {len(immutable)} unchanged protected files, v0.1.0 tag, dependency graph, {links} Markdown destinations and no operational installer")
