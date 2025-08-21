# shellcheck shell=bash

export CI=true

if test "${FIX:-}" = true; then
	unset CI
fi

log() {
	>&2 echo "${PREFIX:-CHECK}: " "${@}"
}

log Documenting...
root.docs
log "Documenting done"

log Linting...
root.lint
log "Linting done"

log Formatting...
root.format
log "Formatting done"
