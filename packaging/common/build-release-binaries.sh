#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")/../.." && pwd)"
CLIENT_BUILD_DIR="${HDHR_CLIENT_BUILD_DIR:-$ROOT_DIR/build/client-release}"

cmake -S "$ROOT_DIR/client" -B "$CLIENT_BUILD_DIR" -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build "$CLIENT_BUILD_DIR"
cargo build --manifest-path "$ROOT_DIR/backend/Cargo.toml" --release --quiet