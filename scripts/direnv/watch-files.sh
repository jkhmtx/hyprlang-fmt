#!/usr/bin/env bash

set -euo pipefail

echo Cargo.lock
find . -type f -name "run.sh"
find . -type f -name "*.nix"
find . -type f -path "./scripts/direnv/*"
