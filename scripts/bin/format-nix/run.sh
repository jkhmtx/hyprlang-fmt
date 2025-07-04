# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

flags=(--quiet)

if test -v CI; then
	flags+=(--check)
fi

alejandra "${flags[@]}" .
