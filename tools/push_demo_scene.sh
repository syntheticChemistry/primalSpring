#!/usr/bin/env bash
# tools/push_demo_scene.sh — Push Grammar of Graphics + streaming data to petalTongue
#
# Sends demo scenes to a running petalTongue IPC socket via JSON-RPC.
# Demonstrates Grammar of Graphics, Manim-style animation, and streaming data.
#
# Usage:
#   ./tools/push_demo_scene.sh                           # default socket
#   ./tools/push_demo_scene.sh --socket /tmp/pt.sock     # custom socket
#   ./tools/push_demo_scene.sh --stream                  # also push streaming updates

set -euo pipefail

SOCKET="${PETALTONGUE_SOCKET:-/tmp/petaltongue.sock}"
STREAM=false

CYAN='\033[0;36m'
GREEN='\033[0;32m'
NC='\033[0m'

log() { echo -e "${CYAN}[push_demo]${NC} $*"; }
ok()  { echo -e "${GREEN}[push_demo]${NC} $*"; }

while [[ $# -gt 0 ]]; do
    case "$1" in
        --socket) SOCKET="$2"; shift 2 ;;
        --stream) STREAM=true; shift ;;
        *)        echo "Unknown: $1"; exit 1 ;;
    esac
done

send_rpc() {
    local method="$1"
    local params="$2"
    local payload
    payload=$(printf '{"jsonrpc":"2.0","method":"%s","params":%s,"id":1}' "$method" "$params")
    if [[ -S "$SOCKET" ]]; then
        echo "$payload" | socat - UNIX-CONNECT:"$SOCKET" 2>/dev/null || \
        echo "$payload" | nc -U "$SOCKET" 2>/dev/null || \
        log "warning: could not connect to $SOCKET (is petaltongue running?)"
    else
        log "socket not found: $SOCKET — writing to stdout for reference"
        echo "$payload"
    fi
}

# === Scene 1: Grammar of Graphics — scatter plot ===
log "pushing Grammar of Graphics scatter scene..."
send_rpc "visualization.render.grammar" '{
  "session_id": "demo-grammar",
  "grammar": {
    "data_source": "inline",
    "geometry": "Point",
    "variables": [
      {"name": "x", "field": "reaction_time", "role": "X"},
      {"name": "y", "field": "accuracy", "role": "Y"},
      {"name": "size", "field": "score", "role": "Size"}
    ],
    "scales": [],
    "coordinate": "Cartesian",
    "facets": null,
    "aesthetics": []
  },
  "data": [
    {"reaction_time": 50, "accuracy": 200, "score": 10},
    {"reaction_time": 120, "accuracy": 350, "score": 15},
    {"reaction_time": 200, "accuracy": 280, "score": 20},
    {"reaction_time": 280, "accuracy": 400, "score": 12},
    {"reaction_time": 350, "accuracy": 320, "score": 25}
  ],
  "modality": "svg",
  "validate_tufte": false,
  "domain": "game"
}'
ok "grammar scatter scene pushed"

# === Scene 2: Render scene with shapes (Manim-style topology) ===
log "pushing Manim-style topology scene..."
send_rpc "visualization.render.scene" '{
  "session_id": "demo-topology",
  "scene": {
    "nodes": {
      "root": {"id":"root","transform":{"a":1.0,"b":0.0,"c":0.0,"d":1.0,"tx":0.0,"ty":0.0},"primitives":[],"children":["title","beardog","songbird","petal","barracuda","conn"],"visible":true,"opacity":1.0,"label":null,"data_source":null},
      "title": {"id":"title","transform":{"a":1.0,"b":0.0,"c":0.0,"d":1.0,"tx":0.0,"ty":0.0},"primitives":[{"Text":{"x":200.0,"y":20.0,"content":"NUCLEUS Live","font_size":22.0,"color":{"r":0.9,"g":0.95,"b":1.0,"a":1.0},"anchor":"TopCenter","bold":true,"italic":false,"data_id":null}}],"children":[],"visible":true,"opacity":1.0,"label":"title","data_source":null},
      "beardog": {"id":"beardog","transform":{"a":1.0,"b":0.0,"c":0.0,"d":1.0,"tx":0.0,"ty":0.0},"primitives":[{"Point":{"x":200.0,"y":100.0,"radius":22.0,"fill":{"r":0.2,"g":0.75,"b":0.3,"a":1.0},"stroke":null,"data_id":"beardog"}},{"Text":{"x":200.0,"y":130.0,"content":"beardog","font_size":12.0,"color":{"r":0.8,"g":0.8,"b":0.8,"a":1.0},"anchor":"TopCenter","bold":false,"italic":false,"data_id":null}}],"children":[],"visible":true,"opacity":1.0,"label":"beardog","data_source":null},
      "songbird": {"id":"songbird","transform":{"a":1.0,"b":0.0,"c":0.0,"d":1.0,"tx":0.0,"ty":0.0},"primitives":[{"Point":{"x":80.0,"y":220.0,"radius":18.0,"fill":{"r":0.3,"g":0.5,"b":0.85,"a":1.0},"stroke":null,"data_id":"songbird"}},{"Text":{"x":80.0,"y":248.0,"content":"songbird","font_size":12.0,"color":{"r":0.8,"g":0.8,"b":0.8,"a":1.0},"anchor":"TopCenter","bold":false,"italic":false,"data_id":null}}],"children":[],"visible":true,"opacity":1.0,"label":"songbird","data_source":null},
      "petal": {"id":"petal","transform":{"a":1.0,"b":0.0,"c":0.0,"d":1.0,"tx":0.0,"ty":0.0},"primitives":[{"Point":{"x":320.0,"y":220.0,"radius":18.0,"fill":{"r":0.85,"g":0.4,"b":0.6,"a":1.0},"stroke":null,"data_id":"petaltongue"}},{"Text":{"x":320.0,"y":248.0,"content":"petalTongue","font_size":12.0,"color":{"r":0.8,"g":0.8,"b":0.8,"a":1.0},"anchor":"TopCenter","bold":false,"italic":false,"data_id":null}}],"children":[],"visible":true,"opacity":1.0,"label":"petalTongue","data_source":null},
      "barracuda": {"id":"barracuda","transform":{"a":1.0,"b":0.0,"c":0.0,"d":1.0,"tx":0.0,"ty":0.0},"primitives":[{"Point":{"x":200.0,"y":220.0,"radius":18.0,"fill":{"r":0.9,"g":0.65,"b":0.2,"a":1.0},"stroke":null,"data_id":"barracuda"}},{"Text":{"x":200.0,"y":248.0,"content":"barraCuda","font_size":12.0,"color":{"r":0.8,"g":0.8,"b":0.8,"a":1.0},"anchor":"TopCenter","bold":false,"italic":false,"data_id":null}}],"children":[],"visible":true,"opacity":1.0,"label":"barraCuda","data_source":null},
      "conn": {"id":"conn","transform":{"a":1.0,"b":0.0,"c":0.0,"d":1.0,"tx":0.0,"ty":0.0},"primitives":[{"Line":{"points":[[200.0,100.0],[80.0,220.0]],"stroke":{"color":{"r":0.4,"g":0.65,"b":0.4,"a":0.6},"width":2.0,"cap":"Butt","join":"Miter"},"closed":false,"data_id":null}},{"Line":{"points":[[200.0,100.0],[200.0,220.0]],"stroke":{"color":{"r":0.4,"g":0.65,"b":0.4,"a":0.6},"width":2.0,"cap":"Butt","join":"Miter"},"closed":false,"data_id":null}},{"Line":{"points":[[200.0,100.0],[320.0,220.0]],"stroke":{"color":{"r":0.4,"g":0.65,"b":0.4,"a":0.6},"width":2.0,"cap":"Butt","join":"Miter"},"closed":false,"data_id":null}}],"children":[],"visible":true,"opacity":1.0,"label":"connections","data_source":null}
    },
    "root_id": "root"
  }
}'
ok "topology scene pushed"

# === Optional: Streaming updates ===
if $STREAM; then
    log "pushing streaming data updates (5 frames)..."
    for i in $(seq 1 5); do
        tick_ms=$(echo "scale=2; 0.5 + ($i * 0.1)" | bc)
        send_rpc "visualization.render.stream" "{
          \"session_id\": \"demo-grammar\",
          \"updates\": [
            {\"binding\": \"tick_ms\", \"values\": [{\"frame\": $((5+i)), \"tick_ms\": $tick_ms}]}
          ]
        }"
        ok "stream frame $i pushed (tick_ms=$tick_ms)"
        sleep 0.5
    done
fi

ok "all demo scenes pushed to $SOCKET"
