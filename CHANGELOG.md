# sola-raylib Changelog

## 6.0.0 - Apr 24, 2026

Upgrade to **raylib 6.0** and **raygui 5**. sola-raylib's major version tracks
raylib's, so 6.x binds raylib 6.0. See raylib's [6.0 release notes][raylib-6]
for the full upstream story.

### Breaking

- raylib 6.0 — upstream redesigned skeletal animation, fullscreen modes, the
  build-config system, and more. Most safe wrappers are unchanged; the
  exceptions are below.
- `RaylibModel::bind_pose` and `bind_pose_mut` now return `Option<&[Transform]>`
  (was `Option<&Transform>`). The underlying field is an array sized by
  `boneCount`, so the old signature read garbage past the first bone.
- Removed `RaylibMesh::indicies` / `indicies_mut`. The typo'd aliases were
  deprecated in 5.5.3 with a "removed in 6.0" note — use `indices` /
  `indices_mut`.
- `DrawModelPoints` / `DrawModelPointsEx` removed — raylib 6.0 dropped these
  from its public API.
- **Removed the rlImGui / imgui integration.** The `imgui` feature, the
  `raylib::imgui` module, the `RayImGUITrait` and friends, the rlImGui
  submodule, and the `imgui` example are all gone. sola-raylib is focused on
  raylib + raygui; if you want imgui-in-raylib, use a community bridge crate or
  roll a small integration crate against `sola-raylib-sys`.
- **Removed the physac status row from the README.** There was no actual physac
  binding in the safe crate; the row was aspirational. Physac itself is a
  separate raylib-extras library — a real binding would live in its own crate.

### Added

New API wrappers for raylib 6.0 additions:

- Models: `update_model_animation_ex` (animation blending).
- Input: `get_key_name`.
- Hashing: `compute_sha256`, `compute_sha1`, `compute_md5`, `compute_crc32`.
- Shapes (`RaylibDraw`): `draw_line_dashed`, `draw_ellipse_v`,
  `draw_ellipse_lines_v`.
- Text: `measure_text_codepoints`.
- Math: `Vector2::cross_product`, `Matrix::multiply_value`, `Matrix::compose`.
- Pixel helpers in `core::texture`: `get_pixel_color` and `set_pixel_color`
  (take a `PixelFormat` enum and byte slice, validate slice length).

### Fixed

- **`RaylibMesh::tangents` / `tangents_mut` return `&[Vector4]` /
  `&mut
  [Vector4]`** (was `Vector3`). Raylib stores tangents as
  `float[4 *
  vertexCount]` (XYZW where W is the bitangent sign), but the
  previous cast sliced 3 of every 4 floats and produced misaligned reads.
  Covered by a new unit test in `raylib/src/core/models.rs`.
- **CMake `USE_WAYLAND` flag updated to `GLFW_BUILD_WAYLAND`.** Upstream renamed
  the knob in 6.0; our `wayland` feature was silently a no-op until this fix.
- Removed two long-silent no-op CMake defines (`SUPPORT_BUSY_WAIT_LOOP=OFF`,
  `SUPPORT_FILEFORMAT_JPG=ON`). raylib 6.0 ignores `SUPPORT_*` overrides unless
  `CUSTOMIZE_BUILD=ON` is also set; experimentally turning that on caused
  unresponsive windows (black screen + "window is not responding") on Linux/X11.
  Keeping raylib's config.h defaults is the safe choice for 6.0.0.
  `custom_frame_control`, `noscreenshot`, and `nogif` features still activate
  `CUSTOMIZE_BUILD=ON` locally when enabled, so those paths keep working.
- Audio: `Wave::export_as_code`. Raw audio-thread processor hooks as
  `unsafe fn`s that take `extern "C" fn(*mut c_void, u32)` pointers:
  `AudioStream::attach_audio_stream_processor` /
  `detach_audio_stream_processor`, and
  `RaylibAudio::attach_audio_mixed_processor` / `detach_audio_mixed_processor`.
  Ergonomic closure wrapping is future work; the raw pointer interface is honest
  about the audio-thread risk.

New examples exercising the new surface: `animation_blending`, `shapes_new`,
`borderless_fullscreen`, `pixel_color` (HSV wheel painted via `set_pixel_color`,
read back under the cursor with `get_pixel_color`). The `input` example now also
displays `get_key_name`.

### Other

- `raylib-sys/README.md` rewritten to reflect the current build-time bindgen
  flow; the old doc described a workflow that no longer exists.
- `DEVELOPING.md` gains a "Bumping raylib" checklist.
- New opt-in feature flags for raylib 6.0's new platform backends. **All three
  are experimental upstream and may not work well yet.** Raylib 6.0 shipped them
  as new backends with known gaps. We expose the flags so you can opt in; what
  actually renders or links is whatever upstream supports today.
  - `software_render`: build raylib with the CPU `rlsw` backend
    (`OPENGL_VERSION=Software`). rlsw is **not compatible with the default GLFW
    desktop backend** per upstream (raylib#5664), so use it via
    `--features "sdl,software_render"` on Linux/macOS (requires SDL2 dev
    headers). `just example-sw <name>` and `just examples-sw` wire that combo
    for you. Windows would need the `rcore_desktop_win32` native backend, which
    sola-raylib does not wire yet.
  - `platform_memory`: build raylib with the headless `PLATFORM=Memory` backend.
    The feature compiles the backend, but reading the framebuffer requires
    `rlsw.h` APIs (e.g. `swGetColorBuffer`) that are **not yet wrapped** in the
    safe crate. Post-release work.
  - `platform_web_rgfw`: use raylib's RGFW-based web backend
    (`PLATFORM=WebRGFW`) when cross-compiling to `wasm32-unknown-emscripten`. No
    local demo path. Requires an emscripten build loop.

### Dependencies

- Bumped `thiserror` from `1.x` to `2.x` in the public dep graph. Our public
  `Error` type still implements `std::error::Error`, so no API change for users.
  Only matters if you pin `thiserror = "1"` elsewhere in your `Cargo.toml`;
  cargo will resolve both versions side by side, or you can bump your own pin.
- Dev-deps (`rand 0.8 → 0.10`) and build-deps (`cmake`, `cc`, `bindgen` patch
  updates) refreshed to current versions. These are not in the downstream user's
  dep graph.
- Patch/minor refresh via `cargo update` across `cfg-if`, `paste`, `seq-macro`,
  `serde`, `serde_json`, `ringbuf`.

[rlsw-pr]: https://github.com/raysan5/raylib/pull/4832
[raylib-6]: https://github.com/raysan5/raylib/releases/tag/6.0

## 5.5.3 - Apr 24, 2026

Doc improvements and bug fixes ported from upstream raylib-rs, scoped to
soundness and correctness. Thanks to all the original authors linked below.

### Breaking

- `Image::export_image_to_memory` now returns `Result<Vec<u8>, Error>` instead
  of `Result<&[u8], Error>`. The previous signature leaked on every call
  (raylib's buffer was never freed) and had an unsound lifetime. Adapted from
  raylib-rs [#250][rr-250] / [#247][rr-247].

### Deprecated

- `RaylibMesh::indicies` / `indicies_mut` — use `indices` / `indices_mut`. The
  typo'd names stay through 5.x and are removed in 6.0.

### Fixed

- **Soundness:** `RaylibMesh` accessors return an empty slice when the
  underlying buffer is null instead of invoking UB via
  `slice::from_raw_parts(null, _)` ([raylib-rs#257][rr-257]).
- **Soundness:** `RaylibMesh::indices` length is now `triangleCount * 3` (what
  raylib allocates) rather than `vertexCount` ([raylib-rs#257][rr-257]).
- **Soundness:** `Sound::alias` lifetimes correctly bind the sound's lifetime to
  the receiver ([`4d53bd4`][rr-4d53bd4]).
- **Use-after-free:** `load_model_animations` frees only the outer array via
  `MemFree`, rather than `UnloadModelAnimations` which also tore down the
  animations the `Vec` had just taken ownership of ([`ccc0827`][rr-ccc0827]).
- **Data corruption:** `AudioStream::update` passes element count (not byte
  size) to `ffi::UpdateAudioStream`, matching the C API
  ([raylib-rs#212][rr-212]).
- `Texture2D::update_texture_rec` validates the destination rectangle and sizes
  the expected pixel buffer from rect dimensions ([`cb0c004`][rr-cb0c004]).
- Closure-based draw helpers (`draw`, `draw_mode2D`, etc.) no longer call
  `ffi::End*()` twice — the handle's `Drop` already does
  ([`aadf1a7`][rr-aadf1a7]).
- `gui_text_box` and `gui_text_input_box` grow the buffer and round-trip the
  null terminator, so text input works ([`e2be94b`][rr-e2be94b]).
- `gui_panel` passes a null pointer for empty strings, matching `GuiPanel`'s C
  API ([`95234d1`][rr-95234d1]).

### Other

- Add docs to README on drop ordering.
- CI and `just ok` build with `--all-targets` so examples are exercised
  alongside the library crates.
- Examples look better on High DPI screens.
- `just examples` to quickly run a bunch of different examples.

[rr-212]: https://github.com/raylib-rs/raylib-rs/pull/212
[rr-247]: https://github.com/raylib-rs/raylib-rs/issues/247
[rr-250]: https://github.com/raylib-rs/raylib-rs/pull/250
[rr-257]: https://github.com/raylib-rs/raylib-rs/pull/257
[rr-4d53bd4]: https://github.com/raylib-rs/raylib-rs/commit/4d53bd49af9a437433b7dff92b38cb06831351df
[rr-95234d1]: https://github.com/raylib-rs/raylib-rs/commit/95234d17a1943e00b06e30f8323a63b78323880c
[rr-aadf1a7]: https://github.com/raylib-rs/raylib-rs/commit/aadf1a76be022f197460d061c17570803010df58
[rr-cb0c004]: https://github.com/raylib-rs/raylib-rs/commit/cb0c0048b6c80ec8e487fafad2b07da1886007c8
[rr-ccc0827]: https://github.com/raylib-rs/raylib-rs/commit/ccc0827b9667578476c67b6c9d3b37a1b167034e
[rr-e2be94b]: https://github.com/raylib-rs/raylib-rs/commit/e2be94bab26db3a30bca7226e5f03a4b15c54b0e

## 5.5.2 - Apr 23, 2026

- Renamed to sola-raylib and sola-raylib-sys
- Fixed incorrect param ordering for `gui_list_view_ex`
- Added `window_highdpi` to `RaylibBuilder` and associated `highdpi` builder
  function
- Expanded CI
- Formatting and linter fixes
- Clean up files in repo

---

Incomplete changelog below from raylib-rs. Keeping around for posterity's sake.

## 3.7.0

- [core] ADDED: LoadVrStereoConfig()
- [core] ADDED: UnloadVrStereoConfig()
- [core] ADDED: BeginVrStereoMode()
- [core] ADDED: EndVrStereoMode()

- [core] ADDED: GetCurrentMonitor() (#1485) by @object71 [core] ADDED:
  SetGamepadMappings() (#1506)
- [core] RENAMED: struct Camera: camera.type to camera.projection
- [core] RENAMED: LoadShaderCode() to LoadShaderFromMemory() (#1690)
- [core] RENAMED: SetMatrixProjection() to rlSetMatrixProjection()
- [core] RENAMED: SetMatrixModelview() to rlSetMatrixModelview()
- [core] RENAMED: GetMatrixModelview() to rlGetMatrixModelview()
- [core] RENAMED: GetMatrixProjection() to rlGetMatrixProjection()
- [core] RENAMED: GetShaderDefault() to rlGetShaderDefault() [core] RENAMED:
  GetTextureDefault() to rlGetTextureDefault() [core] REMOVED:
  GetShapesTexture() [core] REMOVED: GetShapesTextureRec() [core] REMOVED:
  GetMouseCursor()
- [core] REMOVED: SetTraceLogExit() [core] REVIEWED: GetFileName() and
  GetDirectoryPath() (#1534) by @gilzoide [core] REVIEWED: Wait() to support
  FreeBSD (#1618) [core] REVIEWED: HighDPI support on macOS retina (#1510)
  [core] REDESIGNED: GetFileExtension(), includes the .dot [core] REDESIGNED:
  IsFileExtension(), includes the .dot [core] REDESIGNED: Compresion API to use
  sdefl/sinfl libs
- [rlgl] ADDED: SUPPORT_GL_DETAILS_INFO config flag [rlgl] REMOVED:
  GenTexture\*() functions (#721) [rlgl] REVIEWED: rlLoadShaderDefault() [rlgl]
  REDESIGNED: rlLoadExtensions(), more details exposed [raymath] REVIEWED:
  QuaternionFromEuler() (#1651) [raymath] REVIEWED: MatrixRotateZYX() (#1642)
- [shapes] ADDED: DrawLineBezierQuad() (#1468) by @epsilon-phase [shapes] ADDED:
  CheckCollisionLines()
- [shapes] ADDED: CheckCollisionPointLine() by @mkupiec1 [shapes] REVIEWED:
  CheckCollisionPointTriangle() by @mkupiec1 [shapes] REDESIGNED:
  SetShapesTexture()
- [shapes] REDESIGNED: DrawCircleSector(), to use float params
- [shapes] REDESIGNED: DrawCircleSectorLines(), to use float params
- [shapes] REDESIGNED: DrawRing(), to use float params
- [shapes] REDESIGNED: DrawRingLines(), to use float params
- [textures] ADDED: DrawTexturePoly() and example (#1677) by @chriscamacho
- [textures] ADDED: UnloadImageColors() for allocs consistency [textures]
  RENAMED: GetImageData() to LoadImageColors() [textures] REVIEWED:
  ImageClearBackground() and ImageDrawRectangleRec() (#1487) by @JeffM2501
  [textures] REVIEWED: DrawTexturePro() and DrawRectanglePro() transformations
  (#1632) by @ChrisDill [text] REDESIGNED: DrawFPS()
- [models] ADDED: UploadMesh() (#1529) [models] ADDED: UpdateMeshBuffer()
  [models] ADDED: DrawMesh() [models] ADDED: DrawMeshInstanced() [models] ADDED:
  UnloadModelAnimations() (#1648) by @object71 :( [models] REMOVED: DrawGizmo()
- [models] REMOVED: LoadMeshes() [models] REMOVED: MeshNormalsSmooth()
- [models] REVIEWED: DrawLine3D() (#1643) [audio] REVIEWED: Multichannel sound
  system (#1548) [audio] REVIEWED: jar_xm library (#1701) by @jmorel33 [utils]
  ADDED: SetLoadFileDataCallback() [utils] ADDED: SetSaveFileDataCallback()
  [utils] ADDED: SetLoadFileTextCallback() [utils] ADDED:
  SetSaveFileTextCallback() [examples] ADDED: text_draw_3d (#1689) by @Demizdor
  [examples] ADDED: textures_poly (#1677) by @chriscamacho [examples] ADDED:
  models_gltf_model (#1551) by @object71 [examples] RENAMED:
  shaders_rlgl_mesh_instanced to shaders_mesh_intancing [examples] REDESIGNED:
  shaders_rlgl_mesh_instanced by @moliad [examples] REDESIGNED:
  core_vr_simulator [examples] REDESIGNED: models_yaw_pitch_roll [build] ADDED:
  Config flag: SUPPORT_STANDARD_FILEIO [build] ADDED: Config flag:
  SUPPORT_WINMM_HIGHRES_TIMER (#1641) [build] ADDED: Config flag:
  SUPPORT_GL_DETAILS_INFO [build] ADDED: Examples projects to VS2019 solution
  [build] REVIEWED: Makefile to support PLATFORM_RPI (#1580) [build] REVIEWED:
  Multiple typecast warnings by @JeffM2501 [build] REDESIGNED: VS2019 project
  build paths [build] REDESIGNED: CMake build system by @object71 [_] RENAMED:
  Several functions parameters for consistency [_] UPDATED: Multiple bindings to
  latest version [_] UPDATED: All external libraries to latest versions [_]
  Multiple code improvements and fixes by multiple contributors!

## 3.5.0 (Done)

Added: SetWindowState Added: ClearW‌indowState Added: IsWindowFocused Added:
GetWindowScaleDPI Added: GetMonitorRefreshRate Added: IsCursorOnScreen Added:
SetMouseCursor/GetMouseCursor Added: Normalize Added: Remap Added:
Vector2Reflect Added: Vector2LengthSqr Added: Vector2MoveTowards Added:
UnloadFontData Added: LoadFontFromMemmory(ttf) Added: ColorAlphaBlend Added:
GetPixelColor Added: SetPixelColor Added: LoadImageFromMemory Added:
LoadImageAnim Added: DrawTextureTiled Added: UpdateTextureRec Added:
UnloadImageColors, Added: UnloadImagePallet, Added: UnloadWaveSample Added:
DrawTriangle3D Added: DrawTriangleStrip3D Added: LoadWaveFromMemory Added:
MemAlloc() / MemFree() Added: UnloadFileData Added: UnloadFileText

## 0.10.0 (WIP)

- Basic macOS support. Currently untested.
- Improved ergonomics across the board:
  - Copied over and tweaked many FFI structs so that fields use proper types
    instead of FFI types.
  - Added `vec2`, `vec3`, `quat`, `rgb`, and `rgba` convenience functions for a
    middle ground between `From` conversion and `new` methods.
  - Changed several key and gamepad functions to use `u32`, making it more
    ergonomic with key/gamepad constants.
  - Added optional `prelude` module for conveniently bringing in all the common
    types and definitions.
- Fixed unnecessary `&mut` in `load_image_ex` and `draw_poly_ex`.
- Fixed linking on MSVC toolchains by including `user32`.
- Prevent `RaylibHandle` from being manually constructed. Fixes a safety
  soundness hole.

## 0.9.1

- Fixed docs.rs build by removing use of a uniform module path. This also keeps
  the crate compatible with Rust 1.31+.

## 0.9.0

- Initial crates.io release
