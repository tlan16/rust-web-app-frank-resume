#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
DOCS_DIR="$REPO_ROOT/docs"

command -v trunk >/dev/null 2>&1 || {
  echo "trunk is required. Install it with: cargo install trunk" >&2
  exit 1
}

rm -rf "$DOCS_DIR"
mkdir -p "$DOCS_DIR"

trunk build \
  --release \
  --dist "$DOCS_DIR" \
  --public-url ./

touch "$DOCS_DIR/.nojekyll"
