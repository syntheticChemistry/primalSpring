#!/usr/bin/env bash
# tools/godot_bridge.sh — Shared-memory frame bridge: Godot → petalTongue
#
# Watches a shared-memory file (RGBA8 framebuffer) and uploads it to petalTongue
# via `visualization.texture.upload` JSON-RPC whenever the file changes.
#
# Architecture:
#   Godot writes viewport pixels → /dev/shm/godot-frame.rgba + metadata
#   This daemon detects changes (inotify or poll) and pushes via JSON-RPC
#   petalTongue stores in TextureRegistry → Primitive::Texture renders it
#
# Usage:
#   ./tools/godot_bridge.sh                                  # defaults
#   ./tools/godot_bridge.sh --width 1280 --height 720        # explicit resolution
#   ./tools/godot_bridge.sh --socket /tmp/petaltongue.sock   # custom socket
#   ./tools/godot_bridge.sh --texture-id godot-viewport      # custom texture name
#   ./tools/godot_bridge.sh --poll-ms 16                     # ~60Hz polling
#
# Requirements: socat or nc (netcat), base64, inotifywait (optional, falls back to poll)

set -euo pipefail

SHM_PATH="${GODOT_SHM_PATH:-/dev/shm/godot-frame.rgba}"
META_PATH="${GODOT_SHM_META:-/dev/shm/godot-frame.meta}"
SOCKET="${PETALTONGUE_SOCKET:-/tmp/petaltongue.sock}"
TEXTURE_ID="${GODOT_TEXTURE_ID:-godot-viewport}"
WIDTH="${GODOT_FRAME_WIDTH:-800}"
HEIGHT="${GODOT_FRAME_HEIGHT:-600}"
POLL_MS=16
STOP=false

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log()  { echo -e "${CYAN}[godot_bridge]${NC} $*"; }
ok()   { echo -e "${GREEN}[godot_bridge]${NC} $*"; }
warn() { echo -e "${YELLOW}[godot_bridge]${NC} $*"; }
err()  { echo -e "${RED}[godot_bridge]${NC} $*" >&2; }

while [[ $# -gt 0 ]]; do
    case "$1" in
        --socket)     SOCKET="$2"; shift 2 ;;
        --texture-id) TEXTURE_ID="$2"; shift 2 ;;
        --width)      WIDTH="$2"; shift 2 ;;
        --height)     HEIGHT="$2"; shift 2 ;;
        --poll-ms)    POLL_MS="$2"; shift 2 ;;
        --shm)        SHM_PATH="$2"; shift 2 ;;
        --stop)       STOP=true; shift ;;
        *)            err "Unknown option: $1"; exit 1 ;;
    esac
done

if $STOP; then
    pkill -f "godot_bridge.sh" 2>/dev/null && ok "stopped" || warn "not running"
    exit 0
fi

send_rpc() {
    local payload="$1"
    if [[ -S "$SOCKET" ]]; then
        echo "$payload" | socat - UNIX-CONNECT:"$SOCKET" 2>/dev/null || \
        echo "$payload" | nc -U "$SOCKET" 2>/dev/null || true
    fi
}

read_meta() {
    if [[ -f "$META_PATH" ]]; then
        source "$META_PATH"
        WIDTH="${GODOT_WIDTH:-$WIDTH}"
        HEIGHT="${GODOT_HEIGHT:-$HEIGHT}"
    fi
}

upload_frame() {
    if [[ ! -f "$SHM_PATH" ]]; then
        return
    fi
    read_meta
    local b64_data
    b64_data=$(base64 -w0 < "$SHM_PATH")
    local payload
    payload=$(printf '{"jsonrpc":"2.0","method":"visualization.texture.upload","params":{"texture_id":"%s","width":%d,"height":%d,"format":"rgba8","data_base64":"%s"},"id":1}' \
        "$TEXTURE_ID" "$WIDTH" "$HEIGHT" "$b64_data")
    send_rpc "$payload"
}

push_scene_with_texture() {
    local payload
    payload=$(printf '{"jsonrpc":"2.0","method":"visualization.render.scene","params":{"session_id":"godot-bridge","title":"Godot Viewport","domain":"game","scene":{"nodes":[{"id":"godot-frame","primitives":[{"type":"texture","texture_id":"%s","x":0.0,"y":0.0,"width":%d.0,"height":%d.0,"opacity":1.0}]}]}},"id":2}' \
        "$TEXTURE_ID" "$WIDTH" "$HEIGHT")
    send_rpc "$payload"
}

log "Godot bridge daemon starting"
log "  SHM:        $SHM_PATH"
log "  Meta:       $META_PATH"
log "  Socket:     $SOCKET"
log "  Texture ID: $TEXTURE_ID"
log "  Resolution: ${WIDTH}x${HEIGHT}"
log "  Poll:       ${POLL_MS}ms"

push_scene_with_texture
ok "scene with texture reference pushed"

LAST_MOD=0
FRAME_COUNT=0

log "watching for frame updates (Ctrl+C to stop)..."
while true; do
    if [[ -f "$SHM_PATH" ]]; then
        MOD=$(stat -c %Y "$SHM_PATH" 2>/dev/null || echo "$LAST_MOD")
        if [[ "$MOD" != "$LAST_MOD" ]]; then
            upload_frame
            LAST_MOD="$MOD"
            FRAME_COUNT=$((FRAME_COUNT + 1))
            if (( FRAME_COUNT % 60 == 0 )); then
                log "uploaded $FRAME_COUNT frames"
            fi
        fi
    fi
    sleep "$(echo "scale=3; $POLL_MS / 1000" | bc)"
done
