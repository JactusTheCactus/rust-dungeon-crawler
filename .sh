#!/usr/bin/env bash
set -uo pipefail
flag() {
	for f in "$@"
		do [[ -e ".flags/$f" ]] || return 1
	done
}
# cargo clean
cargo +nightly fmt
cargo check
cargo build
