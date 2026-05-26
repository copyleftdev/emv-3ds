#!/usr/bin/env bash
# Wire git to use .githooks/ for this repo.
set -euo pipefail

ROOT="$(git -C "$(dirname "$0")/.." rev-parse --show-toplevel)"

git -C "$ROOT" config core.hooksPath .githooks
chmod +x "$ROOT"/.githooks/*

echo "Git hooks installed from $ROOT/.githooks/"
echo "  pre-commit : fmt + clippy + test"
echo "  pre-push   : cargo mutants (skip with SKIP_MUTANTS=1)"
