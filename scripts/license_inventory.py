#!/usr/bin/env python3
"""Check declared dependency licenses and make a reproducible inventory.

This does not establish ownership of project source or satisfy future binary
redistribution obligations. The root project's license is separately blocked.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import tomllib

# Exact reviewed SPDX expressions; OR expressions permit a permissive choice.
# AND expressions retain all obligations. No LGPL-only dependency is accepted.
ALLOWED = {
    "MIT", "Apache-2.0", "ISC", "BSD-3-Clause", "Zlib", "Unicode-3.0",
    "MIT OR Apache-2.0", "MIT/Apache-2.0", "Apache-2.0/MIT", "Apache-2.0 / MIT",
    "Apache-2.0 OR MIT", "Apache-2.0 OR ISC OR MIT", "BSD-2-Clause OR Apache-2.0 OR MIT",
    "(Apache-2.0 OR MIT) AND BSD-3-Clause", "MIT AND BSD-3-Clause", "Unlicense OR MIT",
    "MIT OR Apache-2.0 OR LGPL-2.1-or-later", "MIT OR Apache-2.0 OR Zlib",
    "Zlib OR Apache-2.0 OR MIT", "CDLA-Permissive-2.0", "Apache-2.0 AND ISC",
    "Apache-2.0 OR BSL-1.0", "(MIT OR Apache-2.0) AND Unicode-3.0",
    "Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT",
}
parser = argparse.ArgumentParser()
parser.add_argument("--write", action="store_true")
args = parser.parse_args()
inventory = []
for manifest in ("Cargo.toml", "rag-harness/Cargo.toml"):
    data = json.loads(subprocess.check_output([
        "cargo", "metadata", "--locked", "--offline", "--format-version", "1", "--manifest-path", manifest]))
    lockpath = Path(manifest).with_name("Cargo.lock")
    lock = tomllib.loads(lockpath.read_text())
    checksums = {(p["name"], p["version"]): p.get("checksum") for p in lock["package"]}
    packages = []
    for p in sorted(data["packages"], key=lambda p: (p["name"], p["version"])):
        if p["source"] is None:
            continue
        assert p["source"] == "registry+https://github.com/rust-lang/crates.io-index", "unreviewed dependency source"
        assert p["license"] in ALLOWED, f"unreviewed declared license: {p['name']} {p['license']}"
        packages.append({"name": p["name"], "version": p["version"], "declared_license": p["license"],
                         "source": p["source"], "checksum": checksums[(p["name"], p["version"])],
                         "repository": p["repository"]})
    inventory.append({"manifest": manifest, "lock_sha256": hashlib.sha256(lockpath.read_bytes()).hexdigest(),
                      "dependencies": packages})
text = json.dumps({"schema_version": 1, "scope": "declared metadata, all lockfile platforms; no dependency source is vendored",
                   "project_license": "BLOCKED: owner and inherited-source rights review required",
                   "manifests": inventory}, indent=2, sort_keys=True) + "\n"
path = Path("docs/legal/dependency-inventory.json")
if args.write:
    path.write_text(text)
else:
    assert path.read_text() == text, "dependency inventory stale; regenerate with --write and review"
print("PASS: both lockfile license inventories match reviewed declared SPDX expressions; project ownership remains BLOCKED")
