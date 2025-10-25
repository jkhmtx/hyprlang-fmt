#!/usr/bin/env bash

set -euo pipefail

export GENERATED_NIX="${GENERATED_NIX}"

mkdir -p .direnv/shell/bin

nix build '#devShell' --out-link .direnv/shell/result

echo .direnv/shell/result/bin

mapfile -t derivations < <(sed -n 's/  \(.*\) =.*/\1/p' "${GENERATED_NIX}")

for derivation in "${derivations[@]}"; do
	printf "nix run '#%s'\n" "${derivation}" >.direnv/shell/bin/"${derivation}"
	chmod +x .direnv/shell/bin/"${derivation}"
done

echo ".direnv/shell/bin"
