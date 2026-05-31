#!/usr/bin/env bash
# Compile-check every `*_solution` crate, then run their tests.
# Two cargo invocations total (not one-per-crate), so the workspace cache
# stays warm.

set -euo pipefail

cd "$(dirname "$0")/.."

# Collect every workspace package whose name ends in `_solution`.
mapfile -t PKGS < <(
  cargo metadata --no-deps --format-version 1 \
    | python3 -c '
import json, sys
for p in json.load(sys.stdin)["packages"]:
    if p["name"].endswith("_solution"):
        print(p["name"])
'
)

if [[ ${#PKGS[@]} -eq 0 ]]; then
  echo "no _solution packages found" >&2
  exit 1
fi

# Build the `-p name -p name ...` args for cargo.
P_ARGS=()
for name in "${PKGS[@]}"; do
  P_ARGS+=(-p "$name")
done

echo "==> cargo check on ${#PKGS[@]} solutions"
cargo check "${P_ARGS[@]}" --quiet

echo "==> cargo test on ${#PKGS[@]} solutions"
cargo test "${P_ARGS[@]}" --quiet

echo
echo "All ${#PKGS[@]} solutions check + test green."
