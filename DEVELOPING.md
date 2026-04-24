# Developing

Documentation on how to work on Sola.

## Style

sola-raylib follows Rust styles and lint rules. It aims to have no warnings.

## Commands

[just](https://just.systems/man/en/) is used for running commands while working
on the project. See `./justfile` for available commands.

Run `just setup` to initialize the needed Git submodules.

## Verifying Your Changes

The best way to verify your changes is to add a project to ./examples that
exercises the changed code. Then add that to `just examples` in the `justfile`.
That way it can be verified to not break with future changes.

If you want to verify your changes haven't broken other code and examples, run
`just examples` to quickly test a bunch of different functionality in
sola-raylib.

Ensure `just ok` is passing before submitting changes.

## Releasing

Guide to creating new releases on crates.io.

### Pre-flight

- Be on `main` with a clean working tree:
  `git checkout main && git pull && git status`
- `just ok` is green
- `just examples` all work

### Steps

1. **Bump the version in a PR.** The version lives in several places — update
   all of them together and add the release notes to
   [CHANGELOG.md](CHANGELOG.md):
   - `raylib-sys/Cargo.toml` — its own `version`
   - `raylib/Cargo.toml` — its own `version` **and** the `version = "X.Y.Z"`
     pinned on the `raylib-sys` dep line (easy to miss)
   - Any version references in `README.md`
2. **After the PR merges, tag the merge commit and push the tag:**
   ```
   git checkout main && git pull
   git tag v5.5.2      # replace with the new version
   git push origin v5.5.2
   ```
3. **Dry-run publish** to catch manifest issues while everything is still
   reversible:
   ```
   cargo publish --workspace --dry-run
   ```
4. **Publish for real:**
   ```
   cargo publish --workspace
   ```
   `--workspace` (stable since Cargo 1.90) publishes all workspace members in
   dependency order and waits for each crate to be indexed on crates.io before
   publishing anything that depends on it — so `sola-raylib-sys` goes first and
   `sola-raylib` only publishes once `-sys` at the new version is resolvable.
5. **Create the GitHub release** from the tag, using the `gh` CLI with the
   relevant CHANGELOG section as the release notes:
   ```
   # Extract this version's section from CHANGELOG.md into notes.md, then:
   gh release create v5.5.2 --title "v5.5.2" --notes-file notes.md
   rm notes.md
   ```
6. **Open a follow-up PR** that bumps every version from step 1 to the next dev
   version (e.g. `5.5.3-dev.0`) so future work on `main` is clearly not a
   released version.

[Cargo reference](https://doc.rust-lang.org/cargo/reference/publishing.html)

## NixOS

To use raylib-rs on NixOS there's a provided nix-shell file `shell.nix` at the
root of the repo that should get you up and running, which can be used like so:

`nix-shell ./shell.nix`

You'll also need to enable the Wayland feature on the raylib crate:

`cargo add raylib -F wayland`

Contributions are welcome to improve or fix the shell.nix!

## Testing

Compile-time and unit tests run via `just test` (which shells out to
`cargo test --workspace`). Runtime verification lives in the `examples` crate.
`just examples` runs a curated set of 18+ binaries so you can eyeball animation,
audio, input, shaders, fonts, and gui against a real raylib context.

## Bumping raylib

When a new raylib release lands upstream, walk through this checklist on a fresh
branch. The major version of sola-raylib tracks raylib's major (5.x binds 5.5,
6.x binds 6.0), so a major raylib bump is also a major sola-raylib bump — treat
it as a breaking release.

1. **Bump the raylib submodule.**

   ```
   cd raylib-sys/raylib
   git fetch --tags
   git checkout <new-tag>       # e.g. 6.0
   cd ../..
   git add raylib-sys/raylib
   ```

2. **Bump raygui to the matching release.** raygui is vendored, not submoduled,
   so replace `raylib-sys/binding/raygui.h` with the new release from
   [raysan5/raygui](https://github.com/raysan5/raygui) and commit it.

3. **Force a clean rebuild of the sys crate.** bindgen caches aggressively and
   `build.rs` won't always notice a header swap:

   ```
   cargo clean -p sola-raylib-sys
   just build
   ```

4. **Triage the compile errors, module by module.** Expect the break to land in
   `raylib/src/core/` and `raylib/src/rgui/`. Common shapes:
   - A struct field was renamed or its type changed (e.g. `Transform` arrays vs
     single values).
   - A function gained/lost a parameter — update the safe wrapper signature.
   - A function was removed; deprecate or delete the wrapper.
   - An enum variant name changed — sweep `consts.rs`.

5. **Survey new APIs.** Run `python3 scripts/find_unimplemented.py` to list FFI
   functions that don't yet have a safe wrapper. Decide which to add for the
   release vs defer. Helpers covered by Rust's `std` (string manipulation, most
   file I/O) stay unwrapped — add them to `wont_impl` at the top of the script
   so they stop showing up.

6. **Runtime-test every example.** `just examples` is the fast loop. A clean
   compile doesn't mean correctness: skeletal animation, model loading, font
   loading, audio, and gui paths only surface problems when actually run. For
   any new API you wrapped, add or extend an example that exercises it.

7. **Update docs and versions.**
   - `CHANGELOG.md` — add a section for the new version.
   - `README.md` — bump every raylib version reference.
   - Workspace crate versions — see the "Releasing" section above.

8. **Ship.** Follow the Releasing checklist.

## Maintenance scripts

`scripts/find_unimplemented.py` lists raylib and raygui FFI functions that
aren't yet wrapped in the safe layer. Run from the repo root:

```
python3 scripts/find_unimplemented.py
```

Output is a `- [ ]` checklist grouped by `Raylib` / `Raygui`, handy when
tracking wrapping progress against a new upstream release. Functions we never
intend to wrap (std-covered helpers, etc.) live in the `wont_impl` list at the
top of the script. Edit there if a new one should be ignored.
