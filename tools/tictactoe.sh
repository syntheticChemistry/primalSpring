#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
# Tic-tac-toe via petalTongue IPC — renders the board as a SceneGraph.
#
# Usage:
#   ./tictactoe.sh render <state>   Render board (state = 9 chars: .XO)
#   ./tictactoe.sh move <state> <cell> <mark>  Apply move, render, print new state
#   ./tictactoe.sh check <state>    Check win/draw, print result
#   ./tictactoe.sh ai <state>       Compute AI (O) move, print cell number
#
# State is a 9-character string, left-to-right top-to-bottom:
#   "........." = empty board
#   "X...O...." = X top-left, O center
#
# Requires: SOCKET env var pointing to petalTongue UDS.

set -euo pipefail

SOCKET="${SOCKET:-/tmp/petaltongue-ttt.sock}"

send_rpc() {
    local method="$1" params="$2"
    local id=$((RANDOM % 9999 + 1))
    printf '{"jsonrpc":"2.0","method":"%s","params":%s,"id":%d}' "$method" "$params" "$id" \
        | timeout 5 socat - "UNIX-CONNECT:$SOCKET" 2>/dev/null
}

# Grid geometry
GRID_X0=80   # left edge
GRID_Y0=80   # top edge
CELL=100     # cell size
LINE_W=3

cell_center() {
    local idx=$1
    local row=$((idx / 3))
    local col=$((idx % 3))
    local cx=$(( GRID_X0 + col * CELL + CELL / 2 ))
    local cy=$(( GRID_Y0 + row * CELL + CELL / 2 ))
    echo "$cx $cy"
}

render_board() {
    local state="$1"
    local status="${2:-Your turn (X)}"

    # Build node JSON for each cell
    local cell_nodes=""
    local cell_ids=""
    for i in $(seq 0 8); do
        local ch="${state:$i:1}"
        read -r cx cy <<< "$(cell_center "$i")"
        local prims=""

        # Clickable background rect for hit testing
        local rx=$((cx - CELL/2 + 4))
        local ry=$((cy - CELL/2 + 4))
        local rw=$((CELL - 8))
        local rh=$((CELL - 8))
        prims="{\"Rect\":{\"x\":${rx}.0,\"y\":${ry}.0,\"width\":${rw}.0,\"height\":${rh}.0,\"fill\":{\"r\":0.15,\"g\":0.15,\"b\":0.2,\"a\":0.3},\"stroke\":null,\"corner_radius\":8.0,\"data_id\":\"cell-${i}\"}}"

        if [ "$ch" = "X" ]; then
            prims="${prims},{\"Text\":{\"x\":${cx}.0,\"y\":$((cy - 20)).0,\"content\":\"X\",\"font_size\":48.0,\"color\":{\"r\":0.3,\"g\":0.85,\"b\":0.4,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":true,\"italic\":false,\"data_id\":null}}"
        elif [ "$ch" = "O" ]; then
            prims="${prims},{\"Text\":{\"x\":${cx}.0,\"y\":$((cy - 20)).0,\"content\":\"O\",\"font_size\":48.0,\"color\":{\"r\":0.85,\"g\":0.35,\"b\":0.55,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":true,\"italic\":false,\"data_id\":null}}"
        fi

        local node_id="cell-${i}"
        local node="{\"id\":\"${node_id}\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[${prims}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":\"${node_id}\",\"data_source\":null}"

        if [ -n "$cell_nodes" ]; then
            cell_nodes="${cell_nodes},\"${node_id}\":${node}"
            cell_ids="${cell_ids},\"${node_id}\""
        else
            cell_nodes="\"${node_id}\":${node}"
            cell_ids="\"${node_id}\""
        fi
    done

    # Grid lines
    local gx1=$((GRID_X0 + CELL))
    local gx2=$((GRID_X0 + 2 * CELL))
    local gy1=$((GRID_Y0 + CELL))
    local gy2=$((GRID_Y0 + 2 * CELL))
    local gxe=$((GRID_X0 + 3 * CELL))
    local gye=$((GRID_Y0 + 3 * CELL))
    local stroke="{\"color\":{\"r\":0.5,\"g\":0.55,\"b\":0.65,\"a\":0.8},\"width\":${LINE_W}.0,\"cap\":\"Round\",\"join\":\"Miter\"}"

    local grid_prims=""
    grid_prims="{\"Line\":{\"points\":[[${gx1}.0,${GRID_Y0}.0],[${gx1}.0,${gye}.0]],\"stroke\":${stroke},\"closed\":false,\"data_id\":null}}"
    grid_prims="${grid_prims},{\"Line\":{\"points\":[[${gx2}.0,${GRID_Y0}.0],[${gx2}.0,${gye}.0]],\"stroke\":${stroke},\"closed\":false,\"data_id\":null}}"
    grid_prims="${grid_prims},{\"Line\":{\"points\":[[${GRID_X0}.0,${gy1}.0],[${gxe}.0,${gy1}.0]],\"stroke\":${stroke},\"closed\":false,\"data_id\":null}}"
    grid_prims="${grid_prims},{\"Line\":{\"points\":[[${GRID_X0}.0,${gy2}.0],[${gxe}.0,${gy2}.0]],\"stroke\":${stroke},\"closed\":false,\"data_id\":null}}"

    local grid_node="{\"id\":\"grid\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[${grid_prims}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":\"grid\",\"data_source\":null}"

    # Title
    local title_node="{\"id\":\"title\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[{\"Text\":{\"x\":$((GRID_X0 + CELL * 3 / 2)).0,\"y\":20.0,\"content\":\"Tic-Tac-Toe\",\"font_size\":28.0,\"color\":{\"r\":0.95,\"g\":0.95,\"b\":1.0,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":true,\"italic\":false,\"data_id\":null}}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":\"title\",\"data_source\":null}"

    # Status line
    local status_node="{\"id\":\"status\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[{\"Text\":{\"x\":$((GRID_X0 + CELL * 3 / 2)).0,\"y\":$((GRID_Y0 + 3 * CELL + 20)).0,\"content\":\"${status}\",\"font_size\":18.0,\"color\":{\"r\":0.8,\"g\":0.8,\"b\":0.85,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":false,\"italic\":false,\"data_id\":null}}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":\"status\",\"data_source\":null}"

    # Root
    local root="{\"id\":\"root\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[],\"children\":[\"title\",\"grid\",${cell_ids},\"status\"],\"visible\":true,\"opacity\":1.0,\"label\":null,\"data_source\":null}"

    local scene="{\"nodes\":{\"root\":${root},\"title\":${title_node},\"grid\":${grid_node},${cell_nodes},\"status\":${status_node}},\"root_id\":\"root\"}"

    send_rpc "visualization.render.scene" "{\"session_id\":\"tictactoe\",\"scene\":${scene}}"
}

check_winner() {
    local s="$1"
    local lines="012 345 678 036 147 258 048 246"
    for line in $lines; do
        local a="${s:${line:0:1}:1}"
        local b="${s:${line:1:1}:1}"
        local c="${s:${line:2:1}:1}"
        if [ "$a" != "." ] && [ "$a" = "$b" ] && [ "$b" = "$c" ]; then
            echo "$a"
            return
        fi
    done
    if [[ "$s" != *"."* ]]; then
        echo "draw"
        return
    fi
    echo ""
}

ai_move() {
    local s="$1"
    # Simple AI: try to win, block, or pick best available
    local lines="012 345 678 036 147 258 048 246"

    # Try to win
    for line in $lines; do
        local i0=${line:0:1} i1=${line:1:1} i2=${line:2:1}
        local a="${s:$i0:1}" b="${s:$i1:1}" c="${s:$i2:1}"
        if [ "$a" = "O" ] && [ "$b" = "O" ] && [ "$c" = "." ]; then echo "$i2"; return; fi
        if [ "$a" = "O" ] && [ "$c" = "O" ] && [ "$b" = "." ]; then echo "$i1"; return; fi
        if [ "$b" = "O" ] && [ "$c" = "O" ] && [ "$a" = "." ]; then echo "$i0"; return; fi
    done

    # Try to block
    for line in $lines; do
        local i0=${line:0:1} i1=${line:1:1} i2=${line:2:1}
        local a="${s:$i0:1}" b="${s:$i1:1}" c="${s:$i2:1}"
        if [ "$a" = "X" ] && [ "$b" = "X" ] && [ "$c" = "." ]; then echo "$i2"; return; fi
        if [ "$a" = "X" ] && [ "$c" = "X" ] && [ "$b" = "." ]; then echo "$i1"; return; fi
        if [ "$b" = "X" ] && [ "$c" = "X" ] && [ "$a" = "." ]; then echo "$i0"; return; fi
    done

    # Prefer center, then corners, then edges
    for cell in 4 0 2 6 8 1 3 5 7; do
        if [ "${s:$cell:1}" = "." ]; then
            echo "$cell"
            return
        fi
    done
}

apply_move() {
    local s="$1" cell="$2" mark="$3"
    echo "${s:0:$cell}${mark}${s:$((cell+1))}"
}

case "${1:-}" in
    render)
        render_board "${2:-.........}" "${3:-Your turn (X)}"
        ;;
    move)
        local state="${2:-.........}"
        local cell="$3"
        local mark="$4"
        state=$(apply_move "$state" "$cell" "$mark")
        render_board "$state" "Move: ${mark} at cell ${cell}"
        echo "$state"
        ;;
    check)
        check_winner "${2:-.........}"
        ;;
    ai)
        ai_move "${2:-.........}"
        ;;
    *)
        echo "Usage: $0 {render|move|check|ai} [args...]"
        exit 1
        ;;
esac
