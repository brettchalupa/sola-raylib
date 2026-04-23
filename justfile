# sola-raylib dev tasks
# Run `just` to see all recipes, `just <name>` to run one.

default:
    @just --list

# Run all checks that should be green before committing/pushing.
ok: build clippy test samples
    @echo "All checks passed."

# Build every crate in the workspace.
build:
    cargo build --workspace

# Lint every crate and target. Warnings are allowed for now (lots of pre-existing upstream noise).
clippy:
    cargo clippy --workspace --all-targets

# Run all workspace tests (unit + doc tests).
# Tests that require a display live in the raylib-test crate, which is excluded from the workspace.
test:
    cargo test --workspace

# Format all code in place.
fmt:
    cargo fmt --all

# TODO: re-enable once the workspace is cleanly formatted end-to-end.
# fmt-check:
#     cargo fmt --all -- --check

# Build the sample binaries crate.
samples:
    cd samples && cargo build

# Run a specific sample by name, e.g. `just sample drop`.
sample name:
    cd samples && cargo run --bin {{name}}
