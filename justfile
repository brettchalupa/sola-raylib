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

# Run an example built with raylib's CPU software renderer (rlsw) over SDL.
# Requires SDL2 dev headers installed (Fedora: `SDL2-devel`, Debian/Ubuntu:
# `libsdl2-dev`, macOS/Homebrew: `sdl2`). rlsw is not compatible with GLFW,
# so we also enable the `sdl` feature; see raylib#5664.
# Defaults to hello_raylib; override with e.g. `just example-sw logo`.
example-sw name="hello_raylib":
    cd examples && cargo run --features "sdl,software_render" --bin {{ name }}

# Run an example with the `noscreenshot` feature so raylib's built-in F12
# screenshot keybind is compiled out. Use to confirm rendering still works
# under the cflag override path that fixed issue #40.
# Defaults to hello_raylib; override with e.g. `just example-noscreenshot logo`.
example-noscreenshot name="hello_raylib":
    cd examples && cargo run --features noscreenshot --bin {{ name }}

# Wasm toolchain: rustup target + simple-http-server. Emscripten itself
# comes from `scripts/setup_emscripten.sh`.
setup-web:
    rustup target add wasm32-unknown-emscripten
    cargo install simple-http-server

# Build an example for the browser. Default: hello_raylib. Stages
# index.html (templated from examples/shell.html), the .js, and the
# .wasm into examples/target/web/ for `just serve-web`. Auto-sources
# emsdk_env.sh from ~/.local/share/emsdk if emcc is not on PATH.
build-web name="hello_raylib":
    bash -c 'set -e; \
        command -v emcc >/dev/null 2>&1 \
            || source ~/.local/share/emsdk/emsdk_env.sh >/dev/null 2>&1 \
            || { echo "[build-web] emcc not on PATH and no emsdk at ~/.local/share/emsdk/. Run scripts/setup_emscripten.sh first." >&2; exit 1; }; \
        cd examples && cargo build --release --bin {{ name }} --target wasm32-unknown-emscripten'
    mkdir -p examples/target/web
    rm -f examples/target/web/*
    sed 's/__SOLA_BIN__/{{ name }}/g' examples/shell.html > examples/target/web/index.html
    cp examples/target/wasm32-unknown-emscripten/release/{{ name }}.wasm examples/target/web/
    cp examples/target/wasm32-unknown-emscripten/release/{{ name }}.js examples/target/web/
    @echo "[build-web] examples/target/web/ ready. Run 'just serve-web' to open."

# Serve examples/target/web/ on http://localhost:3535. Run after `just build-web NAME`.
serve-web:
    simple-http-server --index --nocache -p ${PORT:=3535} examples/target/web

# Serve the mdbook with hot reload (http://localhost:3030).
serve-book:
    cd book && mdbook serve --port ${PORT:=3030}

# Render the mdbook to book/book/ (git-ignored).
build-book:
    cd book && mdbook build

# Initializes git submodules
setup:
    git submodule update --init
    cargo install mdbook
    @just setup-web

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
    just example pixel_color
    just example raymarch
    just example rgui
    just example shapes_new
    just example texture
    just example yaw_pitch_roll
    just example drop
    just example-noscreenshot
    just examples-sw
    just features-build

# Smoke-test the CPU software renderer backend (raylib 6.0 `rlsw`) over SDL.
# Requires SDL2 dev headers. See `example-sw` comment above for details.
examples-sw:
    just example-sw hello_raylib

# Build sola-raylib against a handful of Cargo feature combos to catch
# build-time regressions (CMake knobs renamed upstream, cfg-gated code
# rotting, conflicting feature interactions). Build-only, no windows.
# Each unique feature signature triggers a full raylib C rebuild, so this
# is slow; run it before tagging a release. Skips features that need extra
# system packages (`sdl`, `software_render`, `wayland`); those are covered
# by `examples-sw` and manual checks.
features-build:
    cargo build -p sola-raylib
    cargo build -p sola-raylib --features noscreenshot
    cargo build -p sola-raylib --features custom_frame_control
    cargo build -p sola-raylib --features with_serde
    cargo build -p sola-raylib --features convert_mint
    cargo build -p sola-raylib --features "noscreenshot,with_serde,convert_mint"
