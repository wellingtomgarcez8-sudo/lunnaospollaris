#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
required=(kas/lunnaos.yml meta-lunnaos/conf/distro/lunnaos.conf meta-lunnaos/recipes-core/images/lunnaos-image.bb Containerfile)
for f in "${required[@]}"; do test -f "$ROOT/$f" || { echo "Missing $f"; exit 1; }; done
find "$ROOT/scripts" -type f -name '*.sh' -print0 | xargs -0 -n1 bash -n
echo "Repository validation passed."
