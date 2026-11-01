#!/usr/bin/env python3
"""Offline process/TTY integration. All state lives in a new temporary directory."""
import argparse
import errno
import json
import os
from pathlib import Path
import pty
import select
import signal
import socket
import sqlite3
import subprocess
import tempfile
import time
import urllib.request

parser = argparse.ArgumentParser()
parser.add_argument("--binary", default="target/debug/ragops-harness")
args = parser.parse_args()
binary = str(Path(args.binary).resolve())
env = dict(os.environ)
for key in ["OPENAI_API_KEY", "RAGOPS_PROXY_TOKEN", "RAGOPS_SCAN_TOKEN", "RAGOPS_DB_PATH"]:
    env.pop(key, None)


def run(*argv, **kw):
    return subprocess.run(argv, env=env, check=True, stdout=subprocess.PIPE,
                          stderr=subprocess.PIPE, timeout=60, **kw)


def port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def await_health(p, address):
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}))
    deadline = time.monotonic() + 8
    while time.monotonic() < deadline:
        if p.poll() is not None:
            raise AssertionError("server exited before health check")
        try:
            with opener.open(address + "/health", timeout=0.3) as r:
                assert json.load(r)["status"] == "ok"
                return
        except OSError:
            time.sleep(0.05)
    raise AssertionError("health deadline exceeded")


def read_pty(fd, seconds):
    chunks = []
    end = time.monotonic() + seconds
    while time.monotonic() < end:
        if select.select([fd], [], [], min(0.1, max(0, end-time.monotonic())))[0]:
            try:
                b = os.read(fd, 65536)
                if not b:
                    break
                chunks.append(b)
            except OSError as e:
                if e.errno != errno.EIO:
                    raise
                break
    return b"".join(chunks)


with tempfile.TemporaryDirectory(prefix="ragops-offline-smoke-") as tmp:
    db = str(Path(tmp) / "synthetic.sqlite3")
    fixture_env = dict(env, RAGOPS_TEST_DB=db)
    subprocess.run(["cargo", "test", "--locked", "--offline",
                    "proxy::tests::prepare_full_flow_receipt", "--", "--ignored", "--exact"],
                   env=fixture_env, check=True, timeout=60)
    with sqlite3.connect(db) as c:
        assert c.execute("SELECT outcome,total_tokens FROM request_receipts").fetchall() == [("success", 12)]
        assert c.execute("PRAGMA integrity_check").fetchone()[0] == "ok"
    summary = json.loads(run(binary, "--db", db, "dashboard", "--json").stdout)
    assert summary["attempts"] == 1 and summary["unknown_cost_count"] == 0
    assert summary["estimated_subtotal_usd"] > 0
    assert summary["avg_latency_ms"] is not None
    assert summary["latest"][0]["upstream_id"] == "synthetic-chat-id"
    assert summary["latest"][0]["created_at"]

    # Headless server: no terminal escapes, no provider configuration or egress.
    n = port()
    p = subprocess.Popen([binary, "--db", db, "serve", "--listen", f"127.0.0.1:{n}"],
                         env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    try:
        await_health(p, f"http://127.0.0.1:{n}")
        p.send_signal(signal.SIGTERM)
        out, err = p.communicate(timeout=8)
        assert p.returncode == 0 and b"\x1b[" not in out
    finally:
        if p.poll() is None:
            p.kill()
            p.wait()

    for signum in (signal.SIGINT, signal.SIGTERM):
        master, slave = pty.openpty()
        n = port()
        p = subprocess.Popen([binary, "--db", db, "serve", "--listen", f"127.0.0.1:{n}"],
                             env=env, stdout=slave, stderr=subprocess.PIPE)
        os.close(slave)
        try:
            await_health(p, f"http://127.0.0.1:{n}")
            screen = read_pty(master, 0.5)
            p.send_signal(signum)
            p.wait(timeout=8)
            screen += read_pty(master, 0.5)
            assert p.returncode == 0
            assert b"\x1b[?1049h" in screen and b"\x1b[?25l" in screen
            assert b"\x1b[?25h" in screen and b"\x1b[?1049l" in screen
            assert b"fixture-model" in screen and b"Attempts: 1" in screen
        finally:
            if p.poll() is None:
                p.kill()
                p.wait()
            p.stderr.close()
            os.close(master)

    # Bind failure must exit before entering alternate screen.
    with socket.socket() as busy:
        busy.bind(("127.0.0.1", 0))
        busy.listen()
        master, slave = pty.openpty()
        p = subprocess.Popen([binary, "--db", db, "serve", "--listen",
                              f"127.0.0.1:{busy.getsockname()[1]}"],
                             env=env, stdout=slave, stderr=subprocess.PIPE)
        os.close(slave)
        try:
            p.wait(timeout=8)
            assert p.returncode != 0 and b"\x1b[?1049h" not in read_pty(master, 0.2)
        finally:
            if p.poll() is None:
                p.kill()
                p.wait()
            p.stderr.close()
            os.close(master)

    backup = str(Path(tmp) / "backup.sqlite3")
    run(binary, "--db", db, "backup", "--destination", backup)
    restored = json.loads(run(binary, "--db", backup, "dashboard", "--json").stdout)
    assert restored == summary
    refusal = subprocess.run([binary, "--db", db, "backup", "--destination", backup],
                             env=env, capture_output=True, timeout=8)
    assert refusal.returncode != 0

    first = run(binary, "eval", "--dataset", "examples/dataset.json").stdout
    assert first == run(binary, "eval", "--dataset", "examples/dataset.json").stdout
    assert json.loads(first)["method"] == "lexical_subset_heuristic_v1"
    scan = json.loads(run(binary, "scan").stdout)
    assert scan["mode"] == "offline_fixtures"
    assert [c["outcome"] for c in scan["cases"]] == [
        "refusal_observed", "no_refusal_marker", "error_or_inconclusive"]

    # Live commands without consent fail before a socket/provider request.
    for argv in (["scan", "--target", "http://127.0.0.1:9/v1/chat/completions"],
                 ["eval", "--dataset", "examples/dataset.json", "--judge"]):
        p = subprocess.run([binary, *argv], env=env, capture_output=True, timeout=8)
        assert p.returncode != 0
print("PASS: authenticated loopback mock -> SQLite -> real TUI; signals/headless/bind failure; backup; deterministic offline commands; consent gates")
