#[macro_use]
mod macros;

pub mod audio;
pub mod automation;
pub mod callbacks;
pub mod camera;
pub mod collision;
pub mod color;
pub mod data;
pub mod drawing;
pub mod error;
pub mod file;

pub mod input;
pub mod logging;
pub mod math;
pub mod misc;
pub mod models;
pub mod shaders;
pub mod text;
pub mod texture;
pub mod vr;
pub mod window;

use raylib_sys::TraceLogLevel;

use crate::ffi;
use std::ffi::CString;
use std::marker::PhantomData;

// shamelessly stolen from imgui
#[macro_export]
macro_rules! rstr {
    ($e:tt) => ({
        #[allow(unused_unsafe)]
        unsafe {
          std::ffi::CStr::from_bytes_with_nul_unchecked(concat!($e, "\0").as_bytes())
        }
    });
    ($e:tt, $($arg:tt)*) => ({
        #[allow(unused_unsafe)]
        unsafe {
          std::ffi::CString::new(format!($e, $($arg)*)).unwrap()
        }
    })
}

/// This token is used to ensure certain functions are only running on the same
/// thread raylib was initialized from. This is useful for architectures like macos
/// where cocoa can only be called from one thread.
#[derive(Clone, Debug)]
pub struct RaylibThread(PhantomData<*const ()>);

/// The main interface into the Raylib API.
///
/// This is the way in which you will use the vast majority of Raylib's functionality. A `RaylibHandle` can be constructed using the [`init_window`] function or through a [`RaylibBuilder`] obtained with the [`init`] function.
///
/// [`init_window`]: fn.init_window.html
/// [`RaylibBuilder`]: struct.RaylibBuilder.html
/// [`init`]: fn.init.html
#[derive(Debug)]
pub struct RaylibHandle(()); // inner field is private, preventing manual construction

impl Drop for RaylibHandle {
    fn drop(&mut self) {
        unsafe {
            if ffi::IsWindowReady() {
                ffi::CloseWindow();
            }
        }
    }
}

/// A builder that allows more customization of the game window shown to the user before the `RaylibHandle` is created.
///
/// One field per `ConfigFlags` value defined in raylib 6.0
/// (`FLAG_FULLSCREEN_MODE`, `FLAG_WINDOW_*`, `FLAG_MSAA_4X_HINT`,
/// `FLAG_VSYNC_HINT`, `FLAG_INTERLACED_HINT`). Each one maps to a
/// dedicated builder method so callers don't have to reach for
/// `SetConfigFlags` directly.
#[derive(Debug, Default)]
pub struct RaylibBuilder {
    fullscreen_mode: bool,
    window_resizable: bool,
    window_undecorated: bool,
    window_hidden: bool,
    window_minimized: bool,
    window_maximized: bool,
    window_unfocused: bool,
    window_topmost: bool,
    window_always_run: bool,
    window_transparent: bool,
    window_highdpi: bool,
    window_mouse_passthrough: bool,
    borderless_windowed_mode: bool,
    msaa_4x_hint: bool,
    interlaced_hint: bool,
    vsync_hint: bool,
    log_level: TraceLogLevel,
    width: i32,
    height: i32,
    title: String,
}

/// Creates a `RaylibBuilder` for choosing window options before initialization.
pub fn init() -> RaylibBuilder {
    RaylibBuilder {
        width: 640,
        height: 480,
        title: "raylib-rs".to_string(),
        ..Default::default()
    }
}

impl RaylibBuilder {
    /// Sets the window to be fullscreen (exclusive video mode).
    /// Prefer `borderless_windowed` on modern platforms: borderless
    /// avoids the OS-level mode switch and plays nicer with Alt+Tab,
    /// notifications, and multi-monitor setups.
    pub fn fullscreen(&mut self) -> &mut Self {
        self.fullscreen_mode = true;
        self
    }

    /// Sets the window to launch in borderless windowed mode (a
    /// fullscreen-sized window with no chrome). Produces the visual
    /// effect of fullscreen without the latency / flicker of an
    /// exclusive mode switch. Equivalent to `FLAG_BORDERLESS_WINDOWED_MODE`.
    pub fn borderless_windowed(&mut self) -> &mut Self {
        self.borderless_windowed_mode = true;
        self
    }

    /// Set the builder's log level.
    pub fn log_level(&mut self, level: TraceLogLevel) -> &mut Self {
        self.log_level = level;
        self
    }

    /// Sets the window to be resizable.
    pub fn resizable(&mut self) -> &mut Self {
        self.window_resizable = true;
        self
    }

    /// Sets the window to be undecorated (without a border).
    pub fn undecorated(&mut self) -> &mut Self {
        self.window_undecorated = true;
        self
    }

    /// Launches the window hidden. Use `RaylibHandle::clear_window_state`
    /// with `FLAG_WINDOW_HIDDEN` to reveal it later. Useful for
    /// staging window state (size, position, fullscreen) before the
    /// first frame is visible to the user.
    pub fn hidden(&mut self) -> &mut Self {
        self.window_hidden = true;
        self
    }

    /// Launches the window minimized to the taskbar / dock.
    pub fn minimized(&mut self) -> &mut Self {
        self.window_minimized = true;
        self
    }

    /// Launches the window maximized to the work area of its monitor.
    pub fn maximized(&mut self) -> &mut Self {
        self.window_maximized = true;
        self
    }

    /// Launches the window without input focus. Useful for tools or
    /// overlays that shouldn't steal focus from the calling shell.
    pub fn unfocused(&mut self) -> &mut Self {
        self.window_unfocused = true;
        self
    }

    /// Launches the window pinned above other windows.
    pub fn topmost(&mut self) -> &mut Self {
        self.window_topmost = true;
        self
    }

    /// Keeps the window's main loop running while the window is
    /// minimized or otherwise unfocused. Without this flag, raylib
    /// throttles or pauses while the window isn't on screen.
    pub fn always_run(&mut self) -> &mut Self {
        self.window_always_run = true;
        self
    }

    /// Sets the window to be transparent.
    pub fn transparent(&mut self) -> &mut Self {
        self.window_transparent = true;
        self
    }

    /// Sets the window to scale for high DPI displays. Can fix small windows on
    /// certain operating systems when using fractional scaling.
    pub fn highdpi(&mut self) -> &mut Self {
        self.window_highdpi = true;
        self
    }

    /// Lets mouse events pass through the window to whatever is below
    /// it. Pairs naturally with `transparent` for click-through
    /// overlays.
    pub fn mouse_passthrough(&mut self) -> &mut Self {
        self.window_mouse_passthrough = true;
        self
    }

    /// Hints that 4x MSAA (anti-aliasing) should be enabled. The system's graphics drivers may override this setting.
    pub fn msaa_4x(&mut self) -> &mut Self {
        self.msaa_4x_hint = true;
        self
    }

    /// Hints that interlaced rendering should be enabled (3D TVs).
    /// The system's graphics drivers may override this setting.
    pub fn interlaced(&mut self) -> &mut Self {
        self.interlaced_hint = true;
        self
    }

    /// Hints that vertical sync (VSync) should be enabled. The system's graphics drivers may override this setting.
    pub fn vsync(&mut self) -> &mut Self {
        self.vsync_hint = true;
        self
    }

    /// Sets the window's width.
    pub fn width(&mut self, w: i32) -> &mut Self {
        self.width = w;
        self
    }

    /// Sets the window's height.
    pub fn height(&mut self, h: i32) -> &mut Self {
        self.height = h;
        self
    }

    /// Sets the window's width and height.
    pub fn size(&mut self, w: i32, h: i32) -> &mut Self {
        self.width = w;
        self.height = h;
        self
    }

    /// Sets the window title.
    pub fn title(&mut self, text: &str) -> &mut Self {
        self.title = text.to_string();
        self
    }

    /// Builds and initializes a Raylib window.
    ///
    /// # Panics
    ///
    /// Attempting to initialize Raylib more than once will result in a panic.
    pub fn build(&self) -> (RaylibHandle, RaylibThread) {
        use crate::consts::ConfigFlags::*;
        let mut flags = 0u32;
        if self.fullscreen_mode {
            flags |= FLAG_FULLSCREEN_MODE as u32;
        }
        if self.window_resizable {
            flags |= FLAG_WINDOW_RESIZABLE as u32;
        }
        if self.window_undecorated {
            flags |= FLAG_WINDOW_UNDECORATED as u32;
        }
        if self.window_hidden {
            flags |= FLAG_WINDOW_HIDDEN as u32;
        }
        if self.window_minimized {
            flags |= FLAG_WINDOW_MINIMIZED as u32;
        }
        if self.window_maximized {
            flags |= FLAG_WINDOW_MAXIMIZED as u32;
        }
        if self.window_unfocused {
            flags |= FLAG_WINDOW_UNFOCUSED as u32;
        }
        if self.window_topmost {
            flags |= FLAG_WINDOW_TOPMOST as u32;
        }
        if self.window_always_run {
            flags |= FLAG_WINDOW_ALWAYS_RUN as u32;
        }
        if self.window_transparent {
            flags |= FLAG_WINDOW_TRANSPARENT as u32;
        }
        if self.window_highdpi {
            flags |= FLAG_WINDOW_HIGHDPI as u32;
        }
        if self.window_mouse_passthrough {
            flags |= FLAG_WINDOW_MOUSE_PASSTHROUGH as u32;
        }
        if self.borderless_windowed_mode {
            flags |= FLAG_BORDERLESS_WINDOWED_MODE as u32;
        }
        if self.msaa_4x_hint {
            flags |= FLAG_MSAA_4X_HINT as u32;
        }
        if self.interlaced_hint {
            flags |= FLAG_INTERLACED_HINT as u32;
        }
        if self.vsync_hint {
            flags |= FLAG_VSYNC_HINT as u32;
        }

        unsafe {
            ffi::SetConfigFlags(flags);
        }

        unsafe {
            ffi::SetTraceLogLevel(self.log_level as i32);
        }

        let rl = init_window(self.width, self.height, &self.title);

        (rl, RaylibThread(PhantomData))
    }
}

/// Initializes window and OpenGL context.
///
/// # Panics
///
/// Attempting to initialize Raylib more than once will result in a panic.
fn init_window(width: i32, height: i32, title: &str) -> RaylibHandle {
    if unsafe { ffi::IsWindowReady() } {
        panic!("Attempted to initialize raylib-rs more than once!");
    } else {
        unsafe {
            let c_title = CString::new(title).unwrap();
            ffi::InitWindow(width, height, c_title.as_ptr());
        }
        if !unsafe { ffi::IsWindowReady() } {
            panic!("Attempting to create window failed!");
        }

        RaylibHandle(())
    }
}
