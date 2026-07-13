//! Bundled gamepad mappings (SDL_GameControllerDB).
//!
//! raylib freezes its controller mappings at build time (GLFW embeds them in
//! `mappings.h`; SDL keeps its own table), so games built against an older
//! raylib miss controllers added since. sola-raylib bundles a more recent
//! copy of the community [SDL_GameControllerDB] and, by default, loads it over
//! raylib's built-in table when the window is created. See
//! [`crate::core::RaylibBuilder::bundled_gamepad_mappings`].
//!
//! The database is vendored from [SDL_GameControllerDB] (zlib license, see
//! `raylib/src/gamecontrollerdb.LICENSE.txt`). Refresh it with
//! `just update-gamecontrollerdb`.
//!
//! [SDL_GameControllerDB]: https://github.com/mdqinc/SDL_GameControllerDB

/// The bundled SDL_GameControllerDB, embedded at compile time.
///
/// This is the raw `gamecontrollerdb.txt` text, suitable for
/// [`crate::core::RaylibHandle::set_gamepad_mappings`].
pub const BUNDLED: &str = include_str!("../gamecontrollerdb.txt");
