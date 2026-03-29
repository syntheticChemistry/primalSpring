#!/usr/bin/env bash
# chaos-inject.sh — inject chaos conditions into a running benchScale lab
#
# Actions:
#   partition <container_a> <container_b>  — iptables drop between containers
#   heal      <container_a> <container_b>  — remove iptables drop
#   kill      <container> <primal>         — kill a primal process in container
#   disk-fill <container> <path> <size_mb> — fill disk at path
#   disk-heal <container> <path>           — remove disk fill
#   slow-dns  <container> <delay_ms>       — inject DNS latency
#   clock-drift <container> <offset_sec>   — (requires faketime) shift clock
#
# Requires: docker, root/sudo for iptables actions
#
# Usage:
#   scripts/chaos-inject.sh partition node-tower node-spring
#   scripts/chaos-inject.sh kill node-tower beardog
#   scripts/chaos-inject.sh heal node-tower node-spring

set -euo pipefail

readonly SCRIPT_NAME="$(basename "$0")"

log_info()  { echo "[chaos-inject] INFO  $*"; }
log_warn()  { echo "[chaos-inject] WARN  $*" >&2; }
log_error() { echo "[chaos-inject] ERROR $*" >&2; }

usage() {
    cat <<EOF
Usage: $SCRIPT_NAME <action> [args...]

Actions:
  partition <container_a> <container_b>  Drop traffic between two containers
  heal      <container_a> <container_b>  Restore traffic between two containers
  kill      <container> <process_name>   Kill a process inside a container
  disk-fill <container> <path> <size_mb> Create a large file to fill disk
  disk-heal <container> <path>           Remove disk fill file
  slow-dns  <container> <delay_ms>       Add latency to DNS resolution
  clock-drift <container> <offset_sec>   Shift container clock (needs faketime)

Examples:
  $SCRIPT_NAME partition node-tower node-spring
  $SCRIPT_NAME kill node-tower beardog_primal
  $SCRIPT_NAME disk-fill node-tower /tmp/fill 10
  $SCRIPT_NAME heal node-tower node-spring
EOF
    exit 1
}

get_container_ip() {
    local container="$1"
    docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' "$container" 2>/dev/null || {
        log_error "Cannot get IP for container: $container"
        return 1
    }
}

do_partition() {
    local container_a="$1" container_b="$2"
    local ip_b
    ip_b="$(get_container_ip "$container_b")"
    log_info "Partitioning $container_a ←✕→ $container_b (dropping traffic to $ip_b)"
    docker exec "$container_a" iptables -A OUTPUT -d "$ip_b" -j DROP 2>/dev/null || \
        docker exec "$container_a" sh -c "ip route add unreachable $ip_b" 2>/dev/null || {
            log_warn "iptables/ip not available in $container_a — partition may not work"
            return 1
        }

    local ip_a
    ip_a="$(get_container_ip "$container_a")"
    docker exec "$container_b" iptables -A OUTPUT -d "$ip_a" -j DROP 2>/dev/null || \
        docker exec "$container_b" sh -c "ip route add unreachable $ip_a" 2>/dev/null || true
    log_info "Partition active: $container_a ←✕→ $container_b"
}

do_heal() {
    local container_a="$1" container_b="$2"
    local ip_b
    ip_b="$(get_container_ip "$container_b")"
    log_info "Healing partition: $container_a ←→ $container_b"
    docker exec "$container_a" iptables -D OUTPUT -d "$ip_b" -j DROP 2>/dev/null || \
        docker exec "$container_a" sh -c "ip route del unreachable $ip_b" 2>/dev/null || true

    local ip_a
    ip_a="$(get_container_ip "$container_a")"
    docker exec "$container_b" iptables -D OUTPUT -d "$ip_a" -j DROP 2>/dev/null || \
        docker exec "$container_b" sh -c "ip route del unreachable $ip_a" 2>/dev/null || true
    log_info "Partition healed: $container_a ←→ $container_b"
}

do_kill() {
    local container="$1" process="$2"
    log_info "Killing process '$process' in container '$container'"
    docker exec "$container" pkill -f "$process" 2>/dev/null || {
        log_warn "Process '$process' not found in $container (may already be dead)"
        return 0
    }
    log_info "Process '$process' killed in $container"
}

do_disk_fill() {
    local container="$1" path="$2" size_mb="$3"
    log_info "Filling disk: $container:$path (${size_mb}MB)"
    docker exec "$container" dd if=/dev/zero of="${path}/chaos_fill" bs=1M count="$size_mb" 2>/dev/null || {
        log_warn "dd failed — disk may already be full or path doesn't exist"
        return 1
    }
    log_info "Disk filled: $container:$path (${size_mb}MB)"
}

do_disk_heal() {
    local container="$1" path="$2"
    log_info "Healing disk fill: $container:$path"
    docker exec "$container" rm -f "${path}/chaos_fill" 2>/dev/null || true
    log_info "Disk fill removed: $container:$path"
}

do_slow_dns() {
    local container="$1" delay_ms="$2"
    log_info "Injecting DNS latency: ${delay_ms}ms in $container"
    docker exec "$container" tc qdisc add dev eth0 root netem delay "${delay_ms}ms" 2>/dev/null || {
        log_warn "tc not available in $container — DNS slowdown not applied"
        return 1
    }
    log_info "DNS latency active: ${delay_ms}ms in $container"
}

do_clock_drift() {
    local container="$1" offset_sec="$2"
    log_info "Shifting clock by ${offset_sec}s in $container"
    docker exec "$container" sh -c "date -s @\$(( \$(date +%s) + $offset_sec ))" 2>/dev/null || {
        log_warn "Cannot set date in $container — likely read-only or no permission"
        return 1
    }
    log_info "Clock shifted by ${offset_sec}s in $container"
}

# ── Main ───────────────────────────────────────────────────────────────
[ $# -lt 1 ] && usage

action="$1"
shift

case "$action" in
    partition)
        [ $# -lt 2 ] && { log_error "partition requires 2 container names"; usage; }
        do_partition "$1" "$2"
        ;;
    heal)
        [ $# -lt 2 ] && { log_error "heal requires 2 container names"; usage; }
        do_heal "$1" "$2"
        ;;
    kill)
        [ $# -lt 2 ] && { log_error "kill requires container and process name"; usage; }
        do_kill "$1" "$2"
        ;;
    disk-fill)
        [ $# -lt 3 ] && { log_error "disk-fill requires container, path, and size_mb"; usage; }
        do_disk_fill "$1" "$2" "$3"
        ;;
    disk-heal)
        [ $# -lt 2 ] && { log_error "disk-heal requires container and path"; usage; }
        do_disk_heal "$1" "$2"
        ;;
    slow-dns)
        [ $# -lt 2 ] && { log_error "slow-dns requires container and delay_ms"; usage; }
        do_slow_dns "$1" "$2"
        ;;
    clock-drift)
        [ $# -lt 2 ] && { log_error "clock-drift requires container and offset_sec"; usage; }
        do_clock_drift "$1" "$2"
        ;;
    *)
        log_error "Unknown action: $action"
        usage
        ;;
esac
