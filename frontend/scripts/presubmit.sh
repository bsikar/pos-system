#!/bin/bash

# Script to run presubmit checks. Devs can run this script locally before sending out PRs for review to ensure
# CI is passing for their PR.

set -e

FMT_COMMAND="npm run fmt"
echo "Running $FMT_COMMAND..."
EXIT_CODE=0
$FMT_COMMAND:check || EXIT_CODE=$?
if [[ $EXIT_CODE -ne 0 ]]; then
    echo 'Run `'$FMT_COMMAND'` to fix.'
    exit $EXIT_CODE
fi
echo "$FMT_COMMAND succeeded..."

LINT_COMMAND="npm run lint"
echo "Running $LINT_COMMAND..."
EXIT_CODE=0
$LINT_COMMAND || EXIT_CODE=$?
if [[ $EXIT_CODE -ne 0 ]]; then
    echo 'Run `'$LINT_COMMAND':fix` to fix.'
    exit $EXIT_CODE
fi
echo "$LINT_COMMAND succeeded..."

echo "Congrats! All presubmits checks passed."
