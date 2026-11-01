#!/usr/bin/env python3
"""Inspect exact Git source archives; never include untracked operator state."""
import argparse
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import subprocess
import tarfile

parser = argparse.ArgumentParser()
parser.add_argument("--revision", default="HEAD")
parser.add_argument("--output", required=True)
args = parser.parse_args()
revision = subprocess.check_output(["git", "rev-parse", args.revision + "^{commit}"]).decode().strip()
a = subprocess.check_output(["git", "archive", "--format=tar", revision])
b = subprocess.check_output(["git", "archive", "--format=tar", revision])
assert a == b, "source archives are not reproducible"
raw = subprocess.check_output(["git", "ls-tree", "-rz", "--full-tree", revision])
tree = {}
for item in raw.split(b"\0"):
    if not item:
        continue
    meta, path = item.split(b"\t", 1)
    mode, kind, oid = meta.decode().split()
    path = path.decode()
    assert kind == "blob" and mode in ("100644", "100755"), f"nonregular source: {path}"
    parts = PurePosixPath(path).parts
    assert not path.startswith("/") and ".." not in parts
    assert not any(p in ("target", ".git", "node_modules", "sessions", "credentials") for p in parts)
    name = parts[-1].lower()
    assert not (name.startswith(".env") and name != ".env.example"), f"private path: {path}"
    assert not any(x in name for x in (".sqlite", ".db-wal", ".db-shm")) and not name.endswith((".db", ".pem", ".key", ".p12", ".pfx")), f"operator state: {path}"
    tree[path] = (mode, oid)
manifest = []
with tarfile.open(fileobj=io.BytesIO(a)) as archive:
    files = {m.name: m for m in archive.getmembers() if m.isfile()}
    assert set(files) == set(tree), "archive does not match complete Git tree"
    assert all(m.isfile() or m.isdir() for m in archive.getmembers()), "archive contains special files"
    for name, member in sorted(files.items()):
        data = archive.extractfile(member).read()
        mode, oid = tree[name]
        assert bool(member.mode & 0o111) == (mode == "100755"), f"mode mismatch: {name}"
        assert data == subprocess.check_output(["git", "cat-file", "blob", oid]), f"content mismatch: {name}"
        manifest.append({"path": name, "git_mode": mode, "blob": oid,
                         "sha256": hashlib.sha256(data).hexdigest()})
output = Path(args.output)
output.mkdir(parents=True, exist_ok=True)
archive_path = output / "reviewed-source.tar"
with archive_path.open("xb") as f:
    f.write(a)
report = {"source_sha": revision, "archive_sha256": hashlib.sha256(a).hexdigest(), "files": manifest}
with (output / "archive-manifest.json").open("x") as f:
    json.dump(report, f, indent=2, sort_keys=True)
    f.write("\n")
print(f"PASS: {revision} reproducible archive, {len(manifest)} regular files, no prohibited state paths")
