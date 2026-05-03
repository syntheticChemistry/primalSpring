#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# tools/fetch_primals.sh — Bootstrap primal binaries from plasmidBin GitHub Releases
#
# Self-contained consumer script. No plasmidBin repo checkout needed.
# Downloads musl-static primal binaries and verifies BLAKE3 checksums.
#
# This is the canonical pattern for any spring, composition, or deployment
# to obtain primal binaries. primalSpring uses this the same way
# downstream projects (hotSpring, esotericWebb, etc.) would.
#
# Usage:
#   ./tools/fetch_primals.sh                  # Fetch all 13 NUCLEUS primals
#   ./tools/fetch_primals.sh --primal beardog # Fetch a single primal
#   ./tools/fetch_primals.sh --force          # Re-download even if present
#   ./tools/fetch_primals.sh --dry-run        # Show what would be fetched
#
# Prerequisites:
#   - curl
#   - b3sum (cargo install b3sum) — optional, skips checksum if missing
#   - gh CLI (optional, for private repos; falls back to curl)
#
# Output directory (in priority order):
#   1. $ECOPRIMALS_PLASMID_BIN
#   2. $XDG_DATA_HOME/ecoPrimals/plasmidBin
#   3. ~/.local/share/ecoPrimals/plasmidBin

set -euo pipefail

GITHUB_REPO="ecoPrimals/plasmidBin"

NUCLEUS_PRIMALS=(
    beardog songbird toadstool barracuda coralreef
    nestgate rhizocrypt loamspine sweetgrass
    biomeos squirrel skunkbat petaltongue
)

DEFENSE_PRIMALS=()

DRY_RUN=false
FORCE=false
FETCH_ALL=true
FILTER=""
RELEASE_TAG=""

DOWNLOADED=0
SKIPPED=0
VERIFIED=0
FAILED=0

usage() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --primal NAME    Fetch a single primal by name"
    echo "  --release TAG    Fetch from specific release tag (default: latest)"
    echo "  --force          Re-download even if binary exists"
    echo "  --dry-run        Show what would be fetched"
    echo "  --dest DIR       Override output directory"
    echo "  --help           Show this help"
    echo ""
    echo "Default output: \${ECOPRIMALS_PLASMID_BIN:-~/.local/share/ecoPrimals/plasmidBin}"
}

while [[ $# -gt 0 ]]; do
    case "$1" in
        --primal)    FILTER="$2"; FETCH_ALL=false; shift 2 ;;
        --release)   RELEASE_TAG="$2"; shift 2 ;;
        --force)     FORCE=true; shift ;;
        --dry-run)   DRY_RUN=true; shift ;;
        --dest)      ECOPRIMALS_PLASMID_BIN="$2"; shift 2 ;;
        --help)      usage; exit 0 ;;
        -*)          echo "Unknown option: $1"; usage; exit 1 ;;
        *)           FILTER="$1"; FETCH_ALL=false; shift ;;
    esac
done

resolve_plasmid_bin() {
    if [[ -n "${ECOPRIMALS_PLASMID_BIN:-}" ]]; then
        echo "$ECOPRIMALS_PLASMID_BIN"
    else
        echo "${XDG_DATA_HOME:-$HOME/.local/share}/ecoPrimals/plasmidBin"
    fi
}

detect_target_triple() {
    local machine kernel
    machine=$(uname -m)
    kernel=$(uname -s | tr '[:upper:]' '[:lower:]')
    case "$kernel" in
        linux)
            case "$machine" in
                x86_64)  echo "x86_64-unknown-linux-musl" ;;
                aarch64) echo "aarch64-unknown-linux-musl" ;;
                armv7l)  echo "armv7-unknown-linux-musleabihf" ;;
                riscv64) echo "riscv64gc-unknown-linux-musl" ;;
                *)       echo "${machine}-unknown-linux-musl" ;;
            esac ;;
        darwin)
            case "$machine" in
                x86_64)  echo "x86_64-apple-darwin" ;;
                arm64)   echo "aarch64-apple-darwin" ;;
                *)       echo "${machine}-apple-darwin" ;;
            esac ;;
        *)  echo "${machine}-unknown-${kernel}" ;;
    esac
}

has_b3sum() { command -v b3sum >/dev/null 2>&1; }
has_gh()    { command -v gh >/dev/null 2>&1; }

resolve_release_tag() {
    if [[ -n "$RELEASE_TAG" ]]; then
        echo "$RELEASE_TAG"
        return
    fi
    if has_gh; then
        gh release view --repo "$GITHUB_REPO" --json tagName -q '.tagName' 2>/dev/null || true
    else
        curl -sf --max-time 10 "https://api.github.com/repos/$GITHUB_REPO/releases/latest" 2>/dev/null \
            | grep -oP '"tag_name"\s*:\s*"\K[^"]+' | head -1 || true
    fi
}

list_recent_releases() {
    if has_gh; then
        gh release list --repo "$GITHUB_REPO" --limit 5 2>/dev/null \
            | awk -F'\t' '{print $3}' | head -5 || true
    else
        curl -sf --max-time 10 "https://api.github.com/repos/$GITHUB_REPO/releases?per_page=5" 2>/dev/null \
            | grep -oP '"tag_name"\s*:\s*"\K[^"]+' || true
    fi
}

download_asset() {
    local tag="$1" asset="$2" dest="$3"
    local url="https://github.com/$GITHUB_REPO/releases/download/$tag/$asset"

    if $DRY_RUN; then
        echo "    [dry-run] $url -> $dest"
        return 0
    fi

    if curl -sfL --max-time 300 -o "$dest" "$url" 2>/dev/null; then
        chmod +x "$dest"
        return 0
    fi
    return 1
}

fetch_checksum() {
    local tag="$1" primal="$2" arch="$3"
    local checksums_url="https://github.com/$GITHUB_REPO/releases/download/$tag/checksums.toml"
    local checksums_cache
    checksums_cache="$DEST_DIR/.checksums-${tag}.toml"

    if [[ ! -f "$checksums_cache" ]] || $FORCE; then
        curl -sfL --max-time 30 -o "$checksums_cache" "$checksums_url" 2>/dev/null || true
    fi

    if [[ -f "$checksums_cache" ]]; then
        local in_section=false section_header
        section_header="primals\\.${primal}"
        while IFS= read -r line; do
            if [[ "$line" =~ ^\[${section_header}\] ]]; then
                in_section=true; continue
            fi
            if $in_section && [[ "$line" =~ ^\[ ]]; then break; fi
            if $in_section && [[ "$line" =~ \"${arch}\"[[:space:]]*=[[:space:]]*\"(.*)\" ]]; then
                echo "${BASH_REMATCH[1]}"; return 0
            fi
        done < "$checksums_cache"
    fi
}

DEST_DIR="$(resolve_plasmid_bin)"
ARCH=$(detect_target_triple)
BIN_DIR="$DEST_DIR/primals/$ARCH"

echo "primalSpring fetch — $(date -Iseconds)"
echo "  Arch:   $ARCH"
echo "  Dest:   $BIN_DIR"

TAG=$(resolve_release_tag)
if [[ -z "$TAG" ]]; then
    echo "ERROR: Could not resolve release tag from $GITHUB_REPO"
    echo "  Check network connectivity or pass --release TAG"
    exit 1
fi
echo "  Release: $TAG"
echo ""

mkdir -p "$BIN_DIR"

primals_to_fetch=()
if $FETCH_ALL; then
    primals_to_fetch=("${NUCLEUS_PRIMALS[@]}" "${DEFENSE_PRIMALS[@]}")
else
    primals_to_fetch=("$FILTER")
fi

for primal in "${primals_to_fetch[@]}"; do
    local_path="$BIN_DIR/$primal"
    printf "  [%-12s] " "$primal"

    if [[ -f "$local_path" ]] && ! $FORCE; then
        echo "EXISTS (use --force to re-download)"
        SKIPPED=$((SKIPPED + 1))
        continue
    fi

    # Remove existing binary first — curl can't overwrite a running executable (CURLE_WRITE_ERROR)
    rm -f "$local_path"

    # genomeBin asset naming: {name}-{triple} (multi-arch releases)
    # Falls back to plain {name} for backward compatibility with older releases.
    # If the primal isn't in the requested release (single-primal harvests only
    # include the triggering primal), cascade through recent releases.
    got_it=false
    got_tag=""
    for try_tag in "$TAG" $(list_recent_releases | grep -v "^${TAG}$" | head -4); do
        if download_asset "$try_tag" "${primal}-${ARCH}" "$local_path"; then
            got_it=true; got_tag="$try_tag"; break
        elif download_asset "$try_tag" "$primal" "$local_path"; then
            got_it=true; got_tag="$try_tag"; break
        fi
    done

    if ! $got_it; then
        echo "FAIL  could not download"
        FAILED=$((FAILED + 1))
        continue
    fi

    if $DRY_RUN; then
        DOWNLOADED=$((DOWNLOADED + 1))
        continue
    fi

    if has_b3sum; then
        expected=$(fetch_checksum "$got_tag" "$primal" "$ARCH")
        if [[ -n "${expected:-}" ]]; then
            actual=$(b3sum --no-names "$local_path")
            if [[ "$actual" == "$expected" ]]; then
                if [[ "$got_tag" != "$TAG" ]]; then
                    echo "OK  checksum verified (from $got_tag)"
                else
                    echo "OK  checksum verified"
                fi
                VERIFIED=$((VERIFIED + 1))
            else
                echo "FAIL  checksum mismatch (removing)"
                rm -f "$local_path"
                FAILED=$((FAILED + 1))
                continue
            fi
        else
            echo "OK  (no checksum entry)"
        fi
    else
        echo "OK  (b3sum not installed — skipping checksum)"
    fi

    DOWNLOADED=$((DOWNLOADED + 1))
done

echo ""
echo "Summary:"
echo "  Downloaded: $DOWNLOADED"
echo "  Verified:   $VERIFIED"
echo "  Skipped:    $SKIPPED"
echo "  Failed:     $FAILED"

if [[ $DOWNLOADED -gt 0 || $SKIPPED -gt 0 ]]; then
    echo ""
    echo "Binaries ready at: $BIN_DIR"
    echo "Set for experiments:"
    echo "  export ECOPRIMALS_PLASMID_BIN=$DEST_DIR"
fi

[[ $FAILED -eq 0 ]]
