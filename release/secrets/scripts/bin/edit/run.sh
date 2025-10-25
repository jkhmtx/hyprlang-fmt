# shellcheck shell=bash

export SOPS_CONFIG="${SOPS_CONFIG}"
export RELEASE_SECRETS="${RELEASE_SECRETS}"

sops \
	--config "${SOPS_CONFIG}" \
	edit "${RELEASE_SECRETS}"
