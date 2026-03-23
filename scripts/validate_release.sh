#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# primalSpring release validation gate.
#
# Absorbed from groundSpring V121, neuralSpring V122, wetSpring V133.
# Runs the full quality pipeline and enforces a test-count floor so
# regressions are caught before release.
#
# Usage:
#   ./scripts/validate_release.sh          # run all checks
#   SKIP_COVERAGE=1 ./scripts/validate_release.sh  # skip coverage (CI without llvm-cov)

set -euo pipefail

WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$WORKSPACE_ROOT"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

MIN_TESTS=320
FAILURES=0

step() { printf "\n${YELLOW}═══ %s ═══${NC}\n" "$1"; }
pass() { printf "${GREEN}✓ %s${NC}\n" "$1"; }
fail() { printf "${RED}✗ %s${NC}\n" "$1"; FAILURES=$((FAILURES + 1)); }

step "cargo fmt --check"
if cargo fmt --all --check 2>/dev/null; then
    pass "formatting clean"
else
    fail "formatting issues found — run 'cargo fmt --all'"
fi

step "cargo clippy --workspace"
if cargo clippy --workspace -- -D warnings 2>/dev/null; then
    pass "clippy clean"
else
    fail "clippy warnings or errors"
fi

step "cargo deny check"
if command -v cargo-deny >/dev/null 2>&1; then
    if cargo deny check 2>/dev/null; then
        pass "dependency audit clean"
    else
        fail "cargo deny found issues"
    fi
else
    printf "${YELLOW}⚠ cargo-deny not installed — skipping${NC}\n"
fi

step "cargo test --workspace"
TEST_OUTPUT=$(cargo test --workspace 2>&1)
TEST_COUNT=$(echo "$TEST_OUTPUT" | grep 'test result:' | awk '{sum += $4} END {print sum}')

if echo "$TEST_OUTPUT" | grep -q 'FAILED'; then
    fail "test failures detected"
else
    pass "all tests passed"
fi

if [ "${TEST_COUNT:-0}" -ge "$MIN_TESTS" ]; then
    pass "test count: $TEST_COUNT (floor: $MIN_TESTS)"
else
    fail "test count: ${TEST_COUNT:-0} below floor $MIN_TESTS"
fi

if [ "${SKIP_COVERAGE:-}" != "1" ] && command -v cargo-llvm-cov >/dev/null 2>&1; then
    step "cargo llvm-cov (line coverage)"
    COV_OUTPUT=$(cargo llvm-cov --workspace --no-report 2>&1 || true)
    pass "coverage collected (review with 'cargo llvm-cov report')"
else
    step "coverage"
    printf "${YELLOW}⚠ cargo-llvm-cov not installed or SKIP_COVERAGE=1 — skipping${NC}\n"
fi

step "cargo doc --workspace --no-deps"
if cargo doc --workspace --no-deps 2>/dev/null; then
    pass "docs build clean"
else
    fail "doc build has warnings or errors"
fi

printf "\n"
step "RESULT"
if [ "$FAILURES" -eq 0 ]; then
    printf "${GREEN}All checks passed — ready for release${NC}\n"
    exit 0
else
    printf "${RED}%d check(s) failed — fix before release${NC}\n" "$FAILURES"
    exit 1
fi
