#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# DEPRECATED (Wave 54): Use Rust: primalspring registry --check source
#
# PG-65: Method string drift detector
#
# Extracts all dotted method strings from Rust source and checks them against
# config/capability_registry.toml. Catches silent drift where code uses a
# method name that no primal actually serves.
#
# Usage: tools/check_method_strings.sh [--fix]
#   --fix: print unregistered methods for adding to the registry

set -euo pipefail
cd "$(git rev-parse --show-toplevel)"

REGISTRY="config/capability_registry.toml"

if [[ ! -f "$REGISTRY" ]]; then
    echo "FAIL: $REGISTRY not found"
    exit 1
fi

# Extract all registered methods from TOML (lines matching `"method.name"`)
REGISTERED=$(grep -oP '^\s+"[a-z][a-z0-9_.]+[a-z0-9]+"' "$REGISTRY" | tr -d ' "' | sort -u)

# Extract all dotted strings from Rust source (lib + experiments + bins)
USED=$(rg -o '"[a-z]+\.[a-z_.]+[a-z]+"' ecoPrimal/src/ experiments/ --no-filename 2>/dev/null \
    | tr -d '"' | sort -u)

ERRORS=0
UNREGISTERED=()

while IFS= read -r method; do
    if ! echo "$REGISTERED" | grep -qxF "$method"; then
        UNREGISTERED+=("$method")
        ((ERRORS++)) || true
    fi
done <<< "$USED"

if [[ $ERRORS -eq 0 ]]; then
    TOTAL_REGISTERED=$(echo "$REGISTERED" | wc -l)
    TOTAL_USED=$(echo "$USED" | wc -l)
    echo "OK: $TOTAL_USED method strings used, all in registry ($TOTAL_REGISTERED registered)"
    exit 0
fi

echo "DRIFT: $ERRORS method string(s) not in $REGISTRY:"
echo ""
for m in "${UNREGISTERED[@]}"; do
    echo "  $m"
done
echo ""

if [[ "${1:-}" == "--fix" ]]; then
    echo "Add these to $REGISTRY under the appropriate [domain] section."
fi

exit 1
