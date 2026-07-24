#!/bin/bash
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TS=$(date +%Y%m%d_%H%M%S)

SONGBIRD="${SONGBIRD_BIN:-$(command -v songbird 2>/dev/null || echo "$HOME/.local/share/ecoPrimals/plasmidBin/primals/x86_64-unknown-linux-musl/songbird")}"
if [ ! -x "$SONGBIRD" ]; then
    echo "ERR: songbird not found at $SONGBIRD" >&2
    exit 1
fi

PEERS=(
    "golgi:10.13.37.1"
    "sporeGate:10.13.37.2"
    "eastGate:10.13.37.5"
    "ironGate:10.13.37.7"
    "northGate:10.13.37.8"
    "southGate:10.13.37.9"
)

for entry in "${PEERS[@]}"; do
    name="${entry%%:*}"
    ip="${entry##*:}"
    "$SONGBIRD" benchmark --mode tower-atomic --peer "${ip}:7700" --duration 10s --probes 20 --output json \
        > "$SCRIPT_DIR/tower-atomic_${name}_${TS}.json" 2>/dev/null || true
    "$SONGBIRD" benchmark --mode wireguard --peer "${ip}:7700" --duration 10s --probes 20 --output json \
        > "$SCRIPT_DIR/wireguard_${name}_${TS}.json" 2>/dev/null || true
done
