#!/usr/bin/env bash
set -euo pipefail
for i in "$@"; do
	case "$i" in
		clean) cargo clean ;;
	esac
done
cargo +nightly fmt
cargo check
cargo doc \
	--no-deps #\
	#--document-private-items
cargo build
