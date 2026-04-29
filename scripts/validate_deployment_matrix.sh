#!/usr/bin/env bash
# validate_deployment_matrix.sh — Run primal compositions across the deployment matrix
#
# Reads config/deployment_matrix.toml and executes validation cells: each cell is
# a combination of topology × architecture × network preset × transport mode.
#
# This catches cross-arch, cross-transport, and degraded-network issues BEFORE
# deploying to real gates (Pixel, remote NUCs, friend gates).
#
# Usage:
#   ./scripts/validate_deployment_matrix.sh --cell tower-x86-homelan-uds
#   ./scripts/validate_deployment_matrix.sh --tier P0
#   ./scripts/validate_deployment_matrix.sh --all
#   ./scripts/validate_deployment_matrix.sh --list
#
# Requires: Docker (for Tier 1 cells), qemu-user-static (for aarch64 cells)

set -euo pipefail

GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
BOLD='\033[1m'
CYAN='\033[0;36m'
NC='\033[0m'

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PRIMALSPRING_ROOT="$(dirname "$SCRIPT_DIR")"
MATRIX_FILE="$PRIMALSPRING_ROOT/config/deployment_matrix.toml"

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

MODE=""         # cell, tier, all, list
CELL_ID=""
TIER_FILTER=""
KEEP_LABS=false
SKIP_BUILD=false
DRY_RUN=false
TIMEOUT_SECS=120

usage() {
    cat << EOF
Usage: $0 [options]

Run primal deployment validation across the matrix defined in
config/deployment_matrix.toml.

Selection (pick one):
    --cell <id>         Run a single matrix cell by ID
    --tier <P0|P1|P2>   Run all cells at a priority tier
    --all               Run all non-blocked cells
    --list              List all cells with status (no execution)

Options:
    --benchscale <path>  Path to benchScale root (default: auto-detect)
    --plasmidbin <path>  Path to plasmidBin root (default: auto-detect)
    --keep               Keep labs running after each cell
    --skip-build         Skip cargo build for experiments
    --dry-run            Show what would run without executing
    --timeout <secs>     Per-cell health timeout (default: $TIMEOUT_SECS)
    --help               Show this help

Examples:
    $0 --list                          # See the full matrix
    $0 --cell tower-x86-homelan-uds    # Run the golden-path cell
    $0 --tier P0                       # Run all must-pass cells
    $0 --tier P0 --dry-run             # Preview P0 cells
    $0 --all --keep                    # Full matrix, keep labs for debugging

EOF
    exit 1
}

while [[ $# -gt 0 ]]; do
    case $1 in
        --cell)         MODE="cell"; CELL_ID="$2"; shift 2 ;;
        --tier)         MODE="tier"; TIER_FILTER="$2"; shift 2 ;;
        --all)          MODE="all"; shift ;;
        --list)         MODE="list"; shift ;;
        --benchscale)   BENCHSCALE_DIR="$2"; shift 2 ;;
        --plasmidbin)   PLASMIDBIN_DIR="$2"; shift 2 ;;
        --keep)         KEEP_LABS=true; shift ;;
        --skip-build)   SKIP_BUILD=true; shift ;;
        --dry-run)      DRY_RUN=true; shift ;;
        --timeout)      TIMEOUT_SECS="$2"; shift 2 ;;
        --help)         usage ;;
        *)              echo -e "${RED}Unknown option: $1${NC}"; usage ;;
    esac
done

if [ -z "$MODE" ]; then
    echo -e "${RED}Specify --cell, --tier, --all, or --list${NC}"
    usage
fi

log()      { echo -e "${GREEN}[matrix]${NC} $1"; }
log_info() { echo -e "${BLUE}[matrix]${NC} $1"; }
log_warn() { echo -e "${YELLOW}[matrix]${NC} $1"; }
log_err()  { echo -e "${RED}[matrix]${NC} $1"; }
banner()   { echo -e "\n${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"; echo -e " $1"; echo -e "${BOLD}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}\n"; }

# ── TOML Parsing (minimal, line-based) ──────────────────────────────────────
# We parse the [[cells]] sections from the TOML file. Each cell has fields
# on subsequent lines until the next [[cells]] or [[blockers]] section.

parse_cells() {
    local file="$1"
    local in_cell=false
    local cell_id="" topology="" arch="" preset="" transport="" experiments="" priority="" status="" notes=""

    while IFS= read -r line; do
        line="${line%%#*}"  # strip comments
        line="${line#"${line%%[![:space:]]*}"}"  # strip leading whitespace

        if [[ "$line" == "[[cells]]" ]]; then
            if [ "$in_cell" = true ] && [ -n "$cell_id" ]; then
                echo "$cell_id|$topology|$arch|$preset|$transport|$experiments|$priority|$status|$notes"
            fi
            in_cell=true
            cell_id="" topology="" arch="" preset="" transport="" experiments="" priority="" status="" notes=""
            continue
        fi

        if [[ "$line" == "[[blockers]]" ]] || [[ "$line" == "["* && "$line" != "[[cells]]" ]]; then
            if [ "$in_cell" = true ] && [ -n "$cell_id" ]; then
                echo "$cell_id|$topology|$arch|$preset|$transport|$experiments|$priority|$status|$notes"
            fi
            in_cell=false
            continue
        fi

        if [ "$in_cell" = true ]; then
            local key val
            key="$(echo "$line" | cut -d= -f1 | tr -d ' ')"
            val="$(echo "$line" | cut -d= -f2- | sed 's/^[[:space:]]*"//;s/"[[:space:]]*$//')"
            case "$key" in
                id)          cell_id="$val" ;;
                topology)    topology="$val" ;;
                arch)        arch="$val" ;;
                preset)      preset="$val" ;;
                transport)   transport="$val" ;;
                experiments) experiments="$val" ;;
                priority)    priority="$val" ;;
                status)      status="$val" ;;
                notes)       notes="$val" ;;
            esac
        fi
    done < "$file"

    if [ "$in_cell" = true ] && [ -n "$cell_id" ]; then
        echo "$cell_id|$topology|$arch|$preset|$transport|$experiments|$priority|$status|$notes"
    fi
}

# ── Topology file lookup (matrix topology key → YAML filename) ──────────────

resolve_topology_file() {
    local topo_key="$1"
    local topo_file=""
    local in_topo=false current_key=""

    while IFS= read -r line; do
        line="${line%%#*}"
        line="${line#"${line%%[![:space:]]*}"}"

        if [[ "$line" =~ ^\[topologies\.(.+)\]$ ]]; then
            current_key="${BASH_REMATCH[1]}"
            if [ "$current_key" = "$topo_key" ]; then
                in_topo=true
            else
                in_topo=false
            fi
            continue
        fi

        if [[ "$line" == "["* ]]; then
            in_topo=false
            continue
        fi

        if [ "$in_topo" = true ]; then
            local key val
            key="$(echo "$line" | cut -d= -f1 | tr -d ' ')"
            val="$(echo "$line" | cut -d= -f2- | tr -d ' "')"
            if [ "$key" = "file" ]; then
                topo_file="$val"
            fi
        fi
    done < "$MATRIX_FILE"

    echo "$topo_file"
}

# ── List mode ───────────────────────────────────────────────────────────────

list_matrix() {
    banner "Deployment Validation Matrix"

    printf "${BOLD}%-35s %-8s %-10s %-12s %-10s %-8s${NC}\n" \
        "CELL ID" "ARCH" "PRESET" "TRANSPORT" "PRIORITY" "STATUS"
    echo "──────────────────────────────────────────────────────────────────────────────────────────────"

    parse_cells "$MATRIX_FILE" | while IFS='|' read -r id topo arch preset transport experiments priority status notes; do
        local color="$NC"
        case "$status" in
            pass)     color="$GREEN" ;;
            fail)     color="$RED" ;;
            blocked)  color="$YELLOW" ;;
            untested) color="$CYAN" ;;
        esac
        printf "%-35s %-8s %-10s %-12s %-10s ${color}%-8s${NC}\n" \
            "$id" "$arch" "$preset" "$transport" "$priority" "$status"
    done

    echo ""
    echo "Cells: $(parse_cells "$MATRIX_FILE" | wc -l)"
    echo "Pass:  $(parse_cells "$MATRIX_FILE" | grep '|pass|' | wc -l)"
    echo "Blocked: $(parse_cells "$MATRIX_FILE" | grep '|blocked|' | wc -l)"
    echo "Untested: $(parse_cells "$MATRIX_FILE" | grep '|untested|' | wc -l)"
}

# ── Run a single cell ──────────────────────────────────────────────────────

run_cell() {
    local id="$1" topology="$2" arch="$3" preset="$4" transport="$5" experiments="$6" priority="$7" status="$8" notes="$9"

    if [ "$status" = "blocked" ]; then
        log_warn "SKIP (blocked): $id — $notes"
        echo "SKIP|$id|blocked|$notes" >> "$RESULTS_DIR/matrix_results.txt"
        return 0
    fi

    local topo_file
    topo_file="$(resolve_topology_file "$topology")"
    if [ -z "$topo_file" ]; then
        log_err "Cannot resolve topology file for key: $topology"
        echo "ERROR|$id|bad_topology|$topology" >> "$RESULTS_DIR/matrix_results.txt"
        return 1
    fi

    banner "Cell: $id  [$arch / $preset / $transport]"
    log_info "Topology:    $topo_file"
    log_info "Arch:        $arch"
    log_info "Preset:      $preset"
    log_info "Transport:   $transport"
    log_info "Experiments: $experiments"
    log_info "Priority:    $priority"
    log_info "Notes:       $notes"

    if [ "$DRY_RUN" = true ]; then
        log "DRY RUN — would execute validate_local_lab.sh with:"
        echo "  --topology $topo_file --timeout $TIMEOUT_SECS"
        [ "$arch" = "aarch64" ] && echo "  (Docker --platform linux/arm64, deploy --arch aarch64)"
        [ "$transport" = "tcp_first" ] && echo "  (TCP-only transport env vars injected)"
        echo "DRYRUN|$id|would_run|$notes" >> "$RESULTS_DIR/matrix_results.txt"
        return 0
    fi

    # Check aarch64 prerequisites
    if [ "$arch" = "aarch64" ]; then
        if ! docker run --rm --privileged multiarch/qemu-user-static --reset -p yes &>/dev/null 2>&1; then
            if ! [ -f /proc/sys/fs/binfmt_misc/qemu-aarch64 ]; then
                log_warn "SKIP (no qemu-user-static): $id — install with: docker run --privileged multiarch/qemu-user-static --reset -p yes"
                echo "SKIP|$id|no_qemu|aarch64 emulation not available" >> "$RESULTS_DIR/matrix_results.txt"
                return 0
            fi
        fi
    fi

    local validate_args=()
    validate_args+=(--topology "$topo_file")
    validate_args+=(--timeout "$TIMEOUT_SECS")
    [ "$KEEP_LABS" = true ] && validate_args+=(--keep)
    [ "$SKIP_BUILD" = true ] && validate_args+=(--skip-build)
    [ -n "$BENCHSCALE_DIR" ] && validate_args+=(--benchscale "$BENCHSCALE_DIR")
    [ -n "$PLASMIDBIN_DIR" ] && validate_args+=(--plasmidbin "$PLASMIDBIN_DIR")

    # Set arch-specific env for the deploy script
    local cell_env=()
    if [ "$arch" = "aarch64" ]; then
        cell_env+=(DEPLOY_ARCH=aarch64)
        cell_env+=(DOCKER_PLATFORM="linux/arm64")
    fi

    # Set transport env overrides for TCP-first cells
    if [ "$transport" = "tcp_first" ]; then
        cell_env+=(PRIMAL_TRANSPORT=tcp)
        cell_env+=(BIOMEOS_MODE=api)
        cell_env+=(BIOMEOS_TCP_ONLY=true)
    fi

    local cell_start
    cell_start=$(date +%s)

    local exit_code=0
    if [ ${#cell_env[@]} -gt 0 ]; then
        env "${cell_env[@]}" "$PRIMALSPRING_ROOT/scripts/validate_local_lab.sh" "${validate_args[@]}" 2>&1 | \
            tee "$RESULTS_DIR/${id}.log" || exit_code=$?
    else
        "$PRIMALSPRING_ROOT/scripts/validate_local_lab.sh" "${validate_args[@]}" 2>&1 | \
            tee "$RESULTS_DIR/${id}.log" || exit_code=$?
    fi

    local cell_end elapsed
    cell_end=$(date +%s)
    elapsed=$((cell_end - cell_start))

    if [ $exit_code -eq 0 ]; then
        log "${GREEN}PASS${NC}: $id (${elapsed}s)"
        echo "PASS|$id|${elapsed}s|$notes" >> "$RESULTS_DIR/matrix_results.txt"
    else
        log_err "FAIL: $id (exit $exit_code, ${elapsed}s)"
        echo "FAIL|$id|exit_${exit_code}|${elapsed}s|$notes" >> "$RESULTS_DIR/matrix_results.txt"
    fi

    return $exit_code
}

# ── Main execution ──────────────────────────────────────────────────────────

if [ "$MODE" = "list" ]; then
    list_matrix
    exit 0
fi

if [ -z "$BENCHSCALE_DIR" ]; then
    log_err "Cannot find benchScale. Set --benchscale <path>"
    exit 1
fi

if [ -z "$PLASMIDBIN_DIR" ]; then
    log_err "Cannot find plasmidBin. Set --plasmidbin <path>"
    exit 1
fi

RESULTS_DIR="$PRIMALSPRING_ROOT/validation_results/matrix-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$RESULTS_DIR"

banner "ecoPrimals Deployment Validation Matrix"
log_info "Mode:        $MODE"
log_info "Matrix:      $MATRIX_FILE"
log_info "benchScale:  $BENCHSCALE_DIR"
log_info "plasmidBin:  $PLASMIDBIN_DIR"
log_info "Results:     $RESULTS_DIR"
echo ""

total=0
passed=0
failed=0
skipped=0
start_time=$(date +%s)

# Read cells into an array to avoid subshell variable scoping
mapfile -t CELL_LINES < <(parse_cells "$MATRIX_FILE")

for cell_line in "${CELL_LINES[@]}"; do
    IFS='|' read -r id topo arch preset transport experiments priority status notes <<< "$cell_line"

    # Filter by mode
    case "$MODE" in
        cell)
            [ "$id" != "$CELL_ID" ] && continue
            ;;
        tier)
            [ "$priority" != "$TIER_FILTER" ] && continue
            ;;
        all)
            ;;
    esac

    total=$((total + 1))

    if run_cell "$id" "$topo" "$arch" "$preset" "$transport" "$experiments" "$priority" "$status" "$notes"; then
        if [ "$status" = "blocked" ] || [ "$DRY_RUN" = true ]; then
            skipped=$((skipped + 1))
        else
            passed=$((passed + 1))
        fi
    else
        failed=$((failed + 1))
    fi
done

end_time=$(date +%s)
total_elapsed=$((end_time - start_time))

# ── Summary ─────────────────────────────────────────────────────────────────

banner "Matrix Results"
echo ""
if [ -f "$RESULTS_DIR/matrix_results.txt" ]; then
    printf "${BOLD}%-8s %-35s %-15s %s${NC}\n" "STATUS" "CELL" "TIMING" "NOTES"
    echo "────────────────────────────────────────────────────────────────────────────────"
    while IFS='|' read -r result_status cell_id detail extra; do
        color="$NC"
        case "$result_status" in
            PASS)    color="$GREEN" ;;
            FAIL)    color="$RED" ;;
            SKIP)    color="$YELLOW" ;;
            DRYRUN)  color="$CYAN" ;;
            ERROR)   color="$RED" ;;
        esac
        printf "${color}%-8s${NC} %-35s %-15s %s\n" "$result_status" "$cell_id" "$detail" "$extra"
    done < "$RESULTS_DIR/matrix_results.txt"
else
    log_warn "No results file generated (no matching cells?)"
fi

echo ""
log "Total: $total  Pass: $passed  Fail: $failed  Skip: $skipped  (${total_elapsed}s)"
echo ""
log_info "Full results: $RESULTS_DIR/"

if [ $failed -gt 0 ]; then
    exit 1
fi
