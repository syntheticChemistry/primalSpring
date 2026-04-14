#!/usr/bin/env bash
# validate_remote_gate.sh — Probe a remote gate's NUCLEUS health via JSON-RPC
#
# Connects to each primal's JSON-RPC endpoint on a remote gate and checks
# health.liveness + capabilities.list. Reports per-primal status.
#
# TCP ports are fallback defaults for cross-gate probing. On the same machine,
# use --unix to probe via sockets (the primary transport).
#
# Usage:
#   ./scripts/validate_remote_gate.sh <host> [--unix <socket-dir>]
#   ./scripts/validate_remote_gate.sh 192.168.1.100
#   ./scripts/validate_remote_gate.sh northgate.local

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PLASMIDBIN_PORTS="$SCRIPT_DIR/../../../infra/plasmidBin/ports.env"

HOST="${1:-}"
if [ -z "$HOST" ]; then
    echo "Usage: $0 <host> [--unix <socket-dir>]"
    echo ""
    echo "Examples:"
    echo "  $0 192.168.1.100          # TCP probe (cross-gate)"
    echo "  $0 localhost --unix /run/user/1000/biomeos  # Unix socket probe (local)"
    exit 1
fi

UNIX_DIR=""
shift || true
while [ $# -gt 0 ]; do
    case "$1" in
        --unix) shift; UNIX_DIR="${1:-}"; shift || true ;;
        *) shift ;;
    esac
done

# Source shared port definitions from plasmidBin if available
if [[ -f "$PLASMIDBIN_PORTS" ]]; then
    # shellcheck source=../../infra/plasmidBin/ports.env
    source "$PLASMIDBIN_PORTS"
else
    BEARDOG_PORT="${BEARDOG_PORT:-9100}"
    SONGBIRD_PORT="${SONGBIRD_PORT:-9200}"
    NESTGATE_PORT="${NESTGATE_PORT:-9300}"
    TOADSTOOL_PORT="${TOADSTOOL_PORT:-9400}"
    SQUIRREL_PORT="${SQUIRREL_PORT:-9500}"
    BIOMEOS_PORT="${BIOMEOS_PORT:-9800}"
fi

PRIMALS=("beardog" "songbird" "nestgate" "toadstool" "squirrel" "biomeos")
PORTS=("$BEARDOG_PORT" "$SONGBIRD_PORT" "$NESTGATE_PORT" "$TOADSTOOL_PORT" "$SQUIRREL_PORT" "$BIOMEOS_PORT")

passed=0
failed=0
skipped=0

rpc_tcp() {
    local host="$1"
    local port="$2"
    local method="$3"
    local req
    req=$(printf '{"jsonrpc":"2.0","method":"%s","params":{},"id":1}\n' "$method")
    echo "$req" | timeout 5 nc -w 3 "$host" "$port" 2>/dev/null | head -1
}

rpc_unix() {
    local socket="$1"
    local method="$2"
    local req
    req=$(printf '{"jsonrpc":"2.0","method":"%s","params":{},"id":1}\n' "$method")
    echo "$req" | timeout 5 socat - "UNIX-CONNECT:$socket" 2>/dev/null | head -1
}

HEALTH_METHODS=("health.liveness" "health.check" "health" "toadstool.health")

try_health_probe_unix() {
    local socket="$1"
    for method in "${HEALTH_METHODS[@]}"; do
        local resp
        resp=$(rpc_unix "$socket" "$method" 2>/dev/null || true)
        if echo "$resp" | grep -q '"result"'; then
            echo "$method"
            return 0
        fi
    done
    return 1
}

try_health_probe_tcp() {
    local host="$1"
    local port="$2"
    for method in "${HEALTH_METHODS[@]}"; do
        local resp
        resp=$(rpc_tcp "$host" "$port" "$method" 2>/dev/null || true)
        if echo "$resp" | grep -q '"result"'; then
            echo "$method"
            return 0
        fi
    done
    return 1
}

probe_primal() {
    local name="$1"
    local port="$2"

    if [ -n "$UNIX_DIR" ]; then
        local candidates=(
            "$UNIX_DIR/${name}.jsonrpc.sock"
            "$UNIX_DIR/${name}-"*.sock
            "$UNIX_DIR/${name}.sock"
        )
        # Squirrel and others may place sockets in sibling dirs
        local parent
        parent="$(dirname "$UNIX_DIR")"
        candidates+=("$parent/${name}/${name}.sock")

        local socket=""
        for c in "${candidates[@]}"; do
            if [ -S "$c" ] 2>/dev/null; then
                socket="$c"
                break
            fi
        done
        if [ -z "$socket" ]; then
            printf "  %-12s  SKIP  (no socket found)\n" "$name"
            ((skipped++)) || true
            return
        fi
        local method
        if method=$(try_health_probe_unix "$socket"); then
            printf "  %-12s  LIVE  (%s OK)\n" "$name" "$method"
            ((passed++)) || true
        else
            printf "  %-12s  DOWN  (no health method responded)\n" "$name"
            ((failed++)) || true
        fi
    else
        if ! timeout 2 bash -c "echo >/dev/tcp/$HOST/$port" 2>/dev/null; then
            printf "  %-12s  DOWN  (port %s unreachable)\n" "$name" "$port"
            ((failed++)) || true
            return
        fi
        local method
        if method=$(try_health_probe_tcp "$HOST" "$port"); then
            printf "  %-12s  LIVE  (%s OK)\n" "$name" "$method"
            ((passed++)) || true
        else
            printf "  %-12s  DOWN  (port %s, no health method responded)\n" "$name" "$port"
            ((failed++)) || true
        fi
    fi
}

echo "=== Gate Health Probe: $HOST ==="
if [ -n "$UNIX_DIR" ]; then
    echo "Mode: Unix socket ($UNIX_DIR)"
else
    echo "Mode: TCP"
fi
echo ""

for i in "${!PRIMALS[@]}"; do
    probe_primal "${PRIMALS[$i]}" "${PORTS[$i]}"
done

echo ""
echo "=== Summary ==="
echo "  Live:    $passed"
echo "  Down:    $failed"
echo "  Skipped: $skipped"

if [ "$passed" -eq 0 ]; then
    echo ""
    echo "No primals reachable. Check that NUCLEUS is running on $HOST."
    exit 1
fi
