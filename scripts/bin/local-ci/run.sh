# shellcheck shell=bash

export CI=true

if test "${FIX:-}" = true; then
	unset CI
fi

log() {
	>&2 echo "${PREFIX:-LOCAL-CI}: " "${@}"
}

lints=(
	lint-github-actions
	lint-rust
	lint-shell
)

log Linting...
for check in "${lints[@]}"; do
	"${check}"
done
log "Linting done"

formats=(
	format-nix
	format-rust
	format-shell
	format-yaml
)

log Formatting...
for check in "${formats[@]}"; do
	"${check}"
done
log "Formatting done"
