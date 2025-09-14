#!/usr/bin/env bash

set -euo pipefail

nix build '#devShell' --out-link shell_result

echo "shell_result"
