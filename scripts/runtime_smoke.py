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
import threading
from http.server import BaseHTTPRequestHandler, HTTPServer
import urllib.request

parser = argparse.ArgumentParser()
parser.add_argument("--binary", default="target/debug/llmops-toolkit")
args = parser.parse_args()
binary = str(Path(args.binary).resolve())
env = dict(os.environ)
for key in ["OPENAI_API_KEY", "RAGOPS_PROXY_TOKEN", "RAGOPS_SCAN_TOKEN", "RAGOPS_DB_PATH",
            "LLMOPS_PROXY_TOKEN", "LLMOPS_SCAN_TOKEN", "LLMOPS_DB_PATH"]:
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
                health = json.load(r)
                assert health == {"status": "ok", "service": "llmops-toolkit"}
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


with tempfile.TemporaryDirectory(prefix="llmops-offline-smoke-") as tmp:
    db = str(Path(tmp) / "synthetic.sqlite3")
    fixture_env = dict(env, LLMOPS_TEST_DB=db)
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
    help_text = run(binary, "--help").stdout
    assert b"llmops-toolkit" in help_text and b"LLMOps Toolkit" in help_text
    assert b"LLMOps Toolkit local FinOps monitor" in run(binary, "--db", db, "dashboard").stdout

    # Synthetic process environment tests; never inherit operator credentials.
    before = Path(db).read_bytes()
    for prefix in ("LLMOPS", "RAGOPS"):
        configured = dict(env, **{prefix + "_DB_PATH": db})
        actual = subprocess.run([binary, "dashboard", "--json"], env=configured,
                                capture_output=True, check=True, timeout=8)
        assert json.loads(actual.stdout) == summary
    same = dict(env, LLMOPS_DB_PATH=db, RAGOPS_DB_PATH=db)
    actual = subprocess.run([binary, "dashboard", "--json"], env=same,
                            capture_output=True, check=True, timeout=8)
    assert json.loads(actual.stdout) == summary and Path(db).read_bytes() == before

    # --db does not bypass an alias conflict; rejection precedes file creation.
    forbidden_db = str(Path(tmp) / "must-not-exist.sqlite3")
    for suffix in ("DB_PATH", "PROXY_TOKEN", "SCAN_TOKEN"):
        conflicts = dict(env, **{"LLMOPS_" + suffix: "synthetic-current-value",
                                 "RAGOPS_" + suffix: "synthetic-legacy-value"})
        failed = subprocess.run([binary, "--db", forbidden_db, "serve"], env=conflicts,
                                capture_output=True, timeout=8)
        assert failed.returncode != 0 and not Path(forbidden_db).exists()
        assert b"conflicting" in failed.stderr
        for secret in (b"synthetic-current-value", b"synthetic-legacy-value"):
            assert secret not in failed.stdout + failed.stderr

    # Retain the legacy filename, ignore .env, never move an existing database.
    default_dir = Path(tmp) / "default-path"
    default_dir.mkdir()
    legacy_db = default_dir / "ragops_harness.sqlite3"
    with sqlite3.connect(db) as source, sqlite3.connect(legacy_db) as destination:
        source.backup(destination)
    default_before = legacy_db.read_bytes()
    (default_dir / ".env").write_text("LLMOPS_DB_PATH=must-not-load.sqlite3\n")
    actual = subprocess.run([binary, "dashboard", "--json"], cwd=default_dir, env=env,
                            capture_output=True, check=True, timeout=8)
    assert json.loads(actual.stdout) == summary
    assert legacy_db.read_bytes() == default_before and Path(db).read_bytes() == before
    assert not (default_dir / "must-not-load.sqlite3").exists()
    assert not (default_dir / "llmops_toolkit.sqlite3").exists()

    # Missing/invalid proxy credentials fail before I/O; values stay redacted.
    for prefix in ("LLMOPS", "RAGOPS"):
        for credential in (None, "synthetic-short"):
            credentials = dict(env, OPENAI_API_KEY="synthetic-provider-" + "p" * 32)
            if credential is not None:
                credentials[prefix + "_PROXY_TOKEN"] = credential
            failed = subprocess.run([binary, "--db", forbidden_db, "serve",
                                     "--enable-provider", "--model", "fixture-model",
                                     "--allow-unknown-price"], env=credentials,
                                    capture_output=True, timeout=8)
            assert failed.returncode != 0 and not Path(forbidden_db).exists()
            assert b"synthetic-short" not in failed.stdout + failed.stderr
            assert credentials["OPENAI_API_KEY"].encode() not in failed.stdout + failed.stderr

    # Both proxy token names configure startup; health sends zero inference.
    for prefix in ("LLMOPS", "RAGOPS"):
        credentials = dict(env, OPENAI_API_KEY="synthetic-provider-" + "p" * 32,
                           **{prefix + "_PROXY_TOKEN": "synthetic-owner-" + "o" * 32})
        n = port()
        process = subprocess.Popen([binary, "--db", db, "serve", "--listen",
                                    f"127.0.0.1:{n}", "--enable-provider", "--model",
                                    "fixture-model", "--allow-unknown-price"], env=credentials,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        try:
            await_health(process, f"http://127.0.0.1:{n}")
            process.send_signal(signal.SIGTERM)
            out, err = process.communicate(timeout=8)
            assert process.returncode == 0
            for name in ("OPENAI_API_KEY", prefix + "_PROXY_TOKEN"):
                assert credentials[name].encode() not in out + err
        finally:
            if process.poll() is None:
                process.kill()
                process.wait()

    # Target token aliases reach only a one-request synthetic loopback fixture.
    for prefix in ("LLMOPS", "RAGOPS"):
        seen = []
        secret = "synthetic-target-" + "t" * 32

        class Fixture(BaseHTTPRequestHandler):
            def do_POST(self):
                seen.append((self.path, self.headers.get("Authorization")))
                self.rfile.read(int(self.headers["Content-Length"]))
                body = b'{"choices":[{"message":{"content":"I cannot assist"}}]}'
                self.send_response(200)
                self.send_header("Content-Length", str(len(body)))
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, *_args):
                pass

        with HTTPServer(("127.0.0.1", 0), Fixture) as fixture:
            fixture.timeout = 5
            worker = threading.Thread(target=fixture.handle_request, daemon=True)
            worker.start()
            url = f"http://127.0.0.1:{fixture.server_port}/v1/chat/completions"
            result = subprocess.run([binary, "scan", "--target", url, "--authorize-target", url,
                                     "--model", "fixture-model", "--max-requests", "1"],
                                    env=dict(env, **{prefix + "_SCAN_TOKEN": secret}),
                                    capture_output=True, timeout=8)
            worker.join(timeout=6)
            assert not worker.is_alive() and result.returncode == 0
            assert seen == [("/v1/chat/completions", "Bearer " + secret)]
            assert secret.encode() not in result.stdout + result.stderr
            assert json.loads(result.stdout)["cases"][0]["outcome"] == "refusal_observed"

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
            assert b"LLMOps Toolkit local FinOps monitor" in screen
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
    assert run("python3", "mock_script.py").stdout == first
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
print("PASS: authenticated loopback mock -> SQLite -> real TUI; signals/headless/bind failure; backup; deterministic offline commands; consent gates; new/legacy environment aliases, redacted conflicts and credentials; unchanged legacy database default; no .env loading")
