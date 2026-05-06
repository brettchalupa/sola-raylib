//! Cross-platform game loop helper.
//!
//! On native, [`run`] drives a standard
//! `while !rl.window_should_close()` loop. On
//! `wasm32-unknown-emscripten`, it hands the per-frame closure to
//! `emscripten_set_main_loop_arg`, so you don't need `-sASYNCIFY=1` just
//! to keep the loop responsive. Same source on both:
//!
//! ```no_run
//! use sola_raylib::prelude::*;
//!
//! fn main() {
//!     let (rl, thread) = sola_raylib::init()
//!         .size(640, 480)
//!         .title("Hello")
//!         .build();
//!
//!     sola_raylib::core::game_loop::run(rl, thread, 60, |rl, thread| {
//!         let mut d = rl.begin_drawing(thread);
//!         d.clear_background(Color::WHITE);
//!         d.draw_text("Hello", 12, 12, 20, Color::BLACK);
//!     });
//! }
//! ```
//!
//! ## Tradeoff vs Asyncify
//!
//! The classic on-web pattern is to link with `-sASYNCIFY=1`, which
//! instruments every C function so blocking calls can yield to the
//! browser between frames. It works but adds 10 to 30 percent wasm size
//! and a per-call overhead. `run()` skips it by registering the closure
//! with emscripten's event loop directly.
//!
//! The cost is a `'static` closure: no borrowed locals, so `move` your
//! game state in. On emscripten `run` returns immediately while
//! emscripten keeps calling the closure each frame.

use crate::core::{RaylibHandle, RaylibThread};

#[cfg(target_os = "emscripten")]
use std::os::raw::c_void;

#[cfg(target_os = "emscripten")]
extern "C" {
    fn emscripten_set_main_loop_arg(
        func: extern "C" fn(*mut c_void),
        arg: *mut c_void,
        fps: i32,
        simulate_infinite_loop: i32,
    );
}

/// Run `callback` per frame until the window closes (native) or while
/// the page is open (emscripten).
///
/// `fps` sets the target frame rate. On native it forwards to
/// `RaylibHandle::set_target_fps`; on emscripten it's passed to
/// `emscripten_set_main_loop_arg`. Pass `0` for no cap on native or to
/// use `requestAnimationFrame` cadence on emscripten.
///
/// On emscripten, `run` returns as soon as the callback is registered
/// (emscripten owns the event loop), which is why the closure has to be
/// `'static`. On native it returns when the user closes the window.
pub fn run<F>(rl: RaylibHandle, thread: RaylibThread, fps: i32, callback: F)
where
    F: FnMut(&mut RaylibHandle, &RaylibThread) + 'static,
{
    #[cfg(target_os = "emscripten")]
    {
        struct State<F> {
            rl: RaylibHandle,
            thread: RaylibThread,
            callback: F,
        }

        extern "C" fn trampoline<F>(arg: *mut c_void)
        where
            F: FnMut(&mut RaylibHandle, &RaylibThread) + 'static,
        {
            // Safety: `arg` came from Box::into_raw below and is never
            // freed; emscripten calls this forever.
            let state = unsafe { &mut *(arg as *mut State<F>) };
            (state.callback)(&mut state.rl, &state.thread);
        }

        let state = Box::new(State {
            rl,
            thread,
            callback,
        });
        let arg = Box::into_raw(state) as *mut c_void;

        // simulate_infinite_loop=0 lets us return cleanly while emscripten
        // keeps the runtime alive (default EXIT_RUNTIME=0). =1 would
        // unwind via an emscripten-internal exception and corrupt Rust's
        // drop ordering.
        unsafe { emscripten_set_main_loop_arg(trampoline::<F>, arg, fps, 0) };
    }

    #[cfg(not(target_os = "emscripten"))]
    {
        let mut rl = rl;
        let mut callback = callback;
        if fps > 0 {
            rl.set_target_fps(fps as u32);
        }
        while !rl.window_should_close() {
            callback(&mut rl, &thread);
        }
    }
}
