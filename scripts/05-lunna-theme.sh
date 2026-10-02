#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
test -f "$ROOT/meta-lunnaos/recipes-desktop/lunnaos-defaults/files/lunnaos.css"
test -f "$ROOT/meta-lunnaos/recipes-desktop/lunnaos-shell/files/extension.js"
echo "LunnaOS desktop assets validated."
