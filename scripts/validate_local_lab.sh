#!/usr/bin/env bash
# validate_local_lab.sh — One-command multi-node primal validation via benchScale
#
# Orchestrates: create lab -> deploy plasmidBin -> wait for health -> run
# cross-gate experiments (exp073 + exp074) -> collect results -> teardown.
#
# Requires Docker or LXD. The Docker path needs no root (Tier 1).
# LXD/QEMU path supports libvirt VMs (Tier 2).
#
# Usage:
#   ./scripts/validate_local_lab.sh [options]
#   ./scripts/validate_local_lab.sh --topology ecoprimals-tower-2node
#   ./scripts/validate_local_lab.sh --topology ecoprimals-nucleus-3node --keep
#   ./scripts/validate_local_lab.sh --topology ecoprimals-wan-federation --hypervisor docker

set -euo pipefail

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PRIMALSPRING_ROOT="$(dirname "$SCRIPT_DIR")"

TOPOLOGY="ecoprimals-tower-2node"
HYPERVISOR=""
LAB_NAME=""
KEEP_LAB=false
SKIP_BUILD=false
EXTRA_EXPERIMENTS=""
TIMEOUT_SECS=120

BENCHSCALE_DIR=""
PLASMIDBIN_DIR=""

for candidate in \
    "$PRIMALSPRING_ROOT/../../infra/benchScale" \
    "$PRIMALSPRING_ROOT/../../../infra/benchScale"; do
    if [ -d "$candidate/scripts" ]; then
        BENCHSCALE_DIR="$(cd "$candidate" && pwd)"
        break
    fi
done

PLASMIDBIN_DIR="${ECOPRIMALS_PLASMID_BIN:-${XDG_DATA_HOME:-$HOME/.local/share}/ecoPrimals/plasmidBin}"
if [[ ! -d "$PLASMIDBIN_DIR" ]]; then
    PLASMIDBIN_DIR=""
fi

usage() {
    cat << EOF
Usage: $0 [options]

One-command local multi-node ecoPrimals validation.

Creates a benchScale lab, deploys primal binaries from plasmidBin, runs
cross-gate validation experiments, and tears down the lab.

Options:
    --topology <name>       benchScale topology (default: $TOPOLOGY)
                            Available: ecoprimals-tower-2node, ecoprimals-nucleus-3node,
                                       ecoprimals-wan-federation
    --hypervisor <type>     docker (default if available), lxd, qemu
    --name <lab-name>       Lab name (default: auto-generated from topology)
    --benchscale <path>     Path to benchScale root (default: auto-detect)
    --plasmidbin <path>     Path to plasmidBin root (default: auto-detect)
    --keep                  Keep lab running after validation (don't teardown)
    --skip-build            Skip cargo build step for experiments
    --experiments <list>    Comma-separated extra experiment binaries to run
    --timeout <secs>        Max seconds to wait for primal health (default: $TIMEOUT_SECS)
    --help                  Show this help

Examples:
    $0                                          # Quick 2-node Docker validation
    $0 --topology ecoprimals-nucleus-3node      # Full NUCLEUS + NAT + mobile
    $0 --topology ecoprimals-wan-federation     # WAN degradation testing
    $0 --keep --topology ecoprimals-tower-2node # Keep lab for manual exploration

EOF
    exit 1
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --topology)     TOPOLOGY="$2"; shift 2 ;;
        --hypervisor)   HYPERVISOR="$2"; shift 2 ;;
        --name)         LAB_NAME="$2"; shift 2 ;;
        --benchscale)   BENCHSCALE_DIR="$2"; shift 2 ;;
        --plasmidbin)   PLASMIDBIN_DIR="$2"; shift 2 ;;
        --keep)         KEEP_LAB=true; shift ;;
        --skip-build)   SKIP_BUILD=true; shift ;;
        --experiments)  EXTRA_EXPERIMENTS="$2"; shift 2 ;;
        --timeout)      TIMEOUT_SECS="$2"; shift 2 ;;
        --help)         usage ;;
        *) echo -e "${RED}Unknown option: $1${NC}"; usage ;;
    esac
done

# ── Validation ───────────────────────────────────────────────────────────────

if [ -z "$BENCHSCALE_DIR" ]; then
    echo -e "${RED}Cannot find benchScale. Set --benchscale <path> or ensure infra/benchScale/ exists.${NC}"
    exit 1
fi

if [ -z "$PLASMIDBIN_DIR" ]; then
    echo -e "${RED}Cannot find plasmidBin. Run ./tools/fetch_primals.sh or set ECOPRIMALS_PLASMID_BIN.${NC}"
    exit 1
fi

# Auto-detect hypervisor
if [ -z "$HYPERVISOR" ]; then
    if command -v docker &>/dev/null && docker info &>/dev/null 2>&1; then
        HYPERVISOR="docker"
    elif command -v lxc &>/dev/null; then
        HYPERVISOR="lxd"
    elif command -v qemu-system-x86_64 &>/dev/null; then
        HYPERVISOR="qemu"
    else
        echo -e "${RED}No supported hypervisor found. Install Docker, LXD, or QEMU.${NC}"
        exit 1
    fi
fi

LAB_NAME="${LAB_NAME:-validate-${TOPOLOGY}-$(date +%s)}"

log()      { echo -e "${GREEN}[validate]${NC} $1"; }
log_info() { echo -e "${BLUE}[validate]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[validate]${NC} $1"; }
log_err()  { echo -e "${RED}[validate]${NC} $1"; }
banner()   { echo -e "\n${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"; echo -e " $1"; echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"; }

RESULTS_DIR="$PRIMALSPRING_ROOT/validation_results/${LAB_NAME}"
mkdir -p "$RESULTS_DIR"

cleanup() {
    if [ "$KEEP_LAB" = true ]; then
        log_info "Lab $LAB_NAME kept alive (--keep). Tear down manually:"
        echo "  $BENCHSCALE_DIR/scripts/destroy-lab.sh --lab $LAB_NAME --force"
        return
    fi
    log "Tearing down lab: $LAB_NAME"
    "$BENCHSCALE_DIR/scripts/destroy-lab.sh" --lab "$LAB_NAME" --force 2>/dev/null || true
}
trap cleanup EXIT

# ── Banner ───────────────────────────────────────────────────────────────────

banner "primalSpring Local Validation Substrate"
log_info "Topology:     $TOPOLOGY"
log_info "Hypervisor:   $HYPERVISOR"
log_info "Lab:          $LAB_NAME"
log_info "benchScale:   $BENCHSCALE_DIR"
log_info "plasmidBin:   $PLASMIDBIN_DIR"
log_info "Results:      $RESULTS_DIR"
log_info "Timeout:      ${TIMEOUT_SECS}s"
echo ""

# ── Step 1: Create Lab ──────────────────────────────────────────────────────

banner "Step 1/5: Create benchScale Lab"

"$BENCHSCALE_DIR/scripts/create-lab.sh" \
    --topology "$TOPOLOGY" \
    --name "$LAB_NAME" \
    --hypervisor "$HYPERVISOR" 2>&1 | tee "$RESULTS_DIR/create-lab.log"

log "Lab created"

# ── Step 2: Deploy Binaries ─────────────────────────────────────────────────

banner "Step 2/5: Deploy ecoPrimals Binaries"

"$BENCHSCALE_DIR/scripts/deploy-ecoprimals.sh" \
    --lab "$LAB_NAME" \
    --plasmidbin "$PLASMIDBIN_DIR" \
    --graphs "$PRIMALSPRING_ROOT/graphs" \
    --seed "$LAB_NAME" 2>&1 | tee "$RESULTS_DIR/deploy.log"

log "Binaries deployed"

# ── Step 3: Wait for Health ──────────────────────────────────────────────────

banner "Step 3/5: Wait for Primal Health"

TOPOLOGY_FILE="$BENCHSCALE_DIR/topologies/${TOPOLOGY}.yaml"

get_node_names() {
    grep -E '^\s+-\s+name:' "$TOPOLOGY_FILE" | sed 's/.*name:\s*//' | tr -d '"' | tr -d "'"
}

get_node_env() {
    local node="$1" key="$2"
    awk -v node="$node" -v key="$key" '
        /^\s+-\s+name:/ { current = $NF; gsub(/["'"'"']/, "", current) }
        current == node && $1 == key":" {
            val = substr($0, index($0, ":") + 1)
            gsub(/^[ \t]+/, "", val)
            gsub(/["'"'"']/, "", val)
            print val
        }
    ' "$TOPOLOGY_FILE"
}

container_name() { echo "${LAB_NAME}-${1}"; }

get_node_ip() {
    local node="$1"
    local cname
    cname="$(container_name "$node")"
    case "$HYPERVISOR" in
        docker) docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$cname" 2>/dev/null ;;
        lxd)    lxc list "$cname" --format csv -c 4 2>/dev/null | cut -d' ' -f1 ;;
        *)      echo "127.0.0.1" ;;
    esac
}

all_healthy=false
elapsed=0
interval=5
while [ "$elapsed" -lt "$TIMEOUT_SECS" ]; do
    healthy_count=0
    total_count=0
    while IFS= read -r node; do
        local_primals="$(get_node_env "$node" "PRIMALS")"
        [ -z "$local_primals" ] && continue
        for primal in $local_primals; do
            ((total_count++)) || true
            port_var="${primal^^}_PORT"
            port="$(get_node_env "$node" "$port_var")"
            [ -z "$port" ] && continue
            cname="$(container_name "$node")"
            node_ip="$(get_node_ip "$node")"
            if [ -n "$node_ip" ]; then
                if echo '{"jsonrpc":"2.0","method":"health.liveness","params":{},"id":1}' | \
                    timeout 3 nc -w 2 "$node_ip" "$port" 2>/dev/null | grep -q '"result"'; then
                    ((healthy_count++)) || true
                fi
            fi
        done
    done < <(get_node_names)

    log "Health: $healthy_count/$total_count primals responding (${elapsed}s / ${TIMEOUT_SECS}s)"

    if [ "$healthy_count" -ge "$total_count" ] && [ "$total_count" -gt 0 ]; then
        all_healthy=true
        break
    fi

    sleep "$interval"
    elapsed=$((elapsed + interval))
done

if [ "$all_healthy" = true ]; then
    log "All primals healthy"
else
    log_warn "Timeout: not all primals responded. Proceeding with partial validation."
fi

echo "" >> "$RESULTS_DIR/health.log"
echo "healthy=$healthy_count total=$total_count elapsed=${elapsed}s" >> "$RESULTS_DIR/health.log"

# ── Step 4: Run Experiments ──────────────────────────────────────────────────

banner "Step 4/5: Run Cross-Gate Experiments"

# Determine the primary/tower node IP for REMOTE_GATE_HOST
PRIMARY_NODE=""
for try_node in "node-tower" "node-primary"; do
    if get_node_names | grep -q "$try_node"; then
        PRIMARY_NODE="$try_node"
        break
    fi
done

if [ -z "$PRIMARY_NODE" ]; then
    PRIMARY_NODE="$(get_node_names | head -1)"
fi

GATE_IP="$(get_node_ip "$PRIMARY_NODE")"
log_info "REMOTE_GATE_HOST=$GATE_IP (from $PRIMARY_NODE)"

# Extract FAMILY_ID from topology to match what primals were started with
TOPO_FAMILY_ID="$(get_node_env "$PRIMARY_NODE" "FAMILY_ID")"
TOPO_FAMILY_ID="${TOPO_FAMILY_ID:-$LAB_NAME}"
log_info "FAMILY_ID=$TOPO_FAMILY_ID (from topology)"

exp_pass=0
exp_fail=0
exp_skip=0

run_experiment() {
    local bin_name="$1"
    local desc="$2"
    local pkg_name="${3:-}"
    log "Running $bin_name ($desc)..."

    # Determine package flag: experiments live in primalspring-expNNN packages
    local pkg_flag=""
    if [ -n "$pkg_name" ]; then
        pkg_flag="-p $pkg_name"
    fi

    if [ "$SKIP_BUILD" != true ]; then
        if ! cargo build $pkg_flag --bin "$bin_name" --manifest-path "$PRIMALSPRING_ROOT/Cargo.toml" 2>"$RESULTS_DIR/${bin_name}_build.log"; then
            log_warn "  Build failed for $bin_name (see ${bin_name}_build.log)"
            ((exp_skip++)) || true
            return
        fi
    fi

    local exit_code=0
    REMOTE_GATE_HOST="$GATE_IP" \
    REMOTE_BEARDOG_PORT="${BEARDOG_PORT:-9100}" \
    REMOTE_SONGBIRD_PORT="${SONGBIRD_PORT:-9200}" \
    BEARDOG_PORT="${BEARDOG_PORT:-9100}" \
    SONGBIRD_PORT="${SONGBIRD_PORT:-9200}" \
    NESTGATE_PORT="${NESTGATE_PORT:-9300}" \
    TOADSTOOL_PORT="${TOADSTOOL_PORT:-9400}" \
    SQUIRREL_PORT="${SQUIRREL_PORT:-9500}" \
    FAMILY_ID="$TOPO_FAMILY_ID" \
    timeout 60 cargo run $pkg_flag --bin "$bin_name" --manifest-path "$PRIMALSPRING_ROOT/Cargo.toml" \
        > "$RESULTS_DIR/${bin_name}.log" 2>&1 || exit_code=$?

    if [ "$exit_code" -eq 0 ]; then
        log "  PASS: $bin_name"
        ((exp_pass++)) || true
    else
        log_err "  FAIL: $bin_name (exit $exit_code, see ${bin_name}.log)"
        ((exp_fail++)) || true
    fi
}

# Core cross-gate experiments
run_experiment "exp074_cross_gate_health" "remote NUCLEUS health probe" "primalspring-exp074"
run_experiment "exp073_lan_covalent_mesh" "LAN covalent mesh + BirdSong" "primalspring-exp073"

# Extra experiments if requested
if [ -n "$EXTRA_EXPERIMENTS" ]; then
    IFS=',' read -ra EXTRA_LIST <<< "$EXTRA_EXPERIMENTS"
    for exp_bin in "${EXTRA_LIST[@]}"; do
        run_experiment "$exp_bin" "user-specified"
    done
fi

# ── Step 5: Results Summary ──────────────────────────────────────────────────

banner "Step 5/5: Results Summary"

cat > "$RESULTS_DIR/summary.txt" << EOF
primalSpring Local Validation Results
=====================================
Date:       $(date -u +"%Y-%m-%dT%H:%M:%SZ")
Topology:   $TOPOLOGY
Hypervisor: $HYPERVISOR
Lab:        $LAB_NAME
Gate IP:    $GATE_IP ($PRIMARY_NODE)

Health:
  Healthy: $healthy_count / $total_count primals
  Wait:    ${elapsed}s

Experiments:
  Pass:    $exp_pass
  Fail:    $exp_fail
  Skip:    $exp_skip

EOF

cat "$RESULTS_DIR/summary.txt"

if [ "$exp_fail" -gt 0 ]; then
    log_err "Validation completed with $exp_fail failure(s)"
    log_info "Logs in: $RESULTS_DIR/"
    exit 1
elif [ "$exp_pass" -eq 0 ]; then
    log_warn "No experiments passed (all skipped or none ran)"
    exit 1
else
    log "Validation passed: $exp_pass experiment(s)"
fi

log_info "Full logs: $RESULTS_DIR/"
echo ""
