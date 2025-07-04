# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

flags=(--no-error-on-unmatched-pattern)

if test -v CI; then
	flags+=(--check)
else
	flags+=(--write --log-level=warn)
fi

prettier "${flags[@]}" '*.yml' '*.yaml'
