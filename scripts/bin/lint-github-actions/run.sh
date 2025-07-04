# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

actionlint
