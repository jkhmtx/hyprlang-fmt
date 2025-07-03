#!/usr/bin/env bash

set -euo pipefail

function installed() {
  for arg in "${@}"; do
    command -v "${arg}" >/dev/null || return 1
  done
}

function with_stderr() {
  >&2 "${@}"
}

function log() {
  prefix="${1}"
  shift
  with_stderr echo "${prefix}" "${@}"
}

function empty() {
  with_stderr echo
}

function info() {
  log "INFO: " "${@}"
}

function warn() {
  log "WARN: " "${@}"
}

function error() {
  log "!ERR: " "${@}"
}

function ask_y() {
  empty

  for line in "${@}"; do
    with_stderr log "!ASK: " "${line}"
  done

  empty
  with_stderr log "!ASK: " "Enter [y] to continue"
  empty

  local choice
  read -r choice

  echo "${choice}"
}

empty
if ! installed nix; then
  error "Please install nix."
  error "https://nixos.org/download/"

  exit 1
fi
info "'nix' installed"

if ! installed direnv; then
  empty
  choice="$(
    ask_y \
      "This script will install 'direnv' to your nix profile." \
      "'direnv' is not required, but greatly simplifies using the repository."
  )"

  if test "${choice}" = y; then
    nix profile install nixpkgs#direnv
  else
    warn "Please install 'direnv'"
    warn "https://direnv.net/"

    exit 1
  fi
  empty
  empty
fi

info "'direnv' installed"

empty
info "Make sure you follow the 'hook direnv into your shell' instructions, then use 'direnv allow'"
info "https://direnv.net/docs/hook.html"
