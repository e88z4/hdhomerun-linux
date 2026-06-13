#!/usr/bin/env sh
set -eu

ROOT_DIR="$(CDPATH='' cd -- "$(dirname -- "$0")/../.." && pwd)"
CLIENT_BUILD_DIR="${HDHR_CLIENT_BUILD_DIR:-$ROOT_DIR/build/client-release}"

rm -rf "$CLIENT_BUILD_DIR"
cmake -S "$ROOT_DIR/client" -B "$CLIENT_BUILD_DIR" -G Ninja -DCMAKE_BUILD_TYPE=Release
cmake --build "$CLIENT_BUILD_DIR"
cargo build --manifest-path "$ROOT_DIR/backend/Cargo.toml" --release --quiet
cargo build --manifest-path "$ROOT_DIR/backend/Cargo.toml" --release --bin hdhr-headless --quiet