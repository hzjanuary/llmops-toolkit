#!/usr/bin/env python3
"""Install pinned review tools outside the checkout; no provider/model access."""
import hashlib
from pathlib import Path
import subprocess
import sys
import tarfile
import urllib.request

root = Path(sys.argv[1]).resolve()
root.mkdir(parents=True, exist_ok=True)
version = "8.30.0"
url = f"https://github.com/gitleaks/gitleaks/releases/download/v{version}/gitleaks_{version}_linux_x64.tar.gz"
expected = "79a3ab579b53f71efd634f3aaf7e04a0fa0cf206b7ed434638d1547a2470a66e"
archive = root / "gitleaks.tar.gz"
body = urllib.request.urlopen(url, timeout=60).read()
assert hashlib.sha256(body).hexdigest() == expected, "gitleaks release checksum mismatch"
archive.write_bytes(body)
with tarfile.open(archive) as tar:
    tar.extract(tar.getmember("gitleaks"), root, filter="data")
subprocess.run(["cargo", "install", "cargo-audit", "--version", "0.22.2", "--locked", "--root", str(root)], check=True)
print("Installed gitleaks 8.30.0 (verified SHA256) and cargo-audit 0.22.2")
