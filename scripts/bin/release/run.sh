# shellcheck shell=bash

git fetch --tags --force --all

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

version="$(cat version)"

if test "$(git tag --list 'v*' | sort -V | tail -n1)" = "${version}"; then
  echo "No change detected in 'version'. Update the 'version' and try again."

  exit 1
fi

root.lint-version

main="$(git rev-parse origin/main)"

for tag in "v${version}" latest; do
  git tag "${tag}" --sign --force --message "v${version}" "${main}"
  git push origin "${tag}" --force
done
