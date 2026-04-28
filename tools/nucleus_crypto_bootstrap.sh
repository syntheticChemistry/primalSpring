#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# nucleus_crypto_bootstrap.sh — Two-tier crypto key derivation and wiring
#
# Run after NUCLEUS is up. Derives base encryption keys from published
# primal DNA (seed fingerprints), stores per-atomic purpose keys via
# BearDog secrets, and verifies crypto round-trips.
#
# Usage:
#   ./nucleus_crypto_bootstrap.sh [--family-id ID] [--socket-dir DIR]
#   ./nucleus_crypto_bootstrap.sh --verify-only
#
# Environment:
#   FAMILY_ID          — deployment namespace (default: desktop-nucleus)
#   SOCKET_DIR         — UDS directory (default: $XDG_RUNTIME_DIR/biomeos)
#   BEARDOG_FAMILY_SEED — random entropy for Tier 1 family key

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SPRING_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ECO_ROOT="$(cd "$SPRING_ROOT/../.." && pwd)"

FAMILY_ID="${FAMILY_ID:-desktop-nucleus}"
SOCKET_DIR="${SOCKET_DIR:-${XDG_RUNTIME_DIR:-/tmp}/biomeos}"
SEED_FINGERPRINTS="$SPRING_ROOT/validation/seed_fingerprints.toml"
VERIFY_ONLY=false

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m'

log()  { echo -e "${CYAN}[crypto-bootstrap]${NC} $*"; }
ok()   { echo -e "${GREEN}[crypto-bootstrap]${NC} $*"; }
warn() { echo -e "${YELLOW}[crypto-bootstrap]${NC} $*"; }
err()  { echo -e "${RED}[crypto-bootstrap]${NC} $*" >&2; }

while [[ $# -gt 0 ]]; do
    case "$1" in
        --family-id)   FAMILY_ID="$2"; shift 2 ;;
        --socket-dir)  SOCKET_DIR="$2"; shift 2 ;;
        --verify-only) VERIFY_ONLY=true; shift ;;
        *) err "Unknown flag: $1"; exit 1 ;;
    esac
done

sock() { echo "$SOCKET_DIR/${1}-${FAMILY_ID}.sock"; }

BEARDOG_SOCK="$(sock beardog)"

ipc_call() {
    local sock="$1" method="$2" params="${3:-{}}"
    local payload
    payload=$(printf '{"jsonrpc":"2.0","method":"%s","params":%s,"id":1}' "$method" "$params")
    if command -v socat &>/dev/null; then
        printf '%s\n' "$payload" | timeout 5 socat - "UNIX-CONNECT:$sock" 2>/dev/null || true
    elif command -v python3 &>/dev/null; then
        python3 -c "
import socket,sys,json
s=socket.socket(socket.AF_UNIX,socket.SOCK_STREAM)
s.settimeout(5)
try:
    s.connect('$sock')
    s.sendall(sys.stdin.buffer.read())
    chunks=[]
    while True:
        try:
            d=s.recv(65536)
            if not d: break
            chunks.append(d)
        except socket.timeout: break
    sys.stdout.buffer.write(b''.join(chunks))
except: pass
finally: s.close()
" <<< "$payload" 2>/dev/null || true
    fi
}

result_field() {
    local json="$1" field="$2"
    echo "$json" | python3 -c "
import json,sys
try:
    d=json.load(sys.stdin)
    r=d.get('result',d)
    if isinstance(r,dict):
        print(r.get('$field',''))
    else:
        print(r if r else '')
except: pass
" 2>/dev/null || true
}

has_error() {
    echo "$1" | grep -q '"error"' 2>/dev/null
}

# ── Tier 0: Published Seed Fingerprints ───────────────────────────────

CORE_PRIMALS=(beardog songbird toadstool barracuda coralreef nestgate rhizocrypt loamspine sweetgrass squirrel petaltongue biomeos)

ATOMIC_PURPOSE=(
    beardog:security
    songbird:discovery
    toadstool:compute
    barracuda:tensor
    coralreef:shader
    nestgate:storage
    rhizocrypt:dag
    loamspine:ledger
    sweetgrass:provenance
    squirrel:inference
    petaltongue:visualization
    biomeos:coordination
)

declare -A FINGERPRINTS=()

load_fingerprints() {
    log "── Loading published seed fingerprints ──"
    if [[ ! -f "$SEED_FINGERPRINTS" ]]; then
        err "Seed fingerprints not found: $SEED_FINGERPRINTS"
        return 1
    fi

    local count=0
    while IFS= read -r line; do
        line="${line%%#*}"
        line="$(echo "$line" | xargs 2>/dev/null || true)"
        [[ -z "$line" ]] && continue
        [[ "$line" == "["* ]] && continue

        local key val
        key="${line%%=*}"
        key="$(echo "$key" | xargs)"
        val="${line#*=}"
        val="$(echo "$val" | xargs | tr -d '"')"

        if [[ -n "$key" && -n "$val" && ${#val} -eq 64 ]]; then
            FINGERPRINTS[$key]="$val"
            count=$((count + 1))
        fi
    done < "$SEED_FINGERPRINTS"

    ok "Loaded $count seed fingerprints"
}

# ── Key Derivation ────────────────────────────────────────────────────
#
# Tier 0: base_key = HMAC-SHA256(published_fingerprint, "primal-nucleus-v1:$primal")
# Tier 1: family_key = HMAC-SHA256(base_key || FAMILY_SEED, "family-v1:$FAMILY_ID")
#
# We use BearDog's crypto.hmac_sha256 for all derivation so the key
# material never leaves the Tower atomic.

derive_base_key() {
    local primal="$1" fingerprint="$2"
    local info="primal-nucleus-v1:${primal}"
    local info_hex
    info_hex=$(printf '%s' "$info" | xxd -p | tr -d '\n')

    local resp
    resp=$(ipc_call "$BEARDOG_SOCK" "crypto.hmac_sha256" \
        "{\"key\":\"$fingerprint\",\"message\":\"$info_hex\"}")

    if has_error "$resp"; then
        warn "derive base key ($primal): BearDog hmac_sha256 error"
        echo ""
        return
    fi

    local hash
    hash=$(result_field "$resp" "hash")
    [[ -z "$hash" ]] && hash=$(result_field "$resp" "result")
    [[ -z "$hash" ]] && hash=$(echo "$resp" | grep -oP '"result"\s*:\s*"\K[^"]+' | head -1 || true)
    echo "$hash"
}

derive_family_key() {
    local base_key="$1" primal="$2"
    local family_seed="${BEARDOG_FAMILY_SEED:-}"
    if [[ -z "$family_seed" ]]; then
        warn "No BEARDOG_FAMILY_SEED — Tier 1 keys will equal Tier 0"
        echo "$base_key"
        return
    fi

    local combined="${base_key}${family_seed}"
    local info="family-v1:${FAMILY_ID}:${primal}"
    local info_hex
    info_hex=$(printf '%s' "$info" | xxd -p | tr -d '\n')

    local resp
    resp=$(ipc_call "$BEARDOG_SOCK" "crypto.hmac_sha256" \
        "{\"key\":\"$combined\",\"message\":\"$info_hex\"}")

    local hash
    hash=$(result_field "$resp" "hash")
    [[ -z "$hash" ]] && hash=$(result_field "$resp" "result")
    [[ -z "$hash" ]] && hash=$(echo "$resp" | grep -oP '"result"\s*:\s*"\K[^"]+' | head -1 || true)
    echo "$hash"
}

derive_purpose_key() {
    local family_key="$1" purpose="$2"
    local info="purpose-v1:${purpose}"
    local info_hex
    info_hex=$(printf '%s' "$info" | xxd -p | tr -d '\n')

    local resp
    resp=$(ipc_call "$BEARDOG_SOCK" "crypto.hmac_sha256" \
        "{\"key\":\"$family_key\",\"message\":\"$info_hex\"}")

    local hash
    hash=$(result_field "$resp" "hash")
    [[ -z "$hash" ]] && hash=$(result_field "$resp" "result")
    [[ -z "$hash" ]] && hash=$(echo "$resp" | grep -oP '"result"\s*:\s*"\K[^"]+' | head -1 || true)
    echo "$hash"
}

# ── Secrets Store ─────────────────────────────────────────────────────

store_purpose_key() {
    local purpose="$1" key_value="$2"
    local secret_id="nucleus:${FAMILY_ID}:purpose:${purpose}"

    local resp
    resp=$(ipc_call "$BEARDOG_SOCK" "secrets.store" \
        "{\"id\":\"$secret_id\",\"value\":\"$key_value\",\"metadata\":{\"purpose\":\"$purpose\",\"family_id\":\"$FAMILY_ID\",\"tier\":\"1\"}}")

    if has_error "$resp"; then
        warn "secrets.store ($purpose): $(echo "$resp" | head -c 120)"
        return 1
    fi
    return 0
}

retrieve_purpose_key() {
    local purpose="$1"
    local secret_id="nucleus:${FAMILY_ID}:purpose:${purpose}"

    local resp
    resp=$(ipc_call "$BEARDOG_SOCK" "secrets.retrieve" \
        "{\"id\":\"$secret_id\"}")

    if has_error "$resp"; then
        echo ""
        return 1
    fi
    result_field "$resp" "value"
}

# ── Crypto Round-Trip Tests ───────────────────────────────────────────

test_sign_verify() {
    local message="nucleus-crypto-bootstrap-test:$(date +%s)"
    local msg_b64
    msg_b64=$(printf '%s' "$message" | base64 -w0)

    log "  sign/verify round-trip..."
    local sign_resp
    sign_resp=$(ipc_call "$BEARDOG_SOCK" "crypto.sign" \
        "{\"message\":\"$msg_b64\"}")

    local signature
    signature=$(result_field "$sign_resp" "signature")
    if [[ -z "$signature" ]]; then
        signature=$(echo "$sign_resp" | grep -oP '"signature"\s*:\s*"\K[^"]+' | head -1 || true)
    fi
    if [[ -z "$signature" ]]; then
        warn "  sign failed: $(echo "$sign_resp" | head -c 120)"
        return 1
    fi

    local public_key
    public_key=$(result_field "$sign_resp" "public_key")
    [[ -z "$public_key" ]] && public_key=$(echo "$sign_resp" | grep -oP '"public_key"\s*:\s*"\K[^"]+' | head -1 || true)

    local verify_resp
    verify_resp=$(ipc_call "$BEARDOG_SOCK" "crypto.verify" \
        "{\"message\":\"$msg_b64\",\"signature\":\"$signature\",\"public_key\":\"${public_key:-}\"}")

    if echo "$verify_resp" | grep -qiE '"valid"\s*:\s*true|"result"\s*:\s*true'; then
        ok "  sign/verify: PASS"
        return 0
    else
        warn "  verify response: $(echo "$verify_resp" | head -c 120)"
        return 1
    fi
}

test_encrypt_decrypt() {
    local purpose_key="$1"
    local plaintext="nucleus-encrypt-test:$(date +%s)"
    local pt_b64
    pt_b64=$(printf '%s' "$plaintext" | base64 -w0)

    log "  encrypt/decrypt round-trip..."
    local enc_resp
    enc_resp=$(ipc_call "$BEARDOG_SOCK" "crypto.chacha20_poly1305_encrypt" \
        "{\"key\":\"$purpose_key\",\"plaintext\":\"$pt_b64\"}")

    local ciphertext nonce
    ciphertext=$(result_field "$enc_resp" "ciphertext")
    [[ -z "$ciphertext" ]] && ciphertext=$(echo "$enc_resp" | grep -oP '"ciphertext"\s*:\s*"\K[^"]+' | head -1 || true)
    nonce=$(result_field "$enc_resp" "nonce")
    [[ -z "$nonce" ]] && nonce=$(echo "$enc_resp" | grep -oP '"nonce"\s*:\s*"\K[^"]+' | head -1 || true)

    if [[ -z "$ciphertext" ]]; then
        warn "  encrypt failed: $(echo "$enc_resp" | head -c 120)"
        return 1
    fi

    local dec_resp
    dec_resp=$(ipc_call "$BEARDOG_SOCK" "crypto.chacha20_poly1305_decrypt" \
        "{\"key\":\"$purpose_key\",\"ciphertext\":\"$ciphertext\",\"nonce\":\"$nonce\"}")

    local decrypted
    decrypted=$(result_field "$dec_resp" "plaintext")
    [[ -z "$decrypted" ]] && decrypted=$(echo "$dec_resp" | grep -oP '"plaintext"\s*:\s*"\K[^"]+' | head -1 || true)

    if [[ -n "$decrypted" ]]; then
        local decoded
        decoded=$(echo "$decrypted" | base64 -d 2>/dev/null || true)
        if [[ "$decoded" == "$plaintext" ]]; then
            ok "  encrypt/decrypt: PASS (ChaCha20-Poly1305)"
            return 0
        else
            warn "  decrypt mismatch (got=$decoded expect=$plaintext)"
            return 1
        fi
    else
        warn "  decrypt failed: $(echo "$dec_resp" | head -c 120)"
        return 1
    fi
}

test_btsp_session() {
    log "  BTSP session create..."
    local resp
    resp=$(ipc_call "$BEARDOG_SOCK" "btsp.session.create" \
        "{\"family_seed\":\"${BEARDOG_FAMILY_SEED:-}\"}")

    local pub challenge
    pub=$(echo "$resp" | grep -oP '"server_ephemeral_pub"\s*:\s*"\K[^"]+' | head -1 || true)
    challenge=$(echo "$resp" | grep -oP '"challenge"\s*:\s*"\K[^"]+' | head -1 || true)

    if [[ -n "$pub" && -n "$challenge" ]]; then
        ok "  BTSP session: PASS (pub=${pub:0:16}... challenge=${challenge:0:16}...)"
        return 0
    else
        warn "  BTSP session: $(echo "$resp" | head -c 160)"
        return 1
    fi
}

test_nestgate_encrypt_at_rest() {
    local purpose_key="$1"
    local ng_sock="$(sock nestgate)"
    if [[ ! -S "$ng_sock" ]]; then
        warn "  NestGate not running — skip"
        return 1
    fi

    log "  NestGate encrypt-at-rest (composition-level)..."
    local test_key="crypto-bootstrap-test:$(date +%s)"
    local plaintext="sensitive-data-for-encrypt-at-rest-test"
    local pt_b64
    pt_b64=$(printf '%s' "$plaintext" | base64 -w0)

    local enc_resp
    enc_resp=$(ipc_call "$BEARDOG_SOCK" "crypto.chacha20_poly1305_encrypt" \
        "{\"key\":\"$purpose_key\",\"plaintext\":\"$pt_b64\"}")

    local ciphertext nonce
    ciphertext=$(echo "$enc_resp" | grep -oP '"ciphertext"\s*:\s*"\K[^"]+' | head -1 || true)
    nonce=$(echo "$enc_resp" | grep -oP '"nonce"\s*:\s*"\K[^"]+' | head -1 || true)

    if [[ -z "$ciphertext" || -z "$nonce" ]]; then
        warn "  encrypt step failed: $(echo "$enc_resp" | head -c 120)"
        return 1
    fi

    local envelope
    envelope=$(printf '{"v":1,"ct":"%s","n":"%s","alg":"chacha20-poly1305"}' \
        "$ciphertext" "$nonce")
    local env_b64
    env_b64=$(printf '%s' "$envelope" | base64 -w0)

    local store_resp
    store_resp=$(ipc_call "$ng_sock" "storage.store" \
        "{\"key\":\"$test_key\",\"value\":\"$env_b64\",\"metadata\":{\"encrypted\":true}}")

    if has_error "$store_resp"; then
        warn "  NestGate store failed: $(echo "$store_resp" | head -c 120)"
        return 1
    fi
    ok "    stored encrypted envelope (${#env_b64} bytes)"

    local retrieve_resp
    retrieve_resp=$(ipc_call "$ng_sock" "storage.retrieve" "{\"key\":\"$test_key\"}")

    local stored_val
    stored_val=$(echo "$retrieve_resp" | python3 -c "
import json,sys
try:
    d=json.load(sys.stdin)
    r=d.get('result',d)
    print(r.get('value','') if isinstance(r,dict) else r if isinstance(r,str) else '')
except: pass
" 2>/dev/null || true)

    if [[ -z "$stored_val" ]]; then
        warn "  NestGate retrieve failed: $(echo "$retrieve_resp" | head -c 120)"
        return 1
    fi

    local decoded_envelope
    decoded_envelope=$(echo "$stored_val" | base64 -d 2>/dev/null || true)
    local retrieved_ct
    retrieved_ct=$(echo "$decoded_envelope" | grep -oP '"ct"\s*:\s*"\K[^"]+' || true)
    local retrieved_n
    retrieved_n=$(echo "$decoded_envelope" | grep -oP '"n"\s*:\s*"\K[^"]+' || true)

    local dec_resp
    dec_resp=$(ipc_call "$BEARDOG_SOCK" "crypto.chacha20_poly1305_decrypt" \
        "{\"key\":\"$purpose_key\",\"ciphertext\":\"$retrieved_ct\",\"nonce\":\"$retrieved_n\"}")

    local decrypted_b64
    decrypted_b64=$(echo "$dec_resp" | python3 -c "
import json,sys
try:
    d=json.load(sys.stdin)
    r=d.get('result',d)
    print(r.get('plaintext','') if isinstance(r,dict) else '')
except: pass
" 2>/dev/null || true)

    if [[ -n "$decrypted_b64" ]]; then
        local recovered
        recovered=$(echo "$decrypted_b64" | base64 -d 2>/dev/null || true)
        if [[ "$recovered" == "$plaintext" ]]; then
            ok "  NestGate encrypt-at-rest: PASS (store→encrypt→retrieve→decrypt)"
            ipc_call "$ng_sock" "storage.delete" "{\"key\":\"$test_key\"}" >/dev/null 2>&1 || true
            return 0
        else
            warn "  decrypt mismatch: got='$recovered' expected='$plaintext'"
        fi
    else
        warn "  decrypt step failed: $(echo "$dec_resp" | head -c 120)"
    fi

    log ""
    log "  ── NestGate Upstream Evolution Needed ──"
    log "  NestGate currently stores plaintext — composition-level encryption works"
    log "  but native encrypt-at-rest requires NestGate to evolve:"
    log "    1. Accept NESTGATE_ENCRYPTION_KEY env var or resolve via Tower"
    log "    2. Auto-encrypt on storage.store, auto-decrypt on storage.retrieve"
    log "    3. Store envelope format: {v:1, ct:<ciphertext>, n:<nonce>, alg:<algo>}"
    log "    4. Support key rotation via secrets.retrieve from BearDog"
    return 1
}

# ── Main Bootstrap ────────────────────────────────────────────────────

bootstrap() {
    log "══════════════════════════════════════════════"
    log "  NUCLEUS Two-Tier Crypto Bootstrap"
    log "  Family: $FAMILY_ID"
    log "  Socket: $SOCKET_DIR"
    log "══════════════════════════════════════════════"

    if [[ ! -S "$BEARDOG_SOCK" ]]; then
        err "BearDog socket not found: $BEARDOG_SOCK"
        err "Start NUCLEUS first: ./desktop_nucleus.sh start"
        exit 1
    fi

    load_fingerprints

    local total=0 derived=0 stored=0 failed=0

    log "── Tier 0+1: Deriving purpose keys ──"
    for entry in "${ATOMIC_PURPOSE[@]}"; do
        local primal="${entry%%:*}"
        local purpose="${entry#*:}"
        total=$((total + 1))

        local fp="${FINGERPRINTS[$primal]:-}"
        if [[ -z "$fp" ]]; then
            warn "  $primal: no fingerprint — SKIP"
            failed=$((failed + 1))
            continue
        fi

        log "  $primal ($purpose): fingerprint=${fp:0:16}..."

        local base_key
        base_key=$(derive_base_key "$primal" "$fp")
        if [[ -z "$base_key" ]]; then
            warn "  $primal: base key derivation failed"
            failed=$((failed + 1))
            continue
        fi
        ok "    Tier 0 base: ${base_key:0:16}..."

        local family_key
        family_key=$(derive_family_key "$base_key" "$primal")
        if [[ -z "$family_key" ]]; then
            warn "  $primal: family key derivation failed"
            failed=$((failed + 1))
            continue
        fi
        ok "    Tier 1 family: ${family_key:0:16}..."

        local purpose_key
        purpose_key=$(derive_purpose_key "$family_key" "$purpose")
        if [[ -z "$purpose_key" ]]; then
            warn "  $primal: purpose key derivation failed"
            failed=$((failed + 1))
            continue
        fi
        ok "    Purpose key ($purpose): ${purpose_key:0:16}..."
        derived=$((derived + 1))

        if store_purpose_key "$purpose" "$purpose_key"; then
            stored=$((stored + 1))
            ok "    Stored → secrets.store"
        else
            warn "    Store failed"
        fi
    done

    log ""
    log "── Key Derivation Summary ──"
    log "  Total primals: $total"
    log "  Keys derived:  $derived"
    log "  Keys stored:   $stored"
    log "  Failed:        $failed"

    log ""
    log "── Verifying stored keys ──"
    local verified=0
    for entry in "${ATOMIC_PURPOSE[@]}"; do
        local purpose="${entry#*:}"
        local retrieved
        retrieved=$(retrieve_purpose_key "$purpose")
        if [[ -n "$retrieved" ]]; then
            ok "  $purpose: retrieved (${retrieved:0:16}...)"
            verified=$((verified + 1))
        else
            warn "  $purpose: NOT found in secrets store"
        fi
    done
    log "  Verified: $verified/$stored"

    log ""
    log "── Crypto Round-Trip Tests ──"
    local crypto_pass=0 crypto_fail=0

    if test_sign_verify; then
        crypto_pass=$((crypto_pass + 1))
    else
        crypto_fail=$((crypto_fail + 1))
    fi

    local storage_key
    storage_key=$(retrieve_purpose_key "storage")
    if [[ -n "$storage_key" ]]; then
        if test_encrypt_decrypt "$storage_key"; then
            crypto_pass=$((crypto_pass + 1))
        else
            crypto_fail=$((crypto_fail + 1))
        fi
    else
        warn "  encrypt/decrypt: SKIP (no storage purpose key)"
    fi

    if test_btsp_session; then
        crypto_pass=$((crypto_pass + 1))
    else
        crypto_fail=$((crypto_fail + 1))
    fi

    log ""
    log "── NestGate Encrypt-at-Rest Test ──"
    if [[ -n "$storage_key" ]]; then
        if test_nestgate_encrypt_at_rest "$storage_key"; then
            crypto_pass=$((crypto_pass + 1))
        else
            crypto_fail=$((crypto_fail + 1))
        fi
    else
        warn "  NestGate test: SKIP (no storage purpose key)"
    fi

    log ""
    log "══════════════════════════════════════════════"
    log "  Bootstrap Complete"
    log "  Keys: $derived/$total derived, $stored stored, $verified verified"
    log "  Crypto: $crypto_pass pass, $crypto_fail fail"
    log "══════════════════════════════════════════════"

    if (( failed > 0 || crypto_fail > 0 )); then
        warn "Some operations failed — see above for upstream gaps"
        return 1
    fi
    ok "All crypto tiers operational"
    return 0
}

verify() {
    log "── Verify-only mode ──"

    if [[ ! -S "$BEARDOG_SOCK" ]]; then
        err "BearDog socket not found: $BEARDOG_SOCK"
        exit 1
    fi

    local pass=0 fail=0

    for entry in "${ATOMIC_PURPOSE[@]}"; do
        local purpose="${entry#*:}"
        local retrieved
        retrieved=$(retrieve_purpose_key "$purpose")
        if [[ -n "$retrieved" ]]; then
            ok "  $purpose: ${retrieved:0:16}..."
            pass=$((pass + 1))
        else
            warn "  $purpose: NOT found"
            fail=$((fail + 1))
        fi
    done

    test_sign_verify && pass=$((pass + 1)) || fail=$((fail + 1))
    test_btsp_session && pass=$((pass + 1)) || fail=$((fail + 1))

    log "  Result: $pass pass, $fail fail"
    (( fail == 0 ))
}

if [[ "$VERIFY_ONLY" == "true" ]]; then
    verify
else
    bootstrap
fi
