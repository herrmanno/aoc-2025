#!/bin/sh

set -euo pipefail

DAYS=$@

if [ ! -f ".session" ]; then
	printf ".session file not found"
	exit 1
fi
SESSION="$(cat .session)"

for day in ${DAYS[@]}; do
	wget \
		--header "Cookie: session=$SESSION" \
		--header "User-Agent: o.herrmann92@gmail.com" \
		"https://adventofcode.com/2025/day/$day/input" \
		-O "input/$(printf "%0.2d" $day).txt"
done
