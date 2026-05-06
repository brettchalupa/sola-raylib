# Building for the web (wasm + emscripten)

How to ship a sola-raylib game to the browser. Covers the toolchain, the
project-side config, the game-loop choice, asset bundling, save data,
deploy, and the pitfalls.

## Quick start

Four pieces of setup, then build:

1. emscripten installed, `emcc` on PATH.
2. `wasm32-unknown-emscripten` rustup target.
3. A `.cargo/config.toml` with the linker flags raylib needs.
4. A `shell.html` you serve as `index.html`. Cargo writes `.js` + `.wasm`;
   you supply the host page.

Then:

```sh
cargo build --release --target wasm32-unknown-emscripten
# Output: target/wasm32-unknown-emscripten/release/<binary>.{js,wasm}.
# Drop your shell.html into that dir as index.html, then serve it:
cargo install simple-http-server
simple-http-server --index --nocache \
    target/wasm32-unknown-emscripten/release/
```

The rest of the chapter walks through each piece.

## Toolchain

```sh
rustup target add wasm32-unknown-emscripten
```

Emscripten installs separately. Pick one:

- Upstream installer: <https://emscripten.org/docs/getting_started/downloads.html>
- Package manager (`brew install emscripten`, `pacman -S emscripten`).
- `scripts/setup_emscripten.sh` from this repo, which clones emsdk into
  `~/.local/share/emsdk` and pins a known-good version.

> **Pin emsdk to 5.0.6.** emsdk 5.0.7 ships a wasm-opt whose `--asyncify`
> pass fails on wasm built with `-fwasm-exceptions`
> (`__asyncify_get_call_index does not exist`). We need both:
> raylib's audio backend needs ASYNCIFY, and rustc 1.93+ emits
> `-fwasm-exceptions` unconditionally. Until upstream fixes 5.0.7+,
> stay on 5.0.6.

Source the env script in any shell where you build:

```sh
source ~/.local/share/emsdk/emsdk_env.sh
```

Add it to your shell rc if you build often.

## What goes in your project

### `.cargo/config.toml`

Cargo silently drops `cargo:rustc-link-arg` from rlib build scripts, so
`raylib-sys` can't inject linker flags into your binary's link step. They
have to live in your project's `.cargo/config.toml`. The recipe:

```toml
[target.wasm32-unknown-emscripten]
rustflags = [
    # raylib's web backend uses emscripten's bundled GLFW3 port.
    "-C", "link-arg=-sUSE_GLFW=3",
    # Let the wasm heap grow on demand.
    "-C", "link-arg=-sALLOW_MEMORY_GROWTH=1",
    # raylib's audio backend (miniaudio) busy-waits via `emscripten_sleep(1)`
    # during WebAudio init. The first sound play aborts without ASYNCIFY.
    # The classic `while !rl.window_should_close()` loop also needs it.
    "-C", "link-arg=-sASYNCIFY=1",
    # raylib's image and audio loaders reach `fopen` through macros that
    # emcc's tree-shaker sometimes misses. Force the FS layer in.
    "-C", "link-arg=-sFORCE_FILESYSTEM=1",
    # Match the wasm-EH ABI rustc 1.93+ emits. Without this you hit
    # `__cxa_find_matching_catch_*` link errors.
    "-C", "link-arg=-sSUPPORT_LONGJMP=wasm",
    # Recent emcc tree-shakes these aggressively. Without them, raylib's
    # GLFW glue and your shell's audio / heap access hit "Module.X is
    # undefined" at runtime.
    "-C", "link-arg=-sEXPORTED_RUNTIME_METHODS=['ccall','wasmMemory','HEAPU8','HEAP32','HEAPF32']",
]
```

For assets, add `--preload-file` lines too; see "Bundling assets" below.

If your project pulls C / C++ in via cc-rs (e.g. `mlua` with the
`vendored` feature), match the link ABI rustc 1.93+ emits by adding:

```toml
[env]
CFLAGS_wasm32_unknown_emscripten = "-fwasm-exceptions -sSUPPORT_LONGJMP=wasm"
CXXFLAGS_wasm32_unknown_emscripten = "-fwasm-exceptions -sSUPPORT_LONGJMP=wasm"
```

Without these the foreign `.o` files use legacy JS-EH and the link
fails on `__cxa_find_matching_catch_*`. raylib's own C compiles fine
without this block.

### `shell.html`

Cargo always names the link output `<binary>.js`, so emscripten's
`--shell-file` mechanism is unusable from a Rust build. Ship your own
`index.html` instead:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>my game</title>
    <style>
      html, body {
        margin: 0;
        height: 100%;
        background: #000;
        overflow: hidden;
      }
      body {
        display: grid;
        place-items: center;
      }
      canvas {
        display: block;
        max-width: 100vw;
        max-height: 100vh;
        image-rendering: pixelated;
      }
    </style>
  </head>
  <body>
    <canvas
      id="canvas"
      oncontextmenu="event.preventDefault()"
      tabindex="-1"
    ></canvas>
    <script>
      // Browsers block AudioContext until the user interacts with the page.
      // Resume on first click so raylib's audio comes through.
      window.addEventListener("click", function () {
        var Ctx = window.AudioContext || window.webkitAudioContext;
        if (!Ctx) return;
        [Ctx, Module && Module.audioContext].forEach(function (ctx) {
          if (ctx && ctx.state === "suspended") ctx.resume();
        });
      }, { once: true });
      var Module = {
        canvas: document.getElementById("canvas"),
        print: function (t) {
          console.log(t);
        },
        printErr: function (t) {
          console.error(t);
        },
      };
    </script>
    <script src="my_game.js"></script>
  </body>
</html>
```

Replace `my_game.js` with your binary name plus `.js`. Drop this file into the
directory you serve, alongside `<binary>.wasm` and `<binary>.js`.

### What sola-raylib handles automatically

raylib's C source compiles cleanly under rustc 1.93+'s default link ABI
without any sys-side cflag tweaks (verified empirically on emsdk 5.0.6).
The `[env]` block above only matters if your project has its own C / C++
deps via cc-rs that need to match the link-side ABI.

## The game loop: `game_loop::run` vs Asyncify

raylib's standard loop on native looks like:

```rust
while !rl.window_should_close() {
    let mut d = rl.begin_drawing(&thread);
    // ...
}
```

`begin_drawing` ends in a blocking `glfwSwapBuffers`. The browser has no
notion of "block": JavaScript is single-threaded and event-loop-driven,
so a wasm function that doesn't return freezes the tab.

The recipe above includes `-sASYNCIFY=1` (raylib's audio backend wants
it anyway), so the classic loop works. `game_loop::run` is the
alternative: it hands the per-frame closure to
`emscripten_set_main_loop_arg` and lets the browser drive frames
directly.

### `game_loop::run`

[`sola_raylib::core::game_loop::run`](https://docs.rs/sola-raylib/latest/sola_raylib/core/game_loop/fn.run.html)
registers a per-frame closure with emscripten and returns immediately;
emscripten calls it on each animation frame. On native it just runs the
while-loop. Same source for both targets:

```rust
use sola_raylib::core::game_loop;
use sola_raylib::prelude::*;

fn main() {
    let (rl, thread) = sola_raylib::init().size(640, 480).title("Hello").build();

    game_loop::run(rl, thread, 60, |rl, thread| {
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Hello", 12, 12, 20, Color::BLACK);
    });
}
```

The closure has to be `'static`, so any state crossing frames goes in
via `move`. That's the only constraint.

### Opting out of Asyncify

Asyncify adds ~100 KB and a per-call overhead. If your game has no audio
(no `InitAudioDevice`) and you use `game_loop::run` for the loop, drop
`-sASYNCIFY=1` from the rustflags. Put it back the moment audio comes in
or you switch back to the while-loop.

## Bundling assets

Emscripten gives the wasm a virtual FS (MEMFS by default). Anything you
read from disk has to be in that FS, either preloaded at link time or
fetched at runtime. Most games preload. Add to your rustflags:

```toml
"-C", "link-arg=--preload-file", "-C", "link-arg=assets@/assets",
```

`--preload-file SRC@DEST` packs `SRC` (relative to the link CWD, which
is your project root) into the VFS at `DEST`. Emcc writes a `<crate>.data`
next to the `.js` and `.wasm`, plus JS that fetches and unpacks it
before `main` runs. All four files (`index.html`, `.js`, `.wasm`,
`.data`) need to end up in your served directory.

raylib's `fopen`/`fread` go through the VFS. A path like
`"assets/sprites.png"` in Rust resolves to `/assets/sprites.png`
because emscripten's CWD is `/`. The `-sFORCE_FILESYSTEM=1` already in
the recipe makes sure the FS API isn't tree-shaken out.

For bigger games, look at `--use-preload-plugins`, IDBFS, or runtime
`fetch`.

## Memory tuning

Defaults: 16 MB initial heap, growth allowed. For a heavy game, raise
the initial size so the first few growths don't stall the page:

```toml
"-C", "link-arg=-sINITIAL_MEMORY=134217728",   # 128 MB
```

`-sMAXIMUM_MEMORY=N` caps the upper bound. Default is 4 GB; leave it
unless you have a reason.

## Audio: the user-gesture rule

Browsers block AudioContext until the page sees a user gesture (click,
keypress, touch). raylib's audio backend creates the context in
`InitAudioDevice` but it stays suspended until you resume after a
gesture. The shell.html above handles this on first click. If your game
has its own click-to-start UI, do the resume there instead.

## Save data

raylib's `SaveStorageValue` / `LoadStorageValue` write a `storage.data`
file. On emscripten that lands in MEMFS by default, which is ephemeral.
For real saves, mount IDBFS so a directory mirrors to IndexedDB:

```js
// Inside your shell.html, before main() runs:
Module.preRun = [function () {
  FS.mkdir("/save");
  FS.mount(IDBFS, {}, "/save");
  FS.syncfs(true, function () {}); // pull from IndexedDB on load
}];
```

Then write under `/save/...` from Rust. After each save, call
`FS.syncfs(false, ...)` to flush back to IndexedDB; it doesn't happen
automatically, so an unflushed session loses the save when the tab
closes.

For a few key/value pairs, `localStorage` via `emscripten_run_script` is
simpler. raylib's storage API is file-backed, so stay on IDBFS if you use
it.

## Deploying

A release build produces:

- `<name>.wasm`
- `<name>.js`
- `<name>.data` (if you use `--preload-file`)
- `index.html` (yours)

Any static host works. Make sure `.wasm` is served as `application/wasm`
and `.data` as `application/octet-stream` for streaming decode. GitHub
Pages, Netlify, Vercel, Cloudflare Pages, and S3 + CloudFront already do.
For self-hosted nginx, add `types { application/wasm wasm; }` if your
mime types file is older than 2017.

Brotli or gzip on the `.wasm` and `.data` roughly halves transfer size.

For SharedArrayBuffer (multithreaded wasm), the host also needs
`Cross-Origin-Opener-Policy: same-origin` and
`Cross-Origin-Embedder-Policy: require-corp`. raylib's default build
doesn't need this; pthread-enabled audio or worker threads would.

## Pitfalls

- **`emcc not found`** during `cargo build`. emsdk_env.sh has not been sourced.
  Run `source path/to/emsdk/emsdk_env.sh` and try again.
- **Black canvas with no log output.** Almost always a missing `--preload-file`
  (raylib silently fails to load a missing texture) or a path mismatch (the CWD
  inside the wasm is `/`, not your host CWD). Check the browser DevTools console
  for `404` or `LoadTexture: failed` lines.
- **`undefined symbol: glfwInit` at link time.** `-sUSE_GLFW=3` is missing from
  your rustflags.
- **Page hangs after a few seconds.** You dropped `-sASYNCIFY=1` while still
  using the classic `while !rl.window_should_close()` pattern. Put it back, or
  switch to `game_loop::run`.
- **First call to `PlaySound` aborts the wasm.** Same root cause: ASYNCIFY is
  off but raylib's audio backend needs it.
- **`__cxa_find_matching_catch_*` at link time.** wasm-EH ABI mismatch. Make
  sure both `-sSUPPORT_LONGJMP=wasm` is in rustflags and your `[env]` block sets
  `CFLAGS_wasm32_unknown_emscripten = "-fwasm-exceptions
  -sSUPPORT_LONGJMP=wasm"`
  if you have other cc-rs deps.
- **`Fatal: Module::getFunction: __asyncify_get_call_index does not exist`.**
  emsdk 5.0.7 wasm-opt bug. Pin to emsdk 5.0.6.
- **`abort(OOM)` on a large texture load.** Initial heap too small. Raise
  `-sINITIAL_MEMORY=N`.
- **Save file disappears on reload.** You did not mount IDBFS or did not call
  `FS.syncfs(false, ...)` after saving.
