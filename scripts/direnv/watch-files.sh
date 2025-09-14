#!/usr/bin/env bash

set -euo pipefail

echo Cargo.lock
find . -type f -path "*/scripts/bin/*"
find . -type f -path "./scripts/direnv/*"
