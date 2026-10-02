#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
mkdir -p "$ROOT/build" "$ROOT/output"
command -v podman >/dev/null || { echo "podman is required"; exit 1; }
echo "LunnaOS build environment prepared."
