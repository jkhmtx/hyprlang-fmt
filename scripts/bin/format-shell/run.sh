# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

flags=()

if test -v CI; then
	flags+=(--diff)
else
	flags+=(--write)
fi

shfmt "${flags[@]}" .
