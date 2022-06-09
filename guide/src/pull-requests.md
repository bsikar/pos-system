# Pull Requests

Try to do one pull request per change.

### Backend changes

Make sure to run this [script](https://github.com/bsikar/pos-system/blob/main/backend/scripts/presubmit.sh) from the `/backend/scripts` directory before submitting a pull request.

```sh
#!/bin/bash

# Script to run presubmit checks. Devs can run this script locally before sending out PRs for review to ensure
# CI is passing for their PR.

set -e

FMT_COMMAND="cargo fmt"
echo "Running $FMT_COMMAND..."
EXIT_CODE=0
$FMT_COMMAND -- --check || EXIT_CODE=$?
if [[ $EXIT_CODE -ne 0 ]]; then
    echo 'Run `'$FMT_COMMAND'` to fix.'
    exit $EXIT_CODE
fi
echo "$FMT_COMMAND succeeded..."

echo "Running clippy..."
cargo clippy --tests -- -D warnings
echo "clippy succeeded..."

echo "Running cargo test..."
python test-backend.py
echo "Tests succeeded..."

echo "Congrats! All presubmits checks passed."
```

### Frontend changes

Make sure to run this [script](https://github.com/bsikar/pos-system/blob/main/frontend/scripts/presubmit.sh) from the `/frontend/scripts` directory before submitting a pull request.

```sh
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
```

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/bsikar/pos-system/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!
