#!/usr/bin/env bash
set -euo pipefail

# Generates validation/seed_fingerprints.toml from plasmidBin manifest + binary checksums.
# Fingerprint = BLAKE3("primal-seed-v1" || name || version || binary_blake3)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SPRING_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PLASMIN_BIN="${ECOPRIMALS_PLASMID_BIN:-$(cd "$SPRING_ROOT/../../infra/plasmidBin" && pwd)}"
MANIFEST="$PLASMIN_BIN/manifest.toml"
ARCH="x86_64-unknown-linux-musl"
BIN_DIR="$PLASMIN_BIN/primals/$ARCH"
OUT="$SPRING_ROOT/validation/seed_fingerprints.toml"
DOMAIN="primal-seed-v1"

if [[ ! -f "$MANIFEST" ]]; then
    echo "ERROR: manifest not found at $MANIFEST" >&2
    exit 1
fi
if [[ ! -d "$BIN_DIR" ]]; then
    echo "ERROR: binary dir not found at $BIN_DIR" >&2
    exit 1
fi

declare -A VERSIONS
while IFS= read -r line; do
    if [[ "$line" =~ ^\[primals\.([a-z_]+)\] ]]; then
        current="${BASH_REMATCH[1]}"
    elif [[ -n "${current:-}" && "$line" =~ ^latest[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
        VERSIONS["$current"]="${BASH_REMATCH[1]}"
        current=""
    fi
done < "$MANIFEST"

{
    echo "# primalSpring Seed Fingerprints — BLAKE3"
    echo "# Generated: $(date +%Y-%m-%d)"
    echo "#"
    echo "# Each fingerprint = BLAKE3(\"$DOMAIN\" || primal_name || version || binary_blake3_checksum)"
    echo "# Binary checksums sourced from $BIN_DIR/"
    echo "#"
    echo "# Verify: primalspring_guidestone Layer 0.5 (Seed Provenance)"
    echo "# Regenerate: tools/gen_seed_fingerprints.sh"
    echo ""
    echo "[fingerprints]"

    for bin in "$BIN_DIR"/*; do
        name="$(basename "$bin")"
        [[ -f "$bin" ]] || continue
        ver="${VERSIONS[$name]:-}"
        if [[ -z "$ver" ]]; then
            echo "# SKIP $name — no version in manifest" >&2
            continue
        fi
        cksum="$(b3sum "$bin" | cut -d' ' -f1)"
        fp="$(echo -n "${DOMAIN}${name}${ver}${cksum}" | b3sum --no-names)"
        printf "%-15s = \"%s\"\n" "$name" "$fp"
    done

    echo ""
    echo "[meta]"
    echo "algorithm = \"BLAKE3\""
    echo "domain_separator = \"$DOMAIN\""
    echo "input_format = \"domain_separator || primal_name || version || binary_blake3_checksum\""
    echo "arch = \"$ARCH\""
    echo "source = \"infra/plasmidBin/primals/$ARCH/\""
} > "$OUT"

echo "Wrote $(grep -c '=' "$OUT") entries to $OUT"
