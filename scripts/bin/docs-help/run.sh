# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

cargo run --quiet -- --help >./help.txt

if test -v CI && ! git diff --exit-code -- :/help.txt; then
	echo "Run root.docs-help and commit the result"

	exit 1
fi
