#!/usr/bin/env python3
"""Verify source ZIP/tar.gz contents against an exact local Git commit, offline."""
import argparse
import hashlib
import io
import json
from pathlib import Path
import stat
import subprocess
import tarfile
import tempfile
import zipfile

MAX_BYTES = 64 * 1024 * 1024


def expected_tree(revision):
    sha = subprocess.check_output(["git", "rev-parse", revision + "^{commit}"]).decode().strip()
    tree = {}
    for entry in subprocess.check_output(["git", "ls-tree", "-rz", "--full-tree", sha]).split(b"\0"):
        if not entry:
            continue
        metadata, name = entry.split(b"\t", 1)
        mode, kind, oid = metadata.decode().split()
        assert kind == "blob" and mode in ("100644", "100755"), "unexpected Git object type"
        name = name.decode()
        data = subprocess.check_output(["git", "cat-file", "blob", oid])
        tree[name] = (hashlib.sha256(data).hexdigest(), mode == "100755", len(data))
    assert sum(v[2] for v in tree.values()) <= MAX_BYTES, "source snapshot exceeds verification budget"
    return sha, tree


def relative_path(name, prefixes, directory=False):
    assert not name.startswith("/") and "\\" not in name, "unsafe archive path"
    parts = name.rstrip("/").split("/") if directory else name.split("/")
    assert len(parts) >= (1 if directory else 2) and all(p and p not in (".", "..") for p in parts), "invalid source archive prefix/path"
    prefixes.add(parts[0])
    assert len(prefixes) == 1, "multiple archive roots"
    relative = "/".join(parts[1:])
    basename = parts[-1].lower()
    assert not any(p in (".git", "target", "node_modules", "sessions", "credentials") for p in parts)
    assert not (basename.startswith(".env") and basename != ".env.example"), "operator state in source archive"
    assert ".sqlite" not in basename and not basename.endswith((".db", ".pem", ".key", ".p12", ".pfx"))
    return relative


def archive_files(path, format_name):
    blob = path.read_bytes()
    assert len(blob) <= MAX_BYTES, "compressed archive exceeds verification budget"
    result, prefixes, directories = {}, set(), set()
    if format_name == "zip":
        with zipfile.ZipFile(io.BytesIO(blob)) as archive:
            entries = archive.infolist()
            assert len(entries) <= 10000, "too many ZIP entries"
            assert sum(i.file_size for i in entries) <= MAX_BYTES, "ZIP size budget exceeded"
            for item in entries:
                mode = item.external_attr >> 16
                assert stat.S_IFMT(mode) in (0, stat.S_IFREG, stat.S_IFDIR), "special ZIP entry"
                if item.is_dir():
                    name = relative_path(item.filename, prefixes, directory=True)
                    if name:
                        assert name not in directories, "duplicate ZIP directory"
                        directories.add(name)
                    continue
                name = relative_path(item.filename, prefixes)
                assert name not in result, "duplicate ZIP member"
                data = archive.read(item)
                result[name] = (hashlib.sha256(data).hexdigest(), bool(mode & 0o111), len(data))
    else:
        with tarfile.open(fileobj=io.BytesIO(blob), mode="r:gz") as archive:
            entries = archive.getmembers()
            assert len(entries) <= 10000, "too many tar entries"
            assert sum(i.size for i in entries) <= MAX_BYTES, "tar size budget exceeded"
            for item in entries:
                assert item.isfile() or item.isdir(), "special tar entry"
                if item.isdir():
                    name = relative_path(item.name, prefixes, directory=True)
                    if name:
                        assert name not in directories, "duplicate tar directory"
                        directories.add(name)
                    continue
                name = relative_path(item.name, prefixes)
                assert name not in result, "duplicate tar member"
                data = archive.extractfile(item).read()
                result[name] = (hashlib.sha256(data).hexdigest(), bool(item.mode & 0o111), len(data))
    return result, hashlib.sha256(blob).hexdigest(), directories


def verify(sha, tree, zip_path, tar_path):
    reports = []
    for format_name, path in (("zip", zip_path), ("tar.gz", tar_path)):
        files, digest, directories = archive_files(path, format_name)
        expected_directories = {"/".join(p.split("/")[:i]) for p in tree for i in range(1, len(p.split("/")))}
        assert directories <= expected_directories, "extra source archive directory"
        assert files == tree, f"{format_name} paths/content/modes differ from reviewed Git tree"
        reports.append({"format": format_name, "file": path.name, "sha256": digest,
                        "file_count": len(files), "contents_and_executable_modes_match": True})
    license_text = subprocess.check_output(["git", "show", sha + ":LICENSE"]).decode()
    assert "Copyright (c) 2026 Hoang Nguyen" in license_text
    assert "LICENSE.proposed" not in tree and not any(p.startswith("rag-harness/") for p in tree)
    return {"source_sha": sha, "archives": reports, "file_manifest": [
        {"path": name, "sha256": value[0], "executable": value[1], "size": value[2]}
        for name, value in sorted(tree.items())],
        "scope": "Observed compressed digests; every file/content/executable bit matches the exact commit. GitHub compression bytes may change; commit and content manifest are the durable source identity."}


def self_test(sha, tree):
    with tempfile.TemporaryDirectory(prefix="llmops-archive-verifier-") as temp:
        base = Path(temp)
        zip_path, tar_path = base / "fixture.zip", base / "fixture.tar.gz"
        zip_path.write_bytes(subprocess.check_output(["git", "archive", "--format=zip", "--prefix=zip-fixture/", sha]))
        tar_path.write_bytes(subprocess.check_output(["git", "archive", "--format=tar.gz", "--prefix=tar-fixture/", sha]))
        result = verify(sha, tree, zip_path, tar_path)
        tampered = base / "tampered.zip"
        with zipfile.ZipFile(zip_path) as original, zipfile.ZipFile(tampered, "w") as changed:
            for item in original.infolist():
                data = original.read(item)
                if item.filename.endswith("/LICENSE"):
                    data += b"\nSYNTHETIC TAMPER\n"
                changed.writestr(item, data)
        try:
            verify(sha, tree, tampered, tar_path)
        except AssertionError:
            result["tampered_content_rejected"] = True
        else:
            raise AssertionError("tampered source content was accepted")
        unsafe = base / "unsafe-directory.zip"
        with zipfile.ZipFile(zip_path) as original, zipfile.ZipFile(unsafe, "w") as changed:
            for item in original.infolist():
                changed.writestr(item, original.read(item))
            changed.writestr("../escape/", b"")
        try:
            verify(sha, tree, unsafe, tar_path)
        except AssertionError:
            result["unsafe_directory_rejected"] = True
        else:
            raise AssertionError("unsafe archive directory was accepted")
        result["mode"] = "offline_self_test_with_different_archive_prefixes"
        return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--revision", default="HEAD")
    parser.add_argument("--zip", type=Path)
    parser.add_argument("--tar-gz", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    sha, tree = expected_tree(args.revision)
    if args.self_test:
        assert args.zip is None and args.tar_gz is None, "self-test does not take public archive paths"
        report = self_test(sha, tree)
    else:
        assert args.zip and args.tar_gz, "both public source archives are required"
        report = verify(sha, tree, args.zip, args.tar_gz)
    with args.output.open("x") as output:
        json.dump(report, output, indent=2, sort_keys=True)
        output.write("\n")
    print(f"PASS: ZIP/tar.gz exact source paths, content and executable modes for {sha}; {len(tree)} files")
