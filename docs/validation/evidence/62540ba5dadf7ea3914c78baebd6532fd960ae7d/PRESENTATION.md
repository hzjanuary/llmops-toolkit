# Evidence presentation correction

The initial publication-delta git diff --check FAILED on one Cargo log's blank
line at EOF. It did not report a test, secret or license failure. No merge or
publication occurred with that failed check.

original-evidence.json preserves the exact first manifest bytes. The exact old
redacted test-log bytes are preserved in tests.untrimmed.b64; decoding matches
the original manifest's SHA-256. tests.txt is a presentation copy with the final
extra blank line removed. evidence.json records both digests and the original
manifest digest. Commands, source SHA, exit code and outcomes are unchanged.
The original committed representation also remains in Git history. Earlier
historical evidence directories are untouched. This is not scanner suppression.
