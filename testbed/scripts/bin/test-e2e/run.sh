# shellcheck shell=bash

cleanup() {
	rm formatted >/dev/null || true
}

log() {
	>&2 echo --- "${@}"
}

log Test: E2E

trap cleanup EXIT

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

export HYPRLAND_CONF="${HYPRLAND_CONF}"
export HYPRIDLE_CONF="${HYPRIDLE_CONF}"

nix run >formatted <./testbed/hypr/hyprland.conf
diff -y --suppress-common-lines "${HYPRLAND_CONF}" formatted

nix run >formatted <./testbed/hypr/hypridle.conf
diff -y --suppress-common-lines "${HYPRIDLE_CONF}" formatted

log Success
