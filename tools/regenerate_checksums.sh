#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Regenerate validation/CHECKSUMS from the current source tree.
# Uses b3sum (BLAKE3) — the same algorithm as primalspring::checksums.
#
# Usage: ./tools/regenerate_checksums.sh
#
# The file list comes from the existing CHECKSUMS manifest. To add or
# remove tracked files, edit the list below.

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

MANIFEST="validation/CHECKSUMS"

TRACKED_FILES=(
  ecoPrimal/src/bin/primalspring_guidestone/main.rs
  ecoPrimal/src/composition/mod.rs
  ecoPrimal/src/validation/mod.rs
  ecoPrimal/src/tolerances/mod.rs
  ecoPrimal/src/coordination/mod.rs
  ecoPrimal/src/bonding/mod.rs
  ecoPrimal/src/btsp/mod.rs
  ecoPrimal/src/deploy/mod.rs
  ecoPrimal/src/checksums.rs
  ecoPrimal/Cargo.toml
  graphs/fragments/tower_atomic.toml
  graphs/fragments/node_atomic.toml
  graphs/fragments/nest_atomic.toml
  graphs/fragments/nucleus.toml
  graphs/fragments/meta_tier.toml
  graphs/fragments/provenance_trio.toml
  graphs/downstream/downstream_manifest.toml
  graphs/downstream/proto_nucleate_template.toml
)

{
  echo "# primalSpring guideStone CHECKSUMS — BLAKE3"
  echo "# Generated: $(date +%Y-%m-%d)"
  echo "# Files: ${#TRACKED_FILES[@]}"
  echo "#"
  echo "# Verify: primalspring::checksums::verify_manifest()"
  b3sum "${TRACKED_FILES[@]}"
} > "$MANIFEST"

echo "Regenerated $MANIFEST (${#TRACKED_FILES[@]} files)"
