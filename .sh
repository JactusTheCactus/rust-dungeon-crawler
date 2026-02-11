#!/usr/bin/env bash
set -euo pipefail
log=
clean=
for i in "$@"
	do case "$i" in
		clean) clean=0 ;;
		log|*.log)
			if [[ $i == log ]]
				then log=.log
				else log=$i
			fi
		;;
	esac
done
main() {
	if [[ -n $clean ]]
		then cargo clean
	fi
	cargo +nightly fmt
	cargo check
	cargo build
}
rm -rf logs
mkdir -p logs
if [[ -n $log ]]
	then main &> "logs/$log"
	else main
fi
while read -r pug; do
	pug3 "$pug"
	perl -pgi -e '
		s|\n|&#xd;|g;
		s|\t|&#x9;|g;
	' "${pug%.pug}.html"
done < <(find . -name \*.pug)
while read -r scss
	do sass "$scss" "${scss%.scss}.css" \
		--no-source-map \
		--style=compressed
done < <(find . -name \*.scss)
find . -empty ! -path './target/*' -delete
