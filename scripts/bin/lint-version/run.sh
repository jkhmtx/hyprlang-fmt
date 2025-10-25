# shellcheck shell=bash

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

version="$(cat version)"

sed \
	--in-place \
	--expression \
	's/^version = ".*"$/version = "'"${version}"'"/' \
	Cargo.toml

if test -v CI && ! git diff --exit-code -- :/Cargo.toml; then
	echo "Run root.lint-version and commit the result"

	exit 1
fi

sed \
	--in-place \
	--expression \
	's/version = ".*";$/version = "'"${version}"'";/' \
	nix/package.nix

if test -v CI && ! git diff --exit-code -- :/nix/package.nix; then
	echo "Run root.lint-version and commit the result"

	exit 1
elif ! git diff --exit-code --quiet -- :/nix/package.nix; then
	sed \
		--in-place \
		--expression \
		's/cargoHash = ".*";$/cargoHash = "";/' \
		nix/package.nix

	echo Resetting cargoHash
	set +e
	cargo_hash="$(nix build 2>&1 | grep got: | awk '{ print $2 }')"
	set -e

	sed \
		--in-place \
		--expression \
		's/cargoHash = ".*";$/cargoHash = "'"${cargo_hash}"'";/' \
		nix/package.nix

	nix build

	echo
	echo "Version strings updated, commit the result"

	exit 1
fi
