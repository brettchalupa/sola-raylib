/* raylib-rs
   lib.rs - Main library code (the safe layer)

Copyright (c) 2018-2024 raylib-rs team

This software is provided "as-is", without any express or implied warranty. In no event will the authors be held liable for any damages arising from the use of this software.

Permission is granted to anyone to use this software for any purpose, including commercial applications, and to alter it and redistribute it freely, subject to the following restrictions:

  1. The origin of this software must not be misrepresented; you must not claim that you wrote the original software. If you use this software in a product, an acknowledgment in the product documentation would be appreciated but is not required.

  2. Altered source versions must be plainly marked as such, and must not be misrepresented as being the original software.

  3. This notice may not be removed or altered from any source distribution.
*/

//! # sola-raylib
//!
//! `sola_raylib` is a safe Rust binding to [Raylib](https://www.raylib.com/), a C library for enjoying games programming.
//!
//! To get started, take a look at the [`init_window`] function. This initializes Raylib and shows a window, and returns a [`RaylibHandle`]. This handle is very important, because it is the way in which one accesses the vast majority of Raylib's functionality. This means that it must not go out of scope until the game is ready to exit. You will also recieve a !Send and !Sync [`RaylibThread`] required for thread local functions.
//!
//! For more control over the game window, the [`init`] function will return a [`RaylibBuilder`] which allows for tweaking various settings such as VSync, anti-aliasing, fullscreen, and so on. Calling [`RaylibBuilder::build`] will then provide a [`RaylibHandle`].
//!
//! Some useful constants can be found in the [`consts`] module, which is also re-exported in the [`prelude`] module. In most cases you will probably want to `use sola_raylib::prelude::*;` to make your experience more smooth.
//!
//! [`init_window`]: fn.init_window.html
//! [`init`]: fn.init.html
//! [`RaylibHandle`]: struct.RaylibHandle.html
//! [`RaylibThread`]: struct.RaylibThread.html
//! [`RaylibBuilder`]: struct.RaylibBuilder.html
//! [`RaylibBuilder::build`]: struct.RaylibBuilder.html#method.build
//! [`consts`]: consts/index.html
//! [`prelude`]: prelude/index.html
//!
//! # Examples
//!
//! The classic "Hello, world":
//!
//! ```no_run
//! use sola_raylib::prelude::*;
//!
//! fn main() {
//!     let (mut rl, thread) = sola_raylib::init()
//!         .size(640, 480)
//!         .title("Hello, World")
//!         .build();
//!
//!     while !rl.window_should_close() {
//!         let mut d = rl.begin_drawing(&thread);
//!
//!         d.clear_background(Color::WHITE);
//!         d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
//!     }
//! }
//! ```
//!
//! ## Cargo features
//!
//! `sola-raylib` exposes Cargo features that toggle build-time options on the
//! underlying raylib C library. The full reference, with platform notes and
//! known-broken flags, lives in the [project README]; the quick summary:
//!
//! **Build / linking**
//! - `bindgen` *(default)*: generate FFI bindings at build time. Disable to
//!   supply a hand-rolled `bindings.rs` (for platforms bindgen can't target).
//! - `nobuild`: skip building and linking raylib entirely. You then link it
//!   yourself (used for docs.rs and headless setups).
//!
//! **Platform and rendering backend**
//! - `wayland`: build raylib's GLFW with native Wayland support on Linux.
//!   Requires the system `glfw-devel` (cmake config files), not just the
//!   runtime.
//! - `sdl`: use the SDL platform backend. The build script auto-picks SDL3 if
//!   present, otherwise SDL2, via `pkg-config`.
//! - `opengl_33` / `opengl_21` / `opengl_es_20` / `opengl_es_30`: force a
//!   specific GL backend.
//! - `software_render`, `platform_memory`, `platform_web_rgfw`: experimental
//!   raylib 6.0 backends. See the README for caveats.
//!
//! **Behavior toggles**
//! - `custom_frame_control`: enable raylib's `SUPPORT_CUSTOM_FRAME_CONTROL`,
//!   so you drive frame timing yourself.
//! - `noscreenshot`: disable raylib's built-in F12 screenshot keybind.
//!
//! **Interop**
//! - `with_serde`: derive `serde::Serialize` / `Deserialize` on public types.
//! - `convert_mint`: conversions to/from the [`mint`](https://docs.rs/mint)
//!   math types.
//!
//! [project README]: https://github.com/brettchalupa/sola-raylib#cargo-features

#![allow(dead_code)]
pub mod consts;
pub mod core;
pub mod ease;
pub mod prelude;
pub mod rgui;

/// The raw, unsafe FFI binding, in case you need that escape hatch or the safe layer doesn't provide something you need.
pub mod ffi {
    pub use raylib_sys::*;
}

pub use crate::core::collision::*;
pub use crate::core::misc::open_url;
pub use crate::core::*;

// Re-exports
#[cfg(feature = "with_serde")]
pub use serde;
