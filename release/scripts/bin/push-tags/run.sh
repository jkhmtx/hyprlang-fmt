# shellcheck shell=bash

function cleanup() {
  git config unset --local tag.gpgSign
  git config unset --local gpg.format
  git config unset --local user.signingKey
}

trap cleanup EXIT

secrets_json="$(release.secrets.get-json)"

function get_releaser_secret() {
  local path="${1}"
  local releaser
  releaser="$(git config get user.email)"
  jq \
    --raw-output \
    --arg releaser "${releaser}" \
    --arg path "${path}" \
    'getpath([
      "releasers",
      $releaser,
      $path
    ])' \
    <<<"${secrets_json}"
}

public_key="$(get_releaser_secret gpg-signing-public-key)"
secret_key="$(get_releaser_secret gpg-signing-secret-key)"

for key in "${public_key}" "${secret_key}"; do
  gpg --import <<<"${key}" >/dev/null 2>&1 || true
done

key_id="$(gpg \
  --show-keys \
  --keyid-format=long \
  --with-colons \
  <<<"${public_key}" |
  head -n1 |
  cut -d':' -f5)"

git config --local tag.gpgSign true
git config --local gpg.format openpgp
git config --local user.signingKey "${key_id}"

git fetch --tags --force --all

root="$(git rev-parse --show-toplevel)"

cd "${root}" || exit 1

version="$(cat version)"

if test "$(git tag --list 'v*' | sort -V | tail -n1)" = "v${version}"; then
  {
    echo
    echo "No change detected in 'version'. Update the 'version' and try again."
  } >&2

  exit 1
fi

root.lint-version

main="$(git rev-parse origin/main)"

for tag in "v${version}" latest; do
  git tag "${tag}" \
    --sign \
    --force \
    --message "v${version}" \
    "${main}"

  git verify-tag "${tag}"

  git push origin "${tag}" --force
done
