# sola-raylib dev tasks
# Run `just` to see all recipes, `just <name>` to run one.

default:
    @just --list

# Run all checks that should be green before committing/pushing.
ok: fmt-check build clippy test build-examples
    @echo "All checks passed."

# Build every crate in the workspace.
build:
    cargo build --workspace --all-targets

# Lint every crate and target. Warnings are allowed for now (lots of pre-existing upstream noise).
clippy:
    cargo clippy --workspace --all-targets

# Run all workspace tests (unit + doc tests). Runtime tests live in
# `just examples` — run those to actually exercise raylib in a window.
test:
    cargo test --workspace

# Format all code in place.
fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

# Build the examples binaries crate.
build-examples:
    cd examples && cargo build --all-targets

# Run a specific example by name, e.g. `just example drop`.
example name:
    cd examples && cargo run --bin {{ name }}

# Run an example built with raylib's CPU software renderer (rlsw).
# Defaults to hello_raylib; override with e.g. `just example-sw logo`.
example-sw name="hello_raylib":
    cd examples && cargo run --features software_render --bin {{ name }}

# Initializes git submodules
setup:
    git submodule update --init

# Run a handful of examples to quickly check things are working
examples:
    just example 3d_camera_first_person
    just example animation_blending
    just example arkanoid
    just example asteroids
    just example borderless_fullscreen
    just example camera_2d
    just example extensions
    just example hello_raylib
    just example input
    just example logo
    just example font
    just example model_shader
    just example raymarch
    just example rgui
    just example shapes_new
    just example texture
    just example yaw_pitch_roll
    just example drop
    just examples-sw

# Smoke-test the CPU software renderer backend (raylib 6.0 `rlsw`).
examples-sw:
    just example-sw hello_raylib
