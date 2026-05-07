#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# Graph method string validator
#
# Extracts all capability/method strings from TOML graph files and validates
# them against config/capability_registry.toml. Catches method name drift
# between graph specifications and the canonical registry.
#
# This addresses RP-1 from the projectNUCLEUS sovereignty handoff: graph method
# names should be validated against live primal dispatch tables.
#
# Two categories of unregistered methods:
#   DRIFT  — primal-domain methods missing from the registry (exit 1)
#   SPRING — spring-domain capabilities declared in cell graphs (advisory only)
#
# Usage: tools/check_graph_methods.sh [--strict] [--fix]
#   --strict: treat spring-domain methods as errors too
#   --fix:    print unregistered methods grouped by domain

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

REGISTRY="config/capability_registry.toml"
GRAPH_DIR="graphs"
STRICT=false

for arg in "$@"; do
    [[ "$arg" == "--strict" ]] && STRICT=true
done

if [[ ! -f "$REGISTRY" ]]; then
    echo "FAIL: $REGISTRY not found"
    exit 1
fi

if [[ ! -d "$GRAPH_DIR" ]]; then
    echo "FAIL: $GRAPH_DIR directory not found"
    exit 1
fi

# Spring-domain prefixes: capabilities owned by springs, not primals.
# These appear in cell graphs as spring-declared capabilities and are
# not expected in the primal capability registry.
SPRING_DOMAINS="activation|analysis|app|baseline|calibration|commit|community|data|defense|domain|ecology|ecosystem|game|health\.(aggregate|clinical|de_identify|genomics|pharmacology)|math\.(log2)|measurement|metadata|metrics|model|motor|net\.|neural|physics|pipeline|punch|relay|response|rootpulse|science|sensor|tolerances|validate"

REGISTERED=$(grep -oP '^\s+"[a-z][a-z0-9_.]+[a-z0-9]+"' "$REGISTRY" | tr -d ' "' | sort -u)

GRAPH_METHODS=$(rg -o '"[a-z][a-z_]+\.[a-z][a-z_.]*"' "$GRAPH_DIR/" \
    --glob '*.toml' --no-filename 2>/dev/null \
    | tr -d '"' \
    | grep -vE '\.(toml|png|jpg|svg|md)$' \
    | sort -u)

PRIMAL_ERRORS=0
SPRING_COUNT=0
PRIMAL_UNREGISTERED=()
SPRING_UNREGISTERED=()
TOTAL_GRAPH=0

while IFS= read -r method; do
    [[ -z "$method" ]] && continue
    ((TOTAL_GRAPH++)) || true
    if echo "$REGISTERED" | grep -qxF "$method"; then
        continue
    fi
    if [[ "$STRICT" == "false" ]] && echo "$method" | grep -qP "^($SPRING_DOMAINS)"; then
        SPRING_UNREGISTERED+=("$method")
        ((SPRING_COUNT++)) || true
    else
        PRIMAL_UNREGISTERED+=("$method")
        ((PRIMAL_ERRORS++)) || true
    fi
done <<< "$GRAPH_METHODS"

TOTAL_REGISTERED=$(echo "$REGISTERED" | wc -l)
MATCHED=$((TOTAL_GRAPH - PRIMAL_ERRORS - SPRING_COUNT))

if [[ $PRIMAL_ERRORS -eq 0 ]]; then
    echo "OK: $TOTAL_GRAPH graph refs checked, $MATCHED matched ($TOTAL_REGISTERED registered), $SPRING_COUNT spring-domain (advisory)"
    exit 0
fi

echo "DRIFT: $PRIMAL_ERRORS primal method(s) not in $REGISTRY (+ $SPRING_COUNT spring-domain advisory):"
echo ""
for m in "${PRIMAL_UNREGISTERED[@]}"; do
    FILES=$(rg -l "\"$m\"" "$GRAPH_DIR/" --glob '*.toml' 2>/dev/null \
        | sed 's|^|  |' | head -3)
    echo "  $m"
    echo "$FILES"
    echo ""
done

if [[ "${1:-}" == "--fix" || "${2:-}" == "--fix" ]]; then
    echo "--- Primal drift grouped by domain ---"
    echo ""
    for m in "${PRIMAL_UNREGISTERED[@]}"; do
        DOMAIN="${m%%.*}"
        echo "  [$DOMAIN]  $m"
    done
    if [[ $SPRING_COUNT -gt 0 ]]; then
        echo ""
        echo "--- Spring-domain (advisory, not errors) ---"
        echo ""
        for m in "${SPRING_UNREGISTERED[@]}"; do
            DOMAIN="${m%%.*}"
            echo "  [$DOMAIN]  $m"
        done
    fi
fi

exit 1
