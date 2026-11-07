#!/bin/bash
set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
cd "$SCRIPT_DIR"

if [ "$1" = "release" ]; then
  FLAGS=""
else
  FLAGS="--dev"
fi

wasm-pack build --target web --out-dir ./extension/pkg/ $FLAGS
