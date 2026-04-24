# sola-raylib

sola-raylib is an actively maintained Rust bindings and wrapper for
[raylib](http://www.raylib.com/) 5.5. It currently targets Rust toolchain
version 1.78 or higher.

**Versioning:** sola-raylib's major version tracks raylib's major version — 5.x
binds raylib 5.5, 6.x will bind raylib 6.0, and so on. Minor and patch numbers
are sola-raylib's own (raylib doesn't follow strict semver, so this project
doesn't try to mirror it beyond the major).

This project is a fork of
[github.com/raylib-rs/raylib-rs](https://github.com/raylib-rs/raylib-rs) from
commit
[91bcb492c61dc067945d59357ca6def0d83fcb2c](https://github.com/raylib-rs/raylib-rs/commit/91bcb492c61dc067945d59357ca6def0d83fcb2c)
(v5.5.1 release), which was a fork of
[github.com/deltaphc/raylib-rs](https://github.com/deltaphc/raylib-rs).

Check out the [examples](./examples) directory to find usage examples.

Sola development happens on `main`. Be sure to view the tag version of the
repository if you're wanting to find details on a specific version.

## Features / Bugs

Though this binding tries to stay close to the simple C API, it makes some
changes to be more idiomatic for Rust.

- Resources are automatically cleaned up when they go out of scope (or when
  `std::mem::drop` is called). This is essentially RAII. This means that
  "Unload" functions are not exposed (and not necessary unless you obtain a
  `Weak` resource using make_weak()).
- Most of the Raylib API is exposed through `RaylibHandle`, which is for
  enforcing that Raylib is only initialized once, and for making sure the window
  is closed properly. RaylibHandle has no size and goes away at compile time.
  Because of mutability rules, Raylib-rs is thread safe!
- A `RaylibHandle` and `RaylibThread` are obtained through
  `raylib::init_window(...)` or through the newer `init()` function which will
  allow you to `build` up some window options before initialization (replaces
  `set_config_flags`). RaylibThread should not be sent to any other threads, or
  used in a any syncronization primitives (Mutex, Arc) etc.
- Manually closing the window is unnecessary, because `CloseWindow` is
  automatically called when `RaylibHandle` goes out of scope.
- `Model::set_material`, `Material::set_shader`, and `MaterialMap::set_texture`
  methods were added since one cannot set the fields directly. Also enforces
  correct ownership semantics.
- `Font::from_data`, `Font::set_chars`, and `Font::set_texture` methods were
  added to create a `Font` from loaded `CharInfo` data.
- `SubText` and `FormatText` are omitted, and are instead covered by Rust's
  string slicing and Rust's `format!` macro, respectively.

## Installation

### Supported Platforms

sola-raylib is focused on supporting Windows, Linux, macOS, and Web targets.

The table below shows which core APIs are supported for which platforms:

| API    | Windows            | Linux              | macOS              | Web                |
| ------ | ------------------ | ------------------ | ------------------ | ------------------ |
| core   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: |
| rgui   | :heavy_check_mark: | :heavy_check_mark: | :heavy_check_mark: | ❔                 |
| physac | :construction:     | :construction:     | :construction:     | ❔                 |
| rlgl   | :heavy_check_mark: | :x:                | :x:                | ❔                 |

## Build Dependencies

Requires glfw, cmake, and curl. Tips on making things work smoothly on all
platforms is appreciated. Follow instructions for building raylib for your
platform [here](https://github.com/raysan5/raylib/wiki)

1. Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
sola-raylib = "5.5"
```

Then in your code, use it as `sola_raylib`:

```rust
use sola_raylib::prelude::*;
```

### Drop-in replacement for `raylib-rs`

If you're migrating an existing `raylib-rs` project and don't want to touch
every `use raylib::...` statement, use Cargo's package rename so the crate is
still imported as `raylib` in your source code:

```toml
[dependencies]
raylib = { package = "sola-raylib", version = "5.5" }
```

With that line, all your existing `use raylib::prelude::*` imports keep working
— only `Cargo.toml` changes.

2. Start coding!

```rust
use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
```

## Cross-compiling using `cross`

Cross compiling with sola-raylib can be made easier with cross. See the
[upstream raylib-rs wiki](https://github.com/raylib-rs/raylib-rs/wiki/Cross%E2%80%90compiling-using-cross)
for a writeup that should still largely apply.

## Tech Notes

- Structs holding resources have RAII/move semantics, including: `Image`,
  `Texture2D`, `RenderTexture2D`, `Font`, `Mesh`, `Shader`, `Material`, and
  `Model`.
- `Wave`, `Sound`, `Music`, and `AudioStream` have lifetimes bound to
  `AudioHandle`.
- Functions dealing with string data take in `&str` and/or return an owned
  `String`, for the sake of safety. The exception to this is the gui draw
  functions which take &CStr to avoid per frame allocations. The `rstr!` macro
  helps make this easy.
- In C, `LoadFontData` returns a pointer to a heap-allocated array of `CharInfo`
  structs. In this Rust binding, said array is copied into an owned
  `Vec<CharInfo>`, the original data is freed, and the owned Vec is returned.
- In C, `LoadDroppedFiles` returns a pointer to an array of strings owned by
  raylib. Again, for safety and also ease of use, this binding copies said array
  into a `Vec<String>` which is returned to the caller.
- Linking is automatic, though I've only tested on Windows 10, Ubuntu, and
  MacOS 15. Other platforms may have other considerations.
- OpenGL 3.3, 2.1, and ES 2.0 may be forced via adding `["opengl_33"]`,
  `["opengl_21"]` or `["opengl_es_20]` to the `features` array in your
  Cargo.toml dependency definition.

## Building from source

1. Clone repository: `git clone --recurse-submodules`
2. `cargo build`

### If building for Wayland on Linux

1. Install these packages:\
   `libglfw3-dev wayland-devel libxkbcommon-devel wayland-protocols wayland-protocols-devel libecm-dev`
2. Enable wayland by adding `features=["wayland"]` to your dependency definition

**Note that the packages may not be a comprehensive list, please add details for
your distribution or expand on these packages if you believe this to be
incomplete.**

## Extras

- In addition to the base library, there is also a convenient `ease` module
  which contains various interpolation/easing functions ported from raylib's
  `easings.h`, as well as a `Tween` struct to assist in using these functions.
- Equivalent math and vector operations, ported from `raymath.h`, are `impl`ed
  on the various Vector and Matrix types. Operator overloading is used for more
  intuitive design.

## Contribution & Support

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for more
details.

See [DEVELOPING.md](DEVELOPING.md) for how to work with this repo locally.
