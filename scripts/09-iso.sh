#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEPLOY="$ROOT/build/tmp/deploy/images/genericx86-64"
mkdir -p "$ROOT/output"
shopt -s nullglob
isos=( "$DEPLOY"/*.iso )
if [ -z "${isos[0]-}" ]; then
  echo "No ISO image produced"
  exit 1
fi
cp -f "${isos[0]}" "$ROOT/output/LunnaOS-Polaris-x86_64.iso"
echo "Bootable ISO exported: $ROOT/output/LunnaOS-Polaris-x86_64.iso"
