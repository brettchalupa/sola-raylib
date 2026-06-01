# sola-raylib-sys

Raw FFI bindings for [raylib](https://www.raylib.com/) and
[raygui](https://github.com/raysan5/raygui), consumed by the safe
[`sola-raylib`](../raylib) wrapper. Not intended to be used directly.

`sola-raylib-sys N.x` tracks raylib's major version (5.x → raylib 5.5, 6.x →
raylib 6.0) and is pinned to the matching `sola-raylib N.x`.

## How bindings are built

Bindings are **generated at build time**. There are no checked-in
`bindings_*.rs` files — every `cargo build` of this crate runs
[`bindgen`](https://github.com/rust-lang/rust-bindgen) against
`binding/binding.h`:

1. `build.rs` configures a `bindgen::Builder` with `-std=c99`, rustified enums,
   and `-I./raylib/src` so clang resolves the submoduled raylib headers (not a
   system `/usr/include/raylib.h`).
2. `binding/binding.h` pulls in raylib plus the raygui wrapper shim
   (`binding/rgui_wrapper.h`).
3. The generated `bindings.rs` is written to `$OUT_DIR` and pulled into
   `src/lib.rs` via `include!(concat!(env!("OUT_DIR"), "/bindings.rs"))`.
4. `cc` compiles the raygui wrapper and links it with libraylib.

To inspect the generated bindings:

```
find target -name bindings.rs -path '*sola-raylib-sys*'
```

## Regenerating after a header change

`build.rs` declares `rerun-if-changed` for `binding/binding.h` and for itself,
so bindings regenerate automatically when those inputs change. If cargo seems to
be using stale bindings after you:

- bumped the `raylib` submodule to a new version,
- replaced `binding/raygui.h` with a new raygui release, or
- edited `binding/binding.h` / the wrapper shims,

force a rebuild:

```
cargo clean -p sola-raylib-sys
cargo build -p sola-raylib-sys
```

See [DEVELOPING.md](../DEVELOPING.md) for the full "bumping raylib" checklist.

## Feature flags

The full feature reference (sys + safe crate) lives in the top-level
[README](../README.md#cargo-features). The flags exposed by `sola-raylib-sys`
itself are the same names; the safe `sola-raylib` crate just forwards each one
through. Quick map:

| Feature                                                     | Purpose                                                                                                                               |
| ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- |
| `bindgen` _(default)_                                       | Generate bindings at build time. Turn off if you need to hand-roll `bindings.rs` for a platform bindgen can't target.                 |
| `nobuild`                                                   | Skip building and linking raylib entirely. For docs.rs and headless setups; you are responsible for linking.                          |
| `wayland`                                                   | Build raylib with native Wayland support on Linux. Requires `glfw-devel`.                                                             |
| `sdl`                                                       | Build raylib with the SDL platform backend. raylib prefers SDL3 and falls back to SDL2; the build script links whichever it resolved. |
| `opengl_21` / `opengl_33` / `opengl_es_20` / `opengl_es_30` | Select the GL backend raylib compiles against.                                                                                        |
| `noscreenshot`                                              | Disable raylib's built-in F12 screenshot keybind.                                                                                     |
| `custom_frame_control`                                      | Enable raylib's `SUPPORT_CUSTOM_FRAME_CONTROL` build flag.                                                                            |
| `software_render`, `platform_memory`, `platform_web_rgfw`   | Experimental raylib 6.0 backends; see the top-level README for caveats.                                                               |

## Layout

```
binding/           raygui wrapper shim, binding.h, utils_log
raylib/            git submodule: raysan5/raylib (C source + headers)
src/lib.rs         thin shell that includes the bindgen output
build.rs           drives bindgen + cc for the wrappers + raylib linkage
```
