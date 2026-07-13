#!/usr/bin/env bash
# Refresh the vendored SDL_GameControllerDB that sola-raylib bundles and
# auto-loads at window init (see raylib/src/core/gamepad_db.rs).
#
# Source: https://github.com/mdqinc/SDL_GameControllerDB (zlib license)
#
# Run from the repo root: `just update-gamecontrollerdb` or
# `./scripts/update_gamecontrollerdb.sh`
set -euo pipefail

repo="mdqinc/SDL_GameControllerDB"
branch="master"
dest_dir="raylib/src"

raw="https://raw.githubusercontent.com/${repo}/${branch}"

echo "Fetching gamecontrollerdb.txt from ${repo}@${branch} ..."
curl -sSL -o "${dest_dir}/gamecontrollerdb.txt" "${raw}/gamecontrollerdb.txt"
curl -sSL -o "${dest_dir}/gamecontrollerdb.LICENSE.txt" "${raw}/LICENSE"

lines="$(wc -l < "${dest_dir}/gamecontrollerdb.txt" | tr -d ' ')"

echo "Updated ${dest_dir}/gamecontrollerdb.txt (${lines} lines)"
echo "Review the diff and add a CHANGELOG entry noting the refresh."
