#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::approx_constant)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!(env!("RAYLIB_BINDGEN_LOCATION"));

#[cfg(target_os = "macos")]
pub const MAX_MATERIAL_MAPS: u32 = 12;

// TraceLogLevel is bindgen-generated so we can't use `#[default]` on its
// variants; the manual Default impl is the only way to select LOG_INFO here.
#[allow(clippy::derivable_impls)]
impl Default for TraceLogLevel {
    fn default() -> Self {
        TraceLogLevel::LOG_INFO
    }
}
