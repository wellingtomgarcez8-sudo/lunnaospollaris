#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
required=(
  kas/lunnaos.yml
  meta-lunnaos/conf/distro/lunnaos.conf
  meta-lunnaos/recipes-core/images/lunnaos-image.bb
  meta-lunnaos/recipes-core/packagegroups/packagegroup-lunnaos.bb
  meta-lunnaos/recipes-desktop/packagegroups/packagegroup-lunnaos-shell.bb
  meta-lunnaos/recipes-desktop/lunnaos-platform/lunnaos-platform.bb
  meta-lunnaos/recipes-desktop/lunnaos-shell/files/metadata.json
  meta-lunnaos/recipes-desktop/lunnaos-shell/files/extension.js
  Containerfile
)
for f in "${required[@]}"; do
  test -f "$ROOT/$f" || { echo "Missing $f"; exit 1; }
done
grep -q 'PACKAGE_CLASSES = "package_rpm"' "$ROOT/kas/lunnaos.yml"
grep -q 'package-management' "$ROOT/kas/lunnaos.yml"
grep -q 'flatpak' "$ROOT/meta-lunnaos/recipes-core/packagegroups/packagegroup-lunnaos.bb"
find "$ROOT/scripts" -type f -name '*.sh' -print0 | xargs -0 -n1 bash -n
echo "Repository validation passed."
