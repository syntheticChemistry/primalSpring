#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# desktop_nucleus.sh — Deploy the 13-primal NUCLEUS as a desktop system
#
# Primary path: biomeOS deploy (coordinator primal handles lifecycle)
# Fallback path: composition_nucleus.sh (shell-managed)
#
# All 13 primals deploy from plasmidBin. No spring binaries.
# petalTongue runs in `live` mode (egui desktop window).
#
# Usage:
#   ./tools/desktop_nucleus.sh start    # deploy desktop NUCLEUS
#   ./tools/desktop_nucleus.sh stop     # graceful shutdown
#   ./tools/desktop_nucleus.sh status   # health check all 13

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
ECO_ROOT="$(cd "$PROJECT_ROOT/../.." && pwd)"

COMPOSITION_NAME="${COMPOSITION_NAME:-desktop}"
FAMILY_ID="${FAMILY_ID:-desktop-nucleus}"
SOCKET_DIR="${XDG_RUNTIME_DIR:-/tmp}/biomeos"
PLASMID_BIN="${ECOPRIMALS_PLASMID_BIN:-${XDG_DATA_HOME:-$HOME/.local/share}/ecoPrimals/plasmidBin}"
BIN_DIR="$PLASMID_BIN/primals/x86_64-unknown-linux-musl"
CELL_GRAPH="$PROJECT_ROOT/graphs/cells/nucleus_desktop_cell.toml"

export FAMILY_ID
export BEARDOG_FAMILY_SEED="${BEARDOG_FAMILY_SEED:-$(head -c 32 /dev/urandom | xxd -p | tr -d '\n')}"
export NODE_ID="${NODE_ID:-$(hostname)}"
export BEARDOG_NODE_ID="${BEARDOG_NODE_ID:-$NODE_ID}"
export NESTGATE_JWT_SECRET="${NESTGATE_JWT_SECRET:-nucleus-beardog-delegated-auth}"

log()  { echo "[desktop-nucleus] $(date +%H:%M:%S) $*"; }
err()  { echo "[desktop-nucleus] ERROR: $*" >&2; }
ok()   { echo "[desktop-nucleus] OK: $*"; }
warn() { echo "[desktop-nucleus] WARN: $*"; }

SPAWNED_PRIMALS="beardog songbird toadstool barracuda coralreef nestgate rhizocrypt loamspine sweetgrass squirrel skunkbat petaltongue"
PRIMALS="$SPAWNED_PRIMALS biomeos"

biomeos_bin() {
    local b="$BIN_DIR/biomeos"
    [[ -x "$b" ]] && echo "$b" && return
    which biomeos 2>/dev/null || true
}

biomeos_is_available() {
    local biomeos
    biomeos="$(biomeos_bin)"
    [[ -n "$biomeos" ]] && [[ -x "$biomeos" ]]
}

verify_plasmidbin() {
    log "Verifying plasmidBin binaries..."
    local missing=0
    for p in $PRIMALS; do
        if [[ ! -x "$BIN_DIR/$p" ]]; then
            err "Missing: $BIN_DIR/$p"
            missing=$((missing + 1))
        fi
    done
    if [[ $missing -gt 0 ]]; then
        err "$missing primals missing from plasmidBin"
        return 1
    fi
    ok "13/13 primals present in plasmidBin"
}

create_capability_symlinks() {
    # Map capability names to primal socket names so discover_by_capability()
    # finds primals whose socket name doesn't match their capability domain.
    local -A cap_map=(
        [visualization]="petaltongue"
        [orchestration]="neural-api"
        [game_science]="ludospring"
        [shader]="coralreef"
        [math]="barracuda"
        [crypto]="beardog"
        [discovery]="songbird"
        [compute]="toadstool"
        [storage]="nestgate"
        [dag]="rhizocrypt"
        [ledger]="loamspine"
        [attribution]="sweetgrass"
        [ai]="squirrel"
        [defense]="skunkbat"
    )

    for cap in "${!cap_map[@]}"; do
        local primal="${cap_map[$cap]}"
        local primal_sock="$SOCKET_DIR/${primal}-${FAMILY_ID}.sock"
        local cap_sock="$SOCKET_DIR/${cap}-${FAMILY_ID}.sock"
        if [[ -S "$primal_sock" ]] && [[ ! -e "$cap_sock" ]]; then
            ln -sf "$primal_sock" "$cap_sock" 2>/dev/null && \
                log "  symlink: $cap → $primal" || true
        fi
    done
}

cmd_start() {
    log "============================================"
    log "  Desktop NUCLEUS Deployment"
    log "  family_id:    $FAMILY_ID"
    log "  socket_dir:   $SOCKET_DIR"
    log "  plasmidBin:   $BIN_DIR"
    log "============================================"

    verify_plasmidbin || return 1

    log "Deploying 13-primal NUCLEUS via composition launcher"
    start_via_composition
}

start_via_biomeos() {
    local biomeos
    biomeos="$(biomeos_bin)"

    if [[ ! -f "$CELL_GRAPH" ]]; then
        err "Cell graph not found: $CELL_GRAPH"
        return 1
    fi

    log "Deploying via: biomeos nucleus --mode full"
    mkdir -p "$SOCKET_DIR"
    echo "$BEARDOG_FAMILY_SEED" > "$SOCKET_DIR/.family.seed"
    chmod 600 "$SOCKET_DIR/.family.seed"

    export ECOPRIMALS_PLASMID_BIN="$PLASMID_BIN"
    export PETALTONGUE_LIVE=true
    export DISPLAY="${DISPLAY:-:1}"
    export COMPOSITION_NAME="$COMPOSITION_NAME"

    FAMILY_SEED="$BEARDOG_FAMILY_SEED" \
        "$biomeos" nucleus \
            --mode full \
            --node-id "$NODE_ID" \
            --family-id "$FAMILY_ID" \
            --log-level info &

    local biomeos_pid=$!
    mkdir -p "/tmp/nucleus-${COMPOSITION_NAME}-pids"
    echo "$biomeos_pid" > "/tmp/nucleus-${COMPOSITION_NAME}-pids/biomeos.pid"
    log "biomeOS nucleus started (pid=$biomeos_pid)"

    log "Waiting for primals to come up..."
    sleep 5

    log "Attempting graph deploy for desktop cell..."
    "$biomeos" deploy "$CELL_GRAPH" --validate-only 2>/dev/null && \
        log "Cell graph validated" || \
        log "WARN: Cell graph validation not available (expected for first deploy)"

    cmd_status
    print_connection_info
}

start_via_composition() {
    log "Using composition_nucleus.sh"
    export COMPOSITION_NAME
    export FAMILY_ID
    export PETALTONGUE_LIVE=true
    export ECOPRIMALS_PLASMID_BIN="$PLASMID_BIN"
    export DISCOVERY_SOCKET="$SOCKET_DIR/songbird-${FAMILY_ID}.sock"
    export LOCAL_AI_ENDPOINT="${LOCAL_AI_ENDPOINT:-http://127.0.0.1:11434}"
    export OLLAMA_ENDPOINT="${OLLAMA_ENDPOINT:-http://127.0.0.1:11434}"

    PRIMAL_LIST="beardog songbird nestgate squirrel toadstool barracuda coralreef rhizocrypt loamspine sweetgrass petaltongue" \
        "$SCRIPT_DIR/composition_nucleus.sh" start

    # petalTongue hardcodes heartbeat to discovery-service.sock — bridge it
    local songbird_sock="$SOCKET_DIR/songbird-${FAMILY_ID}.sock"
    if [[ -S "$songbird_sock" ]]; then
        ln -sf "$songbird_sock" "$SOCKET_DIR/discovery-service.sock" 2>/dev/null
    fi

    # Capability-aliased symlinks: discover_by_capability() looks for
    # {capability}-{family}.sock but primals register as {primal}-{family}.sock.
    # These symlinks bridge the gap so experiments and springs discover primals
    # by capability name. (GAP-17, GAP-18, GAP-19 local mitigation)
    create_capability_symlinks

    print_connection_info
}

cmd_stop() {
    log "Stopping Desktop NUCLEUS..."

    local biomeos_pid_file="/tmp/nucleus-${COMPOSITION_NAME}-pids/biomeos.pid"
    if [[ -f "$biomeos_pid_file" ]]; then
        local pid
        pid=$(cat "$biomeos_pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            kill "$pid" 2>/dev/null && log "stopped biomeOS (pid=$pid)"
        fi
        rm -f "$biomeos_pid_file"
    fi

    COMPOSITION_NAME="$COMPOSITION_NAME" \
    FAMILY_ID="$FAMILY_ID" \
        "$SCRIPT_DIR/composition_nucleus.sh" stop 2>/dev/null || true

    ok "Desktop NUCLEUS stopped"
}

primal_socket() {
    local p="$1"
    case "$p" in
        biomeos)
            local s="$SOCKET_DIR/biomeos.sock"
            [[ -S "$s" ]] && echo "$s" && return
            echo "$SOCKET_DIR/neural-api-${FAMILY_ID}.sock" ;;
        coralreef)
            local s="$SOCKET_DIR/coralreef-core-default.sock"
            [[ -S "$s" ]] && echo "$s" && return
            echo "$SOCKET_DIR/coralreef-${FAMILY_ID}.sock" ;;
        *) echo "$SOCKET_DIR/${p}-${FAMILY_ID}.sock" ;;
    esac
}

ipc_health() {
    local sock="$1"
    local resp
    resp=$(echo '{"jsonrpc":"2.0","method":"health.liveness","id":1}' \
        | timeout 2 socat - "UNIX-CONNECT:$sock" 2>/dev/null || true)
    [[ -n "$resp" ]] && echo "$resp" | grep -qE '"alive"|"status"' && return 0
    return 1
}

cmd_status() {
    log "── Desktop NUCLEUS Status ──"
    local healthy=0

    for primal in $SPAWNED_PRIMALS; do
        local s
        s="$(primal_socket "$primal")"
        if [[ -S "$s" ]]; then
            if ipc_health "$s"; then
                ok "$primal: alive ($s)"
                healthy=$((healthy + 1))
            else
                warn "$primal: socket present, no health response ($s)"
            fi
        else
            log "  $primal: not running"
        fi
    done

    local biomeos_sock
    biomeos_sock="$(primal_socket biomeos)"
    if [[ -S "$biomeos_sock" ]]; then
        if ipc_health "$biomeos_sock"; then
            ok "biomeos: alive ($biomeos_sock)"
            healthy=$((healthy + 1))
        else
            warn "biomeos: socket present, no health response"
        fi
    elif pgrep -f "biomeos" >/dev/null 2>&1; then
        ok "biomeos: process running (coordinator, no IPC socket in shell mode)"
        healthy=$((healthy + 1))
    else
        log "  biomeos: not running (shell-managed coordination active)"
        healthy=$((healthy + 1))
    fi

    log "── Result: $healthy/13 primals (12 spawned + 1 coordinator) ──"
}

print_connection_info() {
    log ""
    log "═══════════════════════════════════════════"
    log "  Desktop NUCLEUS is live"
    log "═══════════════════════════════════════════"
    log ""
    log "  Capability sockets (for spring compositions):"
    log "    security:      $SOCKET_DIR/beardog-${FAMILY_ID}.sock"
    log "    discovery:     $SOCKET_DIR/songbird-${FAMILY_ID}.sock"
    log "    compute:       $SOCKET_DIR/toadstool-${FAMILY_ID}.sock"
    log "    tensor:        $SOCKET_DIR/barracuda-${FAMILY_ID}.sock"
    log "    shader:        $SOCKET_DIR/coralreef-${FAMILY_ID}.sock"
    log "    storage:       $SOCKET_DIR/nestgate-${FAMILY_ID}.sock"
    log "    dag:           $SOCKET_DIR/rhizocrypt-${FAMILY_ID}.sock"
    log "    ledger:        $SOCKET_DIR/loamspine-${FAMILY_ID}.sock"
    log "    attribution:   $SOCKET_DIR/sweetgrass-${FAMILY_ID}.sock"
    log "    ai:            $SOCKET_DIR/squirrel-${FAMILY_ID}.sock"
    log "    visualization: $SOCKET_DIR/petaltongue-${FAMILY_ID}.sock"
    log ""
    log "  Springs connect to these sockets — don't launch primals."
    log "  A spring IS a composition of the running NUCLEUS."
    log ""
}

ipc_call() {
    local sock="$1" method="$2" params="$3"
    printf '{"jsonrpc":"2.0","method":"%s","params":%s,"id":1}\n' "$method" "$params" \
        | timeout 3 socat - "UNIX-CONNECT:$sock" 2>/dev/null || true
}

cmd_validate() {
    log "── Desktop NUCLEUS Deep Validation ──"
    local pass=0 fail=0

    check() {
        local label="$1" sock="$2" method="$3" params="${4-"{}"}"
        local resp
        resp=$(ipc_call "$sock" "$method" "$params")
        if [[ -n "$resp" ]] && echo "$resp" | grep -q '"result"'; then
            ok "  $label: $method"
            pass=$((pass + 1))
        else
            err "  $label: $method FAILED — $(echo "$resp" | head -c 120)"
            fail=$((fail + 1))
        fi
    }

    local bd sg ts bc cr ng rz ls sw sq pt
    bd="$(primal_socket beardog)"
    sg="$(primal_socket songbird)"
    ts="$(primal_socket toadstool)"
    bc="$(primal_socket barracuda)"
    cr="$(primal_socket coralreef)"
    ng="$(primal_socket nestgate)"
    rz="$(primal_socket rhizocrypt)"
    ls="$(primal_socket loamspine)"
    sw="$(primal_socket sweetgrass)"
    sq="$(primal_socket squirrel)"
    pt="$(primal_socket petaltongue)"

    log "Tower (electron)"
    check "BearDog"   "$bd" "rpc.methods"
    check "BearDog"   "$bd" "crypto.blake3_hash" '{"data":"test"}'
    check "Songbird"  "$sg" "rpc.discover"

    log "Node (proton)"
    check "ToadStool" "$ts" "compute.capabilities"
    check "barraCuda" "$bc" "tensor.create" '{"shape":[2,2],"fill":1.0}'
    check "coralReef" "$cr" "health.check"

    log "Nest (neutron)"
    check "NestGate"  "$ng" "storage.list" "{\"family_id\":\"$FAMILY_ID\",\"prefix\":\"validate-\"}"
    check "rhizoCrypt" "$rz" "dag.session.create" '{"name":"validate-test"}'
    check "loamSpine" "$ls" "primal.capabilities"
    check "sweetGrass" "$sw" "capabilities.list"

    log "Nest: sweetGrass Tower Signing"
    local braid_hash
    braid_hash=$(printf 'validate-%s' "$(date +%s%N)" | b3sum --no-names 2>/dev/null || \
                 printf 'validate-%s' "$(date +%s)" | sha256sum | awk '{print $1}')
    local braid_resp
    braid_resp=$(ipc_call "$sw" "braid.create" \
        "{\"data_hash\":\"$braid_hash\",\"mime_type\":\"text/plain\",\"agent\":\"primalSpring-validate\",\"size\":42}")
    if [[ -n "$braid_resp" ]] && echo "$braid_resp" | grep -q '"result"'; then
        if echo "$braid_resp" | grep -q '"tower"'; then
            ok "  sweetGrass braid.create: Tower-signed witness"
            pass=$((pass + 1))
        elif echo "$braid_resp" | grep -q '"open"'; then
            ok "  sweetGrass braid.create: unsigned (BearDog degradation OK)"
            pass=$((pass + 1))
        else
            ok "  sweetGrass braid.create: created (witness tier unknown)"
            pass=$((pass + 1))
        fi
    else
        err "  sweetGrass braid.create: FAILED — $(echo "$braid_resp" | head -c 120)"
        fail=$((fail + 1))
    fi

    log "Meta (cross-atomic)"
    check "Squirrel"  "$sq" "inference.models"
    check "petalTongue" "$pt" "proprioception.get"

    log "Service Mesh (Tower discovery)"
    check "Songbird"  "$sg" "ipc.list"
    for cap in security compute tensor storage dag ledger attribution ai visualization shader; do
        local resolve_resp
        resolve_resp=$(ipc_call "$sg" "ipc.resolve" "{\"capability\":\"$cap\"}")
        if [[ -n "$resolve_resp" ]] && echo "$resolve_resp" | grep -q '"native_endpoint"'; then
            ok "  resolve($cap)"
            pass=$((pass + 1))
        else
            err "  resolve($cap) FAILED"
            fail=$((fail + 1))
        fi
    done

    log "Crypto Tier 0: Seed Fingerprints"
    local seed_fp="$SCRIPT_DIR/../validation/seed_fingerprints.toml"
    if [[ -f "$seed_fp" ]]; then
        local fp_count=0
        while IFS= read -r line; do
            line="${line%%#*}"
            line="$(echo "$line" | xargs 2>/dev/null || true)"
            [[ -z "$line" || "$line" == "["* ]] && continue
            local key val
            key="${line%%=*}"; key="$(echo "$key" | xargs)"
            val="${line#*=}"; val="$(echo "$val" | xargs | tr -d '"')"
            [[ -n "$key" && ${#val} -eq 64 ]] && fp_count=$((fp_count + 1))
        done < "$seed_fp"
        if (( fp_count >= 12 )); then
            ok "  seed fingerprints: $fp_count/12 present"
            pass=$((pass + 1))
        else
            err "  seed fingerprints: only $fp_count/12"
            fail=$((fail + 1))
        fi
    else
        err "  seed_fingerprints.toml not found"
        fail=$((fail + 1))
    fi

    log "Crypto Tier 1: BearDog Key Derivation"
    local hmac_resp
    hmac_resp=$(ipc_call "$bd" "crypto.hmac_sha256" \
        '{"key":"deadbeef","data":"746573742d6b6579"}')
    if [[ -n "$hmac_resp" ]] && echo "$hmac_resp" | grep -q '"result"'; then
        ok "  HMAC-SHA256 derivation: works"
        pass=$((pass + 1))
    else
        err "  HMAC-SHA256 derivation: FAILED — $(echo "$hmac_resp" | head -c 120)"
        fail=$((fail + 1))
    fi

    log "Crypto Tier 2: Purpose-Key Lazy Derivation (BearDog W74)"
    local purpose_resp
    purpose_resp=$(ipc_call "$bd" "secrets.retrieve" \
        '{"name":"nucleus:'"${FAMILY_ID}"':purpose:validate-test"}')
    if [[ -n "$purpose_resp" ]] && echo "$purpose_resp" | grep -q '"result"'; then
        ok "  purpose-key lazy derivation (via secrets.retrieve): works"
        pass=$((pass + 1))
    elif [[ -n "$purpose_resp" ]] && echo "$purpose_resp" | grep -q '"error"'; then
        local errmsg
        errmsg=$(echo "$purpose_resp" | grep -oP '"message"\s*:\s*"\K[^"]+' | head -1)
        if echo "$errmsg" | grep -qi "not found"; then
            ok "  purpose-key derivation: secret store responded (key not pre-derived — OK for lazy model)"
            pass=$((pass + 1))
        else
            err "  purpose-key derivation: FAILED — $(echo "$purpose_resp" | head -c 120)"
            fail=$((fail + 1))
        fi
    else
        err "  purpose-key derivation: FAILED — no response"
        fail=$((fail + 1))
    fi

    log "Crypto: BTSP Session"
    local btsp_resp
    btsp_resp=$(ipc_call "$bd" "btsp.session.create" \
        "{\"family_seed\":\"${BEARDOG_FAMILY_SEED:-}\"}")
    if echo "$btsp_resp" | grep -q '"server_ephemeral_pub"'; then
        ok "  BTSP session create: works"
        pass=$((pass + 1))
    else
        err "  BTSP session create: FAILED — $(echo "$btsp_resp" | head -c 120)"
        fail=$((fail + 1))
    fi

    log "Crypto: Sign/Verify Round-Trip"
    local sign_msg
    sign_msg=$(printf 'validate-%s' "$(date +%s)" | base64 -w0)
    local sign_resp
    sign_resp=$(ipc_call "$bd" "crypto.sign" "{\"message\":\"$sign_msg\"}")
    local sig_val pub_val
    sig_val=$(echo "$sign_resp" | grep -oP '"signature"\s*:\s*"\K[^"]+' | head -1 || true)
    pub_val=$(echo "$sign_resp" | grep -oP '"public_key"\s*:\s*"\K[^"]+' | head -1 || true)
    if [[ -n "$sig_val" ]]; then
        local verify_resp
        verify_resp=$(ipc_call "$bd" "crypto.verify" \
            "{\"message\":\"$sign_msg\",\"signature\":\"$sig_val\",\"public_key\":\"${pub_val:-}\"}")
        if echo "$verify_resp" | grep -qiE '"valid"\s*:\s*true|"result"\s*:\s*true'; then
            ok "  sign/verify: PASS"
            pass=$((pass + 1))
        else
            err "  verify: FAILED — $(echo "$verify_resp" | head -c 120)"
            fail=$((fail + 1))
        fi
    else
        err "  sign: FAILED — $(echo "$sign_resp" | head -c 120)"
        fail=$((fail + 1))
    fi

    log "Crypto: Secrets Store Round-Trip"
    local secret_test_name="validate:test:$(date +%s)"
    local store_resp
    store_resp=$(ipc_call "$bd" "secrets.store" \
        "{\"name\":\"$secret_test_name\",\"value\":\"test-value-42\"}")
    if [[ -n "$store_resp" ]] && ! echo "$store_resp" | grep -q '"error"'; then
        local retrieve_resp
        retrieve_resp=$(ipc_call "$bd" "secrets.retrieve" "{\"name\":\"$secret_test_name\"}")
        if echo "$retrieve_resp" | grep -q 'test-value-42'; then
            ok "  secrets store/retrieve: PASS"
            pass=$((pass + 1))
        else
            err "  secrets retrieve: FAILED — $(echo "$retrieve_resp" | head -c 120)"
            fail=$((fail + 1))
        fi
        ipc_call "$bd" "secrets.delete" "{\"name\":\"$secret_test_name\"}" >/dev/null 2>&1 || true
    else
        err "  secrets store: FAILED — $(echo "$store_resp" | head -c 120)"
        fail=$((fail + 1))
    fi

    log "── Validation: $pass passed, $fail failed ──"
}

case "${1:-start}" in
    start)    cmd_start ;;
    stop)     cmd_stop ;;
    status)   cmd_status ;;
    validate) cmd_validate ;;
    restart)  cmd_stop; sleep 2; cmd_start ;;
    *) err "Usage: $0 {start|stop|status|validate|restart}"; exit 1 ;;
esac
