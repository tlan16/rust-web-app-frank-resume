#!/usr/bin/env bash
set -euo pipefail

cd "$(dirname "$0")/.." || exit 1

trunk serve --release --port 8989
