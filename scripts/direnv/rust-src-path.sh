#!/usr/bin/env bash

set -euo pipefail

rustc_path="$(realpath "$(nix path-info '#devShell')"/bin/rustc)"
echo "${rustc_path/\/bin\/rustc/}/lib/rustlib/src/rust/library"
