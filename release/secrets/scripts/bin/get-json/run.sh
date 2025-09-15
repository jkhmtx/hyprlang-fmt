# shellcheck shell=bash

export RELEASE_SECRETS="${RELEASE_SECRETS}"
export SOPS_CONFIG="${SOPS_CONFIG}"

sops \
  --config "${SOPS_CONFIG}" \
  decrypt "${RELEASE_SECRETS}" \
  --output-type json
