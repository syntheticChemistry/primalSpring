#!/bin/bash
set -euo pipefail

SONGBIRD="${HOME}/.local/bin/songbird"
OUTPUT_DIR="${HOME}/Development/ecoPrimals/springs/primalSpring/benchScale/tower_shadow"
TS=$(date +%Y%m%dT%H%M%S)

declare -A PEERS=(
    [golgi]="10.13.37.1:7700"
    [sporeGate]="10.13.37.2:7700"
    [flockGate]="10.13.37.6:7700"
    [sporeGate_LAN]="192.168.4.3:7700"
)

for name in "${!PEERS[@]}"; do
    addr="${PEERS[$name]}"
    "$SONGBIRD" benchmark --mode tower-atomic --peer "$addr" --duration 10s --probes 20 --output json \
        > "${OUTPUT_DIR}/tower-atomic_${name}_${TS}.json" 2>/dev/null || true
    "$SONGBIRD" benchmark --mode wireguard --peer "$addr" --duration 10s --probes 20 --output json \
        > "${OUTPUT_DIR}/wireguard_${name}_${TS}.json" 2>/dev/null || true
done
