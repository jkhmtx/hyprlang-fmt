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

# shellcheck disable=SC2002
cat ./testbed/hypr/hyprland.conf | hyprlang-fmt >formatted

diff -y --suppress-common-lines "${HYPRLAND_CONF}" formatted

log Success
