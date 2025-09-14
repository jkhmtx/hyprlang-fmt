# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1
mkdir -p debug
for file in testbed/hypr/*; do
	name="$(basename "${file}")"
	cargo run --quiet <./testbed/hypr/"${name}" >debug/"${name}"

	echo updated "'debug/${name}'"
done
