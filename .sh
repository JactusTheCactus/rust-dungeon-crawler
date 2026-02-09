#!/usr/bin/env bash
set -euo pipefail
log=
clean=
for i in "$@"; do
	case "$i" in
		clean) clean=0 ;;
		log) log=.log ;;
	esac
done
main() {
	if [[ -n $clean ]]
		then cargo clean
	fi
	cargo +nightly fmt
	cargo check
	cargo doc --no-deps
	cargo build
}
if [[ -z $log ]]
	then main
	else main &> $log
fi
