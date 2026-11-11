#!/usr/bin/env bash
# Refresh the vendored raygui header that sola-raylib binds and compiles.
#
# Source: https://github.com/raysan5/raygui
# src/raygui.h is vendored, so that's all that's fetched
#
# Run from the repo root: `just update-raygui` or
# `./scripts/update_raygui.sh`
set -euo pipefail

repo="raysan5/raygui"
ref="${1:-master}"
dest="raylib-sys/binding/raygui.h"

raw="https://raw.githubusercontent.com/${repo}/${ref}/src/raygui.h"

echo "Fetching raygui.h from ${repo}@${ref} ..."
curl -fsSL -o "${dest}" "${raw}"
echo "Updated ${dest}"
