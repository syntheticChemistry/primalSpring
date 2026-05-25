#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# W7-06: Inverse drift detector — "registered but never exercised"
#
# Complement to check_method_strings.sh which catches code using unregistered
# methods. This script catches the opposite: methods registered in the
# capability_registry.toml that are NEVER referenced in any scenario, test,
# or deploy graph. This is the gap class that let NestGate content.* methods
# pass the structural gate while being semantically untested.
#
# Usage: tools/check_method_coverage.sh [--domains] [--warn-only]
#   --domains  : group output by capability domain
#   --warn-only: exit 0 even if gaps found (advisory mode)

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

REGISTRY="config/capability_registry.toml"
WARN_ONLY=false
BY_DOMAIN=false

for arg in "$@"; do
    case "$arg" in
        --warn-only) WARN_ONLY=true ;;
        --domains)   BY_DOMAIN=true ;;
    esac
done

if [[ ! -f "$REGISTRY" ]]; then
    echo "FAIL: $REGISTRY not found"
    exit 1
fi

# Extract methods, excluding [test_fixtures] and [false_positives] sections
REGISTERED=$(awk '
    /^\[test_fixtures\]/ { skip=1; next }
    /^\[false_positives\]/ { skip=1; next }
    /^\[/ { skip=0 }
    skip { next }
    match($0, /^\s+"([a-z][a-z0-9_.]+[a-z0-9])"/, m) { print m[1] }
' "$REGISTRY" | sort -u)
TOTAL_REGISTERED=$(echo "$REGISTERED" | wc -l)

SEARCH_DIRS=(
    "ecoPrimal/src/validation/scenarios/"
    "ecoPrimal/tests/"
    "graphs/"
)

EXISTING_DIRS=()
for d in "${SEARCH_DIRS[@]}"; do
    [[ -d "$d" ]] && EXISTING_DIRS+=("$d")
done

if [[ ${#EXISTING_DIRS[@]} -eq 0 ]]; then
    echo "FAIL: no scenario/test/graph directories found"
    exit 1
fi

EXERCISED=$(rg -o '"[a-z][a-z0-9_.]+[a-z0-9]+"' "${EXISTING_DIRS[@]}" --no-filename 2>/dev/null \
    | tr -d '"' | sort -u)

UNCOVERED=()
UNCOVERED_DOMAINS=()

while IFS= read -r method; do
    if ! echo "$EXERCISED" | grep -qxF "$method"; then
        UNCOVERED+=("$method")
        domain="${method%%.*}"
        UNCOVERED_DOMAINS+=("$domain")
    fi
done <<< "$REGISTERED"

TOTAL_UNCOVERED=${#UNCOVERED[@]}
TOTAL_COVERED=$((TOTAL_REGISTERED - TOTAL_UNCOVERED))

if [[ $TOTAL_UNCOVERED -eq 0 ]]; then
    echo "OK: $TOTAL_REGISTERED/$TOTAL_REGISTERED registered methods exercised in scenarios/tests/graphs"
    exit 0
fi

echo "COVERAGE GAP: $TOTAL_UNCOVERED/$TOTAL_REGISTERED registered methods never exercised"
echo ""

if $BY_DOMAIN; then
    declare -A DOMAIN_METHODS
    for i in "${!UNCOVERED[@]}"; do
        d="${UNCOVERED_DOMAINS[$i]}"
        m="${UNCOVERED[$i]}"
        DOMAIN_METHODS[$d]="${DOMAIN_METHODS[$d]:-}  $m"$'\n'
    done
    for domain in $(echo "${!DOMAIN_METHODS[@]}" | tr ' ' '\n' | sort -u); do
        echo "[$domain]"
        echo "${DOMAIN_METHODS[$domain]}"
    done
else
    for m in "${UNCOVERED[@]}"; do
        echo "  $m"
    done
    echo ""
fi

echo "Coverage: $TOTAL_COVERED/$TOTAL_REGISTERED ($((TOTAL_COVERED * 100 / TOTAL_REGISTERED))%)"
echo ""
echo "Searched: ${EXISTING_DIRS[*]}"
echo "These methods are in $REGISTRY but never appear in any scenario, test, or graph."
echo "Add contract tests or graph steps that exercise them."

if $WARN_ONLY; then
    exit 0
fi
exit 1
