#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
test -s "$ROOT/config/kernel.config"
echo "Kernel baseline validated."
