#!/usr/bin/env bash
set -uo pipefail
# cargo clean
cargo +nightly fmt
cargo check
cargo build
