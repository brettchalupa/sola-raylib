# sola-raylib dev tasks
# Run `just` to see all recipes, `just <name>` to run one.

default:
    @just --list

# Run all checks that should be green before committing/pushing.
ok: fmt-check build clippy test examples
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

fmt-check:
    cargo fmt --all -- --check

# Build the examples binaries crate.
examples:
    cd examples && cargo build

# Run a specific example by name, e.g. `just sample drop`.
example name:
    cd examples && cargo run --bin {{ name }}
