#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$ROOT/output"
shopt -s nullglob
imgs=( "$ROOT"/build/tmp/deploy/images/genericx86-64/*.wic )
if [ -z "${imgs[0]-}" ]; then echo "No WIC image produced"; exit 1; fi
cp -f "${imgs[0]}" "$ROOT/output/LunnaOS-Polaris-x86_64.wic"
echo "Bootable disk image exported."
