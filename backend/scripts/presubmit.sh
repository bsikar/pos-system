#!/bin/bash

# Script to run presubmit checks. Devs can run this script locally before sending out PRs for review to ensure
# CI is passing for their PR.

set -e

FMT_COMMAND="cargo fmt"
echo "Running $FMT_COMMAND..."
set -e
EXIT_CODE=0
$FMT_COMMAND -- --check || EXIT_CODE=$?
if [[ $EXIT_CODE -ne 0 ]]; then
    echo 'Run `'$FMT_COMMAND'` to fix.'
    exit $EXIT_CODE
fi

echo "Cargo fmt succeeded..."

echo "Running clippy..."
cargo clippy --tests -- -D warnings
echo "clippy succeeded..."

echo "Running cargo test..."
python test-backend.py
echo "Tests succeeded..."

echo "Congrats! All presubmits checks passed."
