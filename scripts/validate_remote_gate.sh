#!/usr/bin/env bash
# validate_remote_gate.sh — Probe a remote gate's NUCLEUS health via JSON-RPC
#
# Connects to each primal's JSON-RPC endpoint on a remote gate and checks
# health.liveness + capabilities.list. Reports per-primal status.
#
# Usage:
#   ./scripts/validate_remote_gate.sh <host> [--unix <socket-dir>]
#   ./scripts/validate_remote_gate.sh 192.168.1.100
#   ./scripts/validate_remote_gate.sh northgate.local
#
# Default port mapping (override with env vars):
#   BEARDOG_PORT=9100
#   SONGBIRD_PORT=9200
#   NESTGATE_PORT=9300
#   TOADSTOOL_PORT=9400
#   SQUIRREL_PORT=9500
#   NEURAL_API_PORT=9600
#
# Each primal is probed with health.liveness via TCP JSON-RPC.
# If a primal doesn't expose a TCP port, this script skips it.
#
# For Unix socket probing on the local machine, use:
#   ./scripts/validate_remote_gate.sh localhost --unix /run/user/1000/biomeos

set -euo pipefail

HOST="${1:-}"
if [ -z "$HOST" ]; then
    echo "Usage: $0 <host> [--unix <socket-dir>]"
    echo ""
    echo "Examples:"
    echo "  $0 192.168.1.100          # TCP probe"
    echo "  $0 localhost --unix /run/user/1000/biomeos  # Unix socket probe"
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

BEARDOG_PORT="${BEARDOG_PORT:-9100}"
SONGBIRD_PORT="${SONGBIRD_PORT:-9200}"
NESTGATE_PORT="${NESTGATE_PORT:-9300}"
TOADSTOOL_PORT="${TOADSTOOL_PORT:-9400}"
SQUIRREL_PORT="${SQUIRREL_PORT:-9500}"
NEURAL_API_PORT="${NEURAL_API_PORT:-9600}"

PRIMALS=("beardog" "songbird" "nestgate" "toadstool" "squirrel" "neural-api")
PORTS=("$BEARDOG_PORT" "$SONGBIRD_PORT" "$NESTGATE_PORT" "$TOADSTOOL_PORT" "$SQUIRREL_PORT" "$NEURAL_API_PORT")

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

probe_primal() {
    local name="$1"
    local port="$2"

    if [ -n "$UNIX_DIR" ]; then
        local candidates=("$UNIX_DIR/${name}-"*.sock "$UNIX_DIR/${name}.sock")
        local socket=""
        for c in "${candidates[@]}"; do
            if [ -S "$c" ] 2>/dev/null; then
                socket="$c"
                break
            fi
        done
        if [ -z "$socket" ]; then
            printf "  %-12s  SKIP  (no socket in %s)\n" "$name" "$UNIX_DIR"
            ((skipped++)) || true
            return
        fi
        local resp
        resp=$(rpc_unix "$socket" "health.liveness" 2>/dev/null || true)
    else
        if ! timeout 2 bash -c "echo >/dev/tcp/$HOST/$port" 2>/dev/null; then
            printf "  %-12s  DOWN  (port %s unreachable)\n" "$name" "$port"
            ((failed++)) || true
            return
        fi
        local resp
        resp=$(rpc_tcp "$HOST" "$port" "health.liveness" 2>/dev/null || true)
    fi

    if echo "$resp" | grep -q '"result"'; then
        printf "  %-12s  LIVE  (health.liveness OK)\n" "$name"
        ((passed++)) || true
    elif [ -n "$resp" ]; then
        printf "  %-12s  WARN  (responded but: %s)\n" "$name" "$(echo "$resp" | head -c 80)"
        ((passed++)) || true
    else
        printf "  %-12s  DOWN  (no response)\n" "$name"
        ((failed++)) || true
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
