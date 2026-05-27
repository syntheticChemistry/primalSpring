#!/usr/bin/env bash
set -euo pipefail

# Generates validation/seed_fingerprints.toml from plasmidBin manifest + binary checksums.
# Fingerprint = BLAKE3("primal-seed-v1" || name || version || binary_blake3)

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SPRING_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PLASMID_BIN="${ECOPRIMALS_PLASMID_BIN:-${XDG_DATA_HOME:-$HOME/.local/share}/ecoPrimals/plasmidBin}"
MANIFEST="$PLASMID_BIN/manifest.toml"
PROVENANCE="$PLASMID_BIN/provenance.toml"
ARCH="x86_64-unknown-linux-musl"
BIN_DIR="$PLASMID_BIN/primals/$ARCH"
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

# Parse provenance.toml for source_commit enrichment (optional)
declare -A PROV_COMMITS
declare -A PROV_HASHES
if [[ -f "$PROVENANCE" ]]; then
    current_primal=""
    current_arch=""
    while IFS= read -r line; do
        if [[ "$line" =~ ^\[primals\.([a-z_]+)\.\"([^\"]+)\"\] ]]; then
            current_primal="${BASH_REMATCH[1]}"
            current_arch="${BASH_REMATCH[2]}"
        elif [[ -n "${current_primal:-}" && "$current_arch" == "$ARCH" ]]; then
            if [[ "$line" =~ ^source_commit[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
                PROV_COMMITS["$current_primal"]="${BASH_REMATCH[1]}"
            elif [[ "$line" =~ ^provenance_hash[[:space:]]*=[[:space:]]*\"([^\"]+)\" ]]; then
                PROV_HASHES["$current_primal"]="${BASH_REMATCH[1]}"
            fi
        fi
    done < "$PROVENANCE"
    echo "Loaded provenance for ${#PROV_COMMITS[@]} primals"
fi

{
    echo "# primalSpring Seed Fingerprints — BLAKE3"
    echo "# Generated: $(date +%Y-%m-%d)"
    echo "#"
    echo "# Each fingerprint = BLAKE3(\"$DOMAIN\" || primal_name || version || binary_blake3_checksum)"
    echo "# Binary checksums sourced from $BIN_DIR/"
    if [[ -f "$PROVENANCE" ]]; then
        echo "# Provenance source_commits from $PROVENANCE"
    fi
    echo "#"
    echo "# Verify: primalspring certify — Layer 0.5 (Seed Provenance)"
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

    # Provenance enrichment section (when provenance.toml is available)
    if [[ ${#PROV_COMMITS[@]} -gt 0 ]]; then
        echo ""
        echo "[provenance]"
        echo "# source_commit from plasmidBin provenance.toml ($ARCH)"
        for name in $(echo "${!PROV_COMMITS[@]}" | tr ' ' '\n' | sort); do
            printf "%-15s = { source_commit = \"%s\"" "$name" "${PROV_COMMITS[$name]}"
            if [[ -n "${PROV_HASHES[$name]:-}" ]]; then
                printf ", provenance_hash = \"%s\"" "${PROV_HASHES[$name]}"
            fi
            printf " }\n"
        done
    fi

    echo ""
    echo "[meta]"
    echo "algorithm = \"BLAKE3\""
    echo "domain_separator = \"$DOMAIN\""
    echo "input_format = \"domain_separator || primal_name || version || binary_blake3_checksum\""
    echo "arch = \"$ARCH\""
    echo "source = \"\$ECOPRIMALS_PLASMID_BIN/primals/$ARCH/\""
    if [[ -f "$PROVENANCE" ]]; then
        echo "provenance_source = \"\$ECOPRIMALS_PLASMID_BIN/provenance.toml\""
    fi
} > "$OUT"

echo "Wrote $(grep -c '=' "$OUT") entries to $OUT"
