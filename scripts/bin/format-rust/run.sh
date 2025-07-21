# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

flags=()

if test -v CI; then
	flags+=(--check)
fi

git ls-files --exclude-standard --others -z '*.rs' && git ls-files --exclude-standard -z '*.rs' >files.lst

mapfile -d '' -t files <files.lst

rm files.lst

rustfmt "${flags[@]}" "${files[@]}"
