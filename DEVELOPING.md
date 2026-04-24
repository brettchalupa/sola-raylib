# Developing

Documentation on how to work on Sola.

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
  `git checkout main && git pull && git status`.
- `just ok` is green.

### Steps

1. **Bump the version in a PR.** The version lives in several places — update
   all of them together and add the release notes to
   [CHANGELOG.md](CHANGELOG.md):
   - `raylib-sys/Cargo.toml` — its own `version`
   - `raylib/Cargo.toml` — its own `version` **and** the `version = "X.Y.Z"`
     pinned on the `raylib-sys` dep line (easy to miss)
   - `examples/Cargo.toml` — keep in lockstep though not published
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

The sola-raylib-test crate tests the bindings by opening a window, and checking
the results of various functions. It requires nightly to use.

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
