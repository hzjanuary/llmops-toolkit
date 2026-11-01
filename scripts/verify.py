#!/usr/bin/env python3
"""Offline source verification with append-only SHA-bound evidence output."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tarfile
import tempfile
from datetime import datetime, timezone

parser = argparse.ArgumentParser()
parser.add_argument("--output", required=True)
args = parser.parse_args()
root = Path.cwd().resolve()
output = Path(args.output).resolve()
output.mkdir(parents=True, exist_ok=False)
subprocess.run(["git", "diff", "--exit-code", "HEAD", "--"], check=True, stdout=subprocess.DEVNULL)
sha = subprocess.check_output(["git", "rev-parse", "HEAD"]).decode().strip()
evidence = {"schema_version": 1, "source_sha": sha,
            "started_utc": datetime.now(timezone.utc).isoformat(),
            "rustc": subprocess.check_output(["rustc", "--version"]).decode().strip(), "checks": []}
env = dict(os.environ)
for key in ("OPENAI_API_KEY", "RAGOPS_PROXY_TOKEN", "RAGOPS_SCAN_TOKEN", "RAGOPS_DB_PATH"):
    env.pop(key, None)


def check(name, command, cwd=root):
    result = subprocess.run(command, cwd=cwd, env=env, text=True, stdout=subprocess.PIPE,
                            stderr=subprocess.STDOUT, timeout=300)
    # Publish only normalized paths; keep provider/operator secrets out by design.
    log = result.stdout.replace(str(root), "<checkout>").replace(str(output), "<evidence>")
    log_path = output / (name + ".txt")
    log_path.write_text(log)
    entry = {"id": name, "command": command, "cwd": "checkout" if cwd == root else "verified-source-archive",
             "exit_code": result.returncode, "status": "PASS" if result.returncode == 0 else "FAIL",
             "log": log_path.name, "log_sha256": hashlib.sha256(log.encode()).hexdigest()}
    evidence["checks"].append(entry)
    (output / "evidence.json").write_text(json.dumps(evidence, indent=2, sort_keys=True) + "\n")
    print(f"{entry['status']}: {name}", flush=True)
    if result.returncode:
        print(log[-8000:])
        raise SystemExit(result.returncode)


check("format", ["cargo", "fmt", "--all", "--", "--check"])
check("clippy", ["cargo", "clippy", "--locked", "--offline", "--all-targets", "--", "-D", "warnings"])
check("tests", ["cargo", "test", "--locked", "--offline"])
check("build", ["cargo", "build", "--locked", "--offline"])
check("runtime", ["python3", "scripts/runtime_smoke.py"])
check("licenses", ["python3", "scripts/license_inventory.py"])
check("source-archive", ["python3", "scripts/source_gate.py", "--revision", sha, "--output", str(output / "archive")])
with tempfile.TemporaryDirectory(prefix="ragops-verified-archive-") as temp:
    temp_path = Path(temp)
    with tarfile.open(output / "archive/reviewed-source.tar") as archive:
        archive.extractall(temp_path, filter="data")
    # New target directory proves the archived source compiles without local objects.
    check("clean-archive-build", ["cargo", "build", "--locked", "--offline", "--target-dir", str(temp_path / "target")], cwd=temp_path)
print(f"PASS: offline verification of source {sha}")
