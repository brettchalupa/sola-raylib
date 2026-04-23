# Developing

Documentation on how to work on Sola.

## Commands

[just](https://just.systems/man/en/) is used for running commands while working
on the project. See `./justfile` for available commands.

`just ok` is a helpful command to run regularly and ensure is passing.

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
   - `examples/Cargo.toml` and `showcase/Cargo.toml` — keep in lockstep even
     though they aren't published
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
