#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# ttt_composition.sh — Tic-Tac-Toe via full NUCLEUS composition
#
# Domain-specific game logic built on nucleus_composition_lib.sh.
# The library handles all NUCLEUS wiring (discovery, transport, DAG,
# ledger, braids, interaction, visualization). This script handles
# only TTT rules, rendering, and the game loop.
#
# Usage:
#   ./tools/ttt_composition.sh              # auto-discover sockets
#   FAMILY_ID=ttt ./tools/ttt_composition.sh  # explicit family

set -euo pipefail

# ── Configuration ─────────────────────────────────────────────────────

COMPOSITION_NAME="ttt"
REQUIRED_CAPS="visualization security"
OPTIONAL_CAPS="compute tensor dag ledger attribution"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/nucleus_composition_lib.sh"

# ── TTT Game State ────────────────────────────────────────────────────

BOARD="........."
TURN="X"
MOVE_NUM=0
GAME_OVER=false

GRID_X0=80
GRID_Y0=80
CELL=100
LINE_W=3

cell_center() {
    local idx=$1
    local row=$((idx / 3))
    local col=$((idx % 3))
    echo "$(( GRID_X0 + col * CELL + CELL / 2 )) $(( GRID_Y0 + row * CELL + CELL / 2 ))"
}

# ── TTT Hit Testing (overrides lib default) ───────────────────────────

hit_test_fn() {
    local px="$1" py="$2"
    px="${px%.*}"
    py="${py%.*}"
    if (( px < GRID_X0 || px >= GRID_X0 + 3 * CELL )); then echo -1; return; fi
    if (( py < GRID_Y0 || py >= GRID_Y0 + 3 * CELL )); then echo -1; return; fi
    local ix=$(( (px - GRID_X0) / CELL ))
    local iy=$(( (py - GRID_Y0) / CELL ))
    echo $(( iy * 3 + ix ))
}

# ── TTT Domain: DAG Helpers ───────────────────────────────────────────

ttt_dag_create_session() {
    dag_create_session "ttt" \
        "[{\"key\":\"board\",\"value\":\".........\"},{\"key\":\"move_num\",\"value\":\"0\"}]"
    if [[ -n "$GENESIS_VERTEX" ]]; then
        STATE_STACK=(".........")
        ttt_braid_record "genesis" "-" "........." "$GENESIS_VERTEX" "genesis" "0"
    fi
}

ttt_dag_append_state() {
    local player="$1" cell="$2" board_after="$3" input_type="${4:-unknown}" hover_moves="${5:-0}"

    local meta
    meta="[{\"key\":\"board\",\"value\":\"$board_after\"},{\"key\":\"player\",\"value\":\"$player\"}"
    meta="${meta},{\"key\":\"cell\",\"value\":\"$cell\"},{\"key\":\"move_num\",\"value\":\"$MOVE_NUM\"}"
    meta="${meta},{\"key\":\"input_type\",\"value\":\"$input_type\"}"
    meta="${meta},{\"key\":\"hover_moves\",\"value\":\"$hover_moves\"}]"

    dag_append_event "ttt" "move" "$board_after" "$meta" "$input_type" "$hover_moves"
}

ttt_dag_undo() {
    if [[ ${#VERTEX_STACK[@]} -le 1 ]]; then
        warn "nothing to undo — at genesis"
        return 1
    fi

    local undo_count=1
    if [[ "$TURN" = "X" ]] && [[ ${#VERTEX_STACK[@]} -ge 3 ]]; then
        undo_count=2
    fi

    for ((i=0; i<undo_count; i++)); do
        if [[ ${#VERTEX_STACK[@]} -le 1 ]]; then break; fi
        unset 'VERTEX_STACK[${#VERTEX_STACK[@]}-1]'
        unset 'STATE_STACK[${#STATE_STACK[@]}-1]'
        unset 'INPUT_TYPE_STACK[${#INPUT_TYPE_STACK[@]}-1]'
        unset 'HOVER_COUNT_STACK[${#HOVER_COUNT_STACK[@]}-1]'
        MOVE_NUM=$((MOVE_NUM > 0 ? MOVE_NUM - 1 : 0))
    done

    local top=$((${#VERTEX_STACK[@]} - 1))
    CURRENT_VERTEX="${VERTEX_STACK[$top]}"
    BOARD="${STATE_STACK[$top]}"
    TURN="X"
    ok "undo → ${CURRENT_VERTEX:0:16}... board=$BOARD (move $MOVE_NUM)"
    return 0
}

# ── TTT Domain: Braid Wrapper ─────────────────────────────────────────

ttt_braid_record() {
    local event_name="$1" player="$2" board="$3" vertex_id="$4"
    local input_type="${5:-unknown}" hover_moves="${6:-0}"

    local custom
    custom=$(printf '{"event":"%s","player":"%s","board":"%s","vertex":"%s"' \
        "$event_name" "$player" "$board" "${vertex_id:0:16}")

    braid_record "$event_name" "application/x-ttt-state" "$board" "${custom}}" \
        "$input_type" "$hover_moves"
}

# ── TTT Domain: Ledger Commit ─────────────────────────────────────────

ttt_ledger_commit_game_line() {
    local result="$1"
    cap_available ledger || return
    [[ -n "$SPINE_ID" ]] || return

    local committed=0
    if [[ ${#VERTEX_STACK[@]} -gt 0 ]]; then
        for ((i=0; i<${#VERTEX_STACK[@]}; i++)); do
            local vid="${VERTEX_STACK[$i]}"
            local brd="${STATE_STACK[$i]}"
            local itype="${INPUT_TYPE_STACK[$i]:-unknown}"
            local hcount="${HOVER_COUNT_STACK[$i]:-0}"
            local move_label="genesis"
            [[ $i -gt 0 ]] && move_label="move-$i"

            local game_data
            game_data=$(printf '{"vertex":"%s","board":"%s","step":%d,"total":%d,"input_type":"%s","hover_moves":%d}' \
                "${vid:0:16}" "$brd" "$i" "${#VERTEX_STACK[@]}" "$itype" "$hcount")
            if ledger_append_entry "ttt-${move_label}" "$game_data"; then
                committed=$((committed+1))
            fi
        done
    fi

    local result_data
    result_data=$(printf '{"result":"%s","moves":%d,"board":"%s","dag_session":"%s","genesis":"%s"}' \
        "$result" "$MOVE_NUM" "$BOARD" "${DAG_SESSION:-none}" "${GENESIS_VERTEX:0:16}")
    ledger_append_entry "ttt-result" "$result_data"

    if ledger_seal_spine; then
        ok "game sealed: $committed entries → spine $SPINE_ID"
    else
        ok "game committed: $committed entries → spine $SPINE_ID (unsealed)"
    fi
}

# ── TTT Domain: Branching Visualization ───────────────────────────────

show_branches() {
    if [[ -z "$CURRENT_VERTEX" ]] || ! cap_available dag; then
        render_board "$BOARD" "DAG offline — no branches" "$HOVER_CELL"
        return
    fi
    local resp
    resp=$(dag_get_children "$CURRENT_VERTEX")
    local children
    children=$(echo "$resp" | grep -oP '"result"\s*:\s*\[\K[^\]]*' | head -1 || true)
    local count=0
    if [[ -n "$children" ]]; then
        count=$(echo "$children" | tr ',' '\n' | grep -c '"' || true)
    fi

    local frontier_resp
    frontier_resp=$(dag_get_frontier)
    local frontier_count=0
    local frontier_ids
    frontier_ids=$(echo "$frontier_resp" | grep -oP '"result"\s*:\s*\[\K[^\]]*' | head -1 || true)
    if [[ -n "$frontier_ids" ]]; then
        frontier_count=$(echo "$frontier_ids" | tr ',' '\n' | grep -c '"' || true)
    fi

    local depth=${#VERTEX_STACK[@]}
    local status_msg="Branches: $count from here | Depth: $depth | Timelines: $frontier_count"
    render_board "$BOARD" "$status_msg" "$HOVER_CELL"
    ok "branches=$count depth=$depth frontier=$frontier_count vertex=${CURRENT_VERTEX:0:16}"
}

show_tree_view() {
    if [[ -z "$GENESIS_VERTEX" ]] || ! cap_available dag; then
        render_board "$BOARD" "DAG offline — no tree" "$HOVER_CELL"
        return
    fi
    build_tree_scene
}

build_tree_scene() {
    local -a queue=("$GENESIS_VERTEX")
    local -A visited=()
    local -A node_x=()
    local -A node_y=()
    local -A node_label=()
    local all_nodes=""
    local all_ids=""
    local next_x=50
    local level_y=40
    local x_gap=90
    local y_gap=60

    local idx=0
    while [[ ${#queue[@]} -gt 0 ]]; do
        local vid="${queue[0]}"
        queue=("${queue[@]:1}")
        local short="${vid:0:8}"
        if [[ -n "${visited[$short]+_}" ]]; then continue; fi
        visited[$short]=1

        local depth=0
        for ((d=0; d<${#VERTEX_STACK[@]}; d++)); do
            if [[ "${VERTEX_STACK[$d]}" = "$vid" ]]; then
                depth=$d
                break
            fi
        done

        local nx=$next_x
        local ny=$((level_y + depth * y_gap))
        next_x=$((next_x + x_gap))
        node_x[$short]=$nx
        node_y[$short]=$ny

        local board_str=""
        for ((d=0; d<${#VERTEX_STACK[@]}; d++)); do
            if [[ "${VERTEX_STACK[$d]}" = "$vid" ]]; then
                board_str="${STATE_STACK[$d]}"
                break
            fi
        done

        local is_current="false"
        [[ "$vid" = "$CURRENT_VERTEX" ]] && is_current="true"
        local fill_r=0.2 fill_g=0.2 fill_b=0.3
        if [[ "$is_current" = "true" ]]; then
            fill_r=0.15; fill_g=0.5; fill_b=0.3
        fi

        local label="${short}"
        [[ -n "$board_str" ]] && label="${board_str}"
        [[ "$vid" = "$GENESIS_VERTEX" ]] && label="GEN"

        local nid="tv-${idx}"
        local node_json="{\"id\":\"${nid}\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[{\"Rect\":{\"x\":$((nx-35)).0,\"y\":${ny}.0,\"width\":70.0,\"height\":40.0,\"fill\":{\"r\":${fill_r},\"g\":${fill_g},\"b\":${fill_b},\"a\":0.8},\"stroke\":null,\"corner_radius\":6.0,\"data_id\":\"${nid}\"}},{\"Text\":{\"x\":${nx}.0,\"y\":$((ny+5)).0,\"content\":\"${label}\",\"font_size\":10.0,\"color\":{\"r\":0.9,\"g\":0.9,\"b\":0.95,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":${is_current},\"italic\":false,\"data_id\":null}}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":null,\"data_source\":null}"

        if [[ -n "$all_nodes" ]]; then
            all_nodes="${all_nodes},\"${nid}\":${node_json}"
            all_ids="${all_ids},\"${nid}\""
        else
            all_nodes="\"${nid}\":${node_json}"
            all_ids="\"${nid}\""
        fi
        node_label[$short]="$nid"
        idx=$((idx+1))

        local children_resp
        children_resp=$(dag_get_children "$vid")
        local child_ids
        child_ids=$(echo "$children_resp" | grep -oP '"[a-f0-9]{64}"' || true)
        while IFS= read -r child_hex; do
            child_hex="${child_hex//\"/}"
            [[ -z "$child_hex" ]] && continue
            queue+=("$child_hex")
        done <<< "$child_ids"

        if [[ $idx -gt 30 ]]; then break; fi
    done

    local title_nid="tv-title"
    local title_node="{\"id\":\"${title_nid}\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[{\"Text\":{\"x\":230.0,\"y\":10.0,\"content\":\"Game State Tree (${idx} vertices)\",\"font_size\":18.0,\"color\":{\"r\":0.95,\"g\":0.95,\"b\":1.0,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":true,\"italic\":false,\"data_id\":null}}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":null,\"data_source\":null}"
    all_nodes="\"${title_nid}\":${title_node},${all_nodes}"
    all_ids="\"${title_nid}\",${all_ids}"

    local root="{\"id\":\"root\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[],\"children\":[${all_ids}],\"visible\":true,\"opacity\":1.0,\"label\":null,\"data_source\":null}"
    local scene="{\"nodes\":{\"root\":${root},${all_nodes}},\"root_id\":\"root\"}"
    push_scene "ttt-tree" "$scene"
    ok "tree view: $idx vertices rendered"
}

# ── TTT Domain: Compute / AI ──────────────────────────────────────────

compute_ai_move() {
    if cap_available tensor; then
        local tensor_data=""
        for i in $(seq 0 8); do
            local ch="${BOARD:$i:1}"
            case "$ch" in
                X) tensor_data="${tensor_data}1.0," ;;
                O) tensor_data="${tensor_data}-1.0," ;;
                *) tensor_data="${tensor_data}0.0," ;;
            esac
        done
        tensor_data="${tensor_data%,}"

        local create_resp
        create_resp=$(send_rpc "$(cap_socket tensor)" "tensor.create" \
            "{\"shape\":[3,3],\"data\":[$tensor_data]}")
        local tensor_id
        tensor_id=$(echo "$create_resp" | grep -oP '"tensor_id"\s*:\s*"\K[^"]+' | head -1 || true)
        if [[ -n "$tensor_id" ]]; then
            ok "tensor: board encoded as $tensor_id" >&2
            local matmul_resp
            matmul_resp=$(send_rpc "$(cap_socket tensor)" "tensor.matmul" \
                "{\"lhs_id\":\"$tensor_id\",\"rhs_id\":\"$tensor_id\"}")
            if echo "$matmul_resp" | grep -q '"result_id"'; then
                ok "tensor: matmul computed (board evaluation)" >&2
            fi
        else
            warn "tensor.create returned: $(echo "$create_resp" | head -c 80)" >&2
        fi
    fi
    local_ai_move "$BOARD"
}

local_ai_move() {
    local s="$1"
    local lines="012 345 678 036 147 258 048 246"

    for line in $lines; do
        local i0=${line:0:1} i1=${line:1:1} i2=${line:2:1}
        local a="${s:$i0:1}" b="${s:$i1:1}" c="${s:$i2:1}"
        if [ "$a" = "O" ] && [ "$b" = "O" ] && [ "$c" = "." ]; then echo "$i2"; return; fi
        if [ "$a" = "O" ] && [ "$c" = "O" ] && [ "$b" = "." ]; then echo "$i1"; return; fi
        if [ "$b" = "O" ] && [ "$c" = "O" ] && [ "$a" = "." ]; then echo "$i0"; return; fi
    done

    for line in $lines; do
        local i0=${line:0:1} i1=${line:1:1} i2=${line:2:1}
        local a="${s:$i0:1}" b="${s:$i1:1}" c="${s:$i2:1}"
        if [ "$a" = "X" ] && [ "$b" = "X" ] && [ "$c" = "." ]; then echo "$i2"; return; fi
        if [ "$a" = "X" ] && [ "$c" = "X" ] && [ "$b" = "." ]; then echo "$i1"; return; fi
        if [ "$b" = "X" ] && [ "$c" = "X" ] && [ "$a" = "." ]; then echo "$i0"; return; fi
    done

    for cell in 4 0 2 6 8 1 3 5 7; do
        if [ "${s:$cell:1}" = "." ]; then
            echo "$cell"
            return
        fi
    done
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

apply_move() {
    local s="$1" cell="$2" mark="$3"
    echo "${s:0:$cell}${mark}${s:$((cell+1))}"
}

# ── TTT Domain: Render Board ──────────────────────────────────────────

render_board() {
    local state="$1"
    local status="${2:-Your turn (X)}"
    local hover="${3:--1}"

    local cell_nodes=""
    local cell_ids=""
    for i in $(seq 0 8); do
        local ch="${state:$i:1}"
        read -r cx cy <<< "$(cell_center "$i")"
        local prims=""

        local rx=$((cx - CELL/2 + 4))
        local ry=$((cy - CELL/2 + 4))
        local rw=$((CELL - 8))
        local rh=$((CELL - 8))

        local bg_r=0.15 bg_g=0.15 bg_b=0.2 bg_a=0.3
        if [[ "$i" -eq "$hover" ]] && [[ "$ch" = "." ]]; then
            bg_r=0.25; bg_g=0.30; bg_b=0.4; bg_a=0.5
        fi

        prims="{\"Rect\":{\"x\":${rx}.0,\"y\":${ry}.0,\"width\":${rw}.0,\"height\":${rh}.0,\"fill\":{\"r\":${bg_r},\"g\":${bg_g},\"b\":${bg_b},\"a\":${bg_a}},\"stroke\":null,\"corner_radius\":8.0,\"data_id\":\"cell-${i}\"}}"

        if [ "$ch" = "X" ]; then
            prims="${prims},{\"Text\":{\"x\":${cx}.0,\"y\":$((cy - 20)).0,\"content\":\"X\",\"font_size\":48.0,\"color\":{\"r\":0.3,\"g\":0.85,\"b\":0.4,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":true,\"italic\":false,\"data_id\":null}}"
        elif [ "$ch" = "O" ]; then
            prims="${prims},{\"Text\":{\"x\":${cx}.0,\"y\":$((cy - 20)).0,\"content\":\"O\",\"font_size\":48.0,\"color\":{\"r\":0.85,\"g\":0.35,\"b\":0.55,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":true,\"italic\":false,\"data_id\":null}}"
        elif [[ "$i" -eq "$hover" ]]; then
            prims="${prims},{\"Text\":{\"x\":${cx}.0,\"y\":$((cy - 20)).0,\"content\":\"X\",\"font_size\":48.0,\"color\":{\"r\":0.3,\"g\":0.85,\"b\":0.4,\"a\":0.25},\"anchor\":\"TopCenter\",\"bold\":true,\"italic\":false,\"data_id\":null}}"
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

    local title_node="{\"id\":\"title\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[{\"Text\":{\"x\":$((GRID_X0 + CELL * 3 / 2)).0,\"y\":20.0,\"content\":\"NUCLEUS Tic-Tac-Toe\",\"font_size\":28.0,\"color\":{\"r\":0.95,\"g\":0.95,\"b\":1.0,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":true,\"italic\":false,\"data_id\":null}}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":\"title\",\"data_source\":null}"

    local info_text="[NUCLEUS] beardog:✓  petal:✓"
    cap_available compute && info_text="$info_text  compute:✓" || info_text="$info_text  compute:✗"
    cap_available dag && info_text="$info_text  dag:✓" || info_text="$info_text  dag:✗"
    cap_available ledger && info_text="$info_text  ledger:✓" || info_text="$info_text  ledger:✗"
    cap_available attribution && info_text="$info_text  braid:✓" || info_text="$info_text  braid:✗"

    local info_node="{\"id\":\"info\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[{\"Text\":{\"x\":$((GRID_X0 + CELL * 3 / 2)).0,\"y\":55.0,\"content\":\"${info_text}\",\"font_size\":11.0,\"color\":{\"r\":0.5,\"g\":0.7,\"b\":0.5,\"a\":0.8},\"anchor\":\"TopCenter\",\"bold\":false,\"italic\":false,\"data_id\":null}}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":\"info\",\"data_source\":null}"

    local dag_info=""
    if [[ -n "$CURRENT_VERTEX" ]]; then
        dag_info="  DAG: ${CURRENT_VERTEX:0:8}.. d=${#VERTEX_STACK[@]}"
    fi
    local keys_node="{\"id\":\"keys\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[{\"Text\":{\"x\":$((GRID_X0 + CELL * 3 / 2)).0,\"y\":$((GRID_Y0 + 3 * CELL + 50)).0,\"content\":\"1-9 place  U undo  B branches  T tree  R restart  Q quit${dag_info}\",\"font_size\":11.0,\"color\":{\"r\":0.45,\"g\":0.45,\"b\":0.55,\"a\":0.7},\"anchor\":\"TopCenter\",\"bold\":false,\"italic\":false,\"data_id\":null}}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":\"keys\",\"data_source\":null}"

    local status_node="{\"id\":\"status\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[{\"Text\":{\"x\":$((GRID_X0 + CELL * 3 / 2)).0,\"y\":$((GRID_Y0 + 3 * CELL + 20)).0,\"content\":\"${status}\",\"font_size\":18.0,\"color\":{\"r\":0.8,\"g\":0.8,\"b\":0.85,\"a\":1.0},\"anchor\":\"TopCenter\",\"bold\":false,\"italic\":false,\"data_id\":null}}],\"children\":[],\"visible\":true,\"opacity\":1.0,\"label\":\"status\",\"data_source\":null}"

    local root="{\"id\":\"root\",\"transform\":{\"a\":1.0,\"b\":0.0,\"c\":0.0,\"d\":1.0,\"tx\":0.0,\"ty\":0.0},\"primitives\":[],\"children\":[\"title\",\"info\",\"grid\",${cell_ids},\"status\",\"keys\"],\"visible\":true,\"opacity\":1.0,\"label\":null,\"data_source\":null}"

    local scene="{\"nodes\":{\"root\":${root},\"title\":${title_node},\"info\":${info_node},\"grid\":${grid_node},${cell_nodes},\"status\":${status_node},\"keys\":${keys_node}},\"root_id\":\"root\"}"

    push_scene "tictactoe" "$scene"
}

# ── TTT Domain: Cell Click Extraction ─────────────────────────────────

extract_cell_click() {
    local resp="$1"
    local cell_id
    cell_id=$(echo "$resp" | grep -oP '"target"\s*:\s*"cell-\K[0-9]' | head -1 || true)
    if [[ -z "$cell_id" ]]; then
        cell_id=$(echo "$resp" | grep -oP '"data_id"\s*:\s*"cell-\K[0-9]' | head -1 || true)
    fi
    if [[ -z "$cell_id" ]]; then
        cell_id=$(echo "$resp" | grep -oP 'cell-\K[0-9]' | head -1 || true)
    fi
    echo "$cell_id"
}

# ── TTT Game Loop ─────────────────────────────────────────────────────

game_loop() {
    log "╔══════════════════════════════════════════════════╗"
    log "║     NUCLEUS Tic-Tac-Toe — Branching States    ║"
    log "╠══════════════════════════════════════════════════╣"
    log "║  You are X. Click or press 1-9.                ║"
    log "║  U undo  B branches  T tree  R restart  Q quit ║"
    log "║  DAG: rhizoCrypt  Ledger: loamSpine             ║"
    log "║  Provenance: sweetGrass braids                  ║"
    log "║  Every state is a DAG vertex. Undo = branch.    ║"
    log "╚══════════════════════════════════════════════════╝"

    composition_startup "NUCLEUS Tic-Tac-Toe" "Branching State Composition"

    subscribe_interactions "click"
    subscribe_sensor_stream
    ttt_dag_create_session
    ledger_create_spine

    render_board "$BOARD" "Your turn (X) — click or press 1-9" "$HOVER_CELL"
    ok "board rendered — three interaction layers active"

    game_loop_round
}

game_loop_round() {
    while ! $GAME_OVER; do
        if [ "$TURN" = "X" ]; then
            local selected_cell=-1

            local sensor_batch
            sensor_batch=$(poll_sensor_stream)
            process_sensor_batch "$sensor_batch"

            ACCUMULATED_HOVER_MOVES=$((ACCUMULATED_HOVER_MOVES + SENSOR_HOVER_MOVES))
            if $SENSOR_HOVER_CHANGED; then
                render_board "$BOARD" "Your turn (X) — click or press 1-9" "$HOVER_CELL"
            fi

            if [[ -n "$SENSOR_KEY" ]]; then
                case "$SENSOR_KEY" in
                    Num1|1) selected_cell=0 ;;
                    Num2|2) selected_cell=1 ;;
                    Num3|3) selected_cell=2 ;;
                    Num4|4) selected_cell=3 ;;
                    Num5|5) selected_cell=4 ;;
                    Num6|6) selected_cell=5 ;;
                    Num7|7) selected_cell=6 ;;
                    Num8|8) selected_cell=7 ;;
                    Num9|9) selected_cell=8 ;;
                    U|u)
                        log "undo requested"
                        if ttt_dag_undo; then
                            render_board "$BOARD" "Undo! Your turn (X) — move $MOVE_NUM" -1
                        else
                            render_board "$BOARD" "Nothing to undo" "$HOVER_CELL"
                        fi
                        sleep "$POLL_INTERVAL"
                        continue
                        ;;
                    B|b)
                        log "show branches at current vertex"
                        show_branches
                        sleep "$POLL_INTERVAL"
                        continue
                        ;;
                    T|t)
                        log "tree view requested"
                        show_tree_view
                        sleep 2.0
                        render_board "$BOARD" "Your turn (X) — click or press 1-9" "$HOVER_CELL"
                        continue
                        ;;
                    R|r)
                        log "restart requested via keyboard"
                        BOARD="........."
                        TURN="X"
                        MOVE_NUM=0
                        HOVER_CELL=-1
                        SPINE_ID=""
                        ttt_dag_create_session
                        ledger_create_spine
                        render_board "$BOARD" "Restarted! Your turn (X)" -1
                        sleep "$POLL_INTERVAL"
                        continue
                        ;;
                    Q|q|Escape)
                        log "quit requested via keyboard"
                        GAME_OVER=true
                        continue
                        ;;
                esac
            fi

            if [[ "$selected_cell" -eq -1 ]] && [[ "$SENSOR_CLICK_CELL" -ge 0 ]]; then
                selected_cell=$SENSOR_CLICK_CELL
                SENSOR_INPUT_TYPE="click"
            fi

            if [[ "$selected_cell" -eq -1 ]]; then
                local sem_resp
                sem_resp=$(poll_interaction)
                if echo "$sem_resp" | grep -q '"click"' && echo "$sem_resp" | grep -q "cell-"; then
                    local sem_cell
                    sem_cell=$(extract_cell_click "$sem_resp")
                    if [[ -n "$sem_cell" ]]; then
                        selected_cell=$sem_cell
                        SENSOR_INPUT_TYPE="semantic_click"
                    fi
                fi
            fi

            if [[ "$selected_cell" -eq -1 ]]; then
                check_proprioception
                sleep "$POLL_INTERVAL"
                continue
            fi

            if [ "${BOARD:$selected_cell:1}" != "." ]; then
                render_board "$BOARD" "Cell $((selected_cell+1)) occupied! Pick another" "$HOVER_CELL"
                sleep "$POLL_INTERVAL"
                continue
            fi

            local move_input_type="${SENSOR_INPUT_TYPE:-unknown}"
            local move_hover_count="$ACCUMULATED_HOVER_MOVES"

            BOARD=$(apply_move "$BOARD" "$selected_cell" "X")
            MOVE_NUM=$((MOVE_NUM + 1))
            log "player X → cell $selected_cell via $move_input_type (move $MOVE_NUM, $move_hover_count hovers)"

            sign_payload "X:$selected_cell:$BOARD:$MOVE_NUM"
            ttt_dag_append_state "X" "$selected_cell" "$BOARD" "$move_input_type" "$move_hover_count"
            ttt_braid_record "move" "X" "$BOARD" "$CURRENT_VERTEX" "$move_input_type" "$move_hover_count"
            ACCUMULATED_HOVER_MOVES=0

            local winner
            winner=$(check_winner "$BOARD")
            if [ "$winner" = "X" ]; then
                render_board "$BOARD" "X wins! (R to restart, Q to quit)" -1
                ok "GAME OVER: X wins in $MOVE_NUM moves"
                GAME_OVER=true
                ttt_braid_record "win" "X" "$BOARD" "$CURRENT_VERTEX" "$move_input_type" "0"
                ttt_ledger_commit_game_line "X_wins"
            elif [ "$winner" = "draw" ]; then
                render_board "$BOARD" "Draw! (R to restart, Q to quit)" -1
                ok "GAME OVER: Draw in $MOVE_NUM moves"
                GAME_OVER=true
                ttt_braid_record "draw" "-" "$BOARD" "$CURRENT_VERTEX" "$move_input_type" "0"
                ttt_ledger_commit_game_line "draw"
            else
                render_board "$BOARD" "AI thinking..." -1
                TURN="O"
            fi
        else
            sleep 0.5
            local ai_cell
            ai_cell=$(compute_ai_move)

            if [[ -z "$ai_cell" ]]; then
                err "AI could not find a move"
                GAME_OVER=true
                continue
            fi

            BOARD=$(apply_move "$BOARD" "$ai_cell" "O")
            MOVE_NUM=$((MOVE_NUM + 1))
            log "AI O → cell $ai_cell (move $MOVE_NUM)"

            sign_payload "O:$ai_cell:$BOARD:$MOVE_NUM"
            ttt_dag_append_state "O" "$ai_cell" "$BOARD" "ai" "0"
            ttt_braid_record "move" "O" "$BOARD" "$CURRENT_VERTEX" "ai" "0"

            local winner
            winner=$(check_winner "$BOARD")
            if [ "$winner" = "O" ]; then
                render_board "$BOARD" "O wins! (R to restart, Q to quit)" -1
                ok "GAME OVER: O wins in $MOVE_NUM moves"
                GAME_OVER=true
                ttt_braid_record "win" "O" "$BOARD" "$CURRENT_VERTEX" "ai" "0"
                ttt_ledger_commit_game_line "O_wins"
            elif [ "$winner" = "draw" ]; then
                render_board "$BOARD" "Draw! (R to restart, Q to quit)" -1
                ok "GAME OVER: Draw in $MOVE_NUM moves"
                GAME_OVER=true
                ttt_braid_record "draw" "-" "$BOARD" "$CURRENT_VERTEX" "ai" "0"
                ttt_ledger_commit_game_line "draw"
            else
                render_board "$BOARD" "Your turn (X) — click or press 1-9" "$HOVER_CELL"
                TURN="X"
            fi
        fi
    done

    log "Game over — waiting for R (restart) or Q (quit)..."
    local quit_requested=false
    while true; do
        local sensor_batch
        sensor_batch=$(poll_sensor_stream)
        process_sensor_batch "$sensor_batch"
        if [[ -n "$SENSOR_KEY" ]]; then
            case "$SENSOR_KEY" in
                R|r)
                    log "restarting game..."
                    BOARD="........."
                    TURN="X"
                    MOVE_NUM=0
                    GAME_OVER=false
                    HOVER_CELL=-1
                    DAG_SESSION=""
                    SPINE_ID=""
                    ttt_dag_create_session
                    ledger_create_spine
                    render_board "$BOARD" "New game! Your turn (X)" -1
                    break
                    ;;
                Q|q|Escape)
                    log "quitting..."
                    quit_requested=true
                    break
                    ;;
            esac
        fi
        sleep "$POLL_INTERVAL"
    done

    if ! $GAME_OVER && ! $quit_requested; then
        game_loop_round
        return
    fi

    composition_summary

    ok "Game complete. Run again or: ./tools/ttt_nucleus.sh stop"
    composition_teardown "tictactoe" "ttt-tree"
}

# ── Main ──────────────────────────────────────────────────────────────

discover_capabilities || { err "Required primals not found. Run: ./tools/ttt_nucleus.sh start"; exit 1; }
game_loop
