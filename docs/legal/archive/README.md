# Retained historical dependency records

`dependency-inventory-before-crate-retirement.json` is preserved byte-for-byte
from the earlier source. It includes both former dependency graphs and the
then-pending project-license status; current MIT/dependencies are described in
../dependency-inventory.json and ../OWNER_DECLARATION.md.

Obsolete installer and copied task attachments are no longer distributed in the
current tree. Their original paths, Git blob IDs, SHA-256, source and removal
commits remain in ../RETIREMENT_RECORD.json; exact content is recoverable from
Git history. This removes setup ambiguity without rewriting history, altering
the v0.1.0 source snapshot or discarding dependency obligations.
