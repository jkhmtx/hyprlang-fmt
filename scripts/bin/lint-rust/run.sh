# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

clippy_flags=()
check_flags=(--workspace --all-targets)

if ! test -v CI; then
	clippy_flags+=(--fix --allow-dirty --quiet)
	check_flags+=(--quiet)
fi

cargo clippy "${clippy_flags[@]}"
cargo check "${check_flags[@]}"
