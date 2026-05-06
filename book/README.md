# sola-raylib book

[mdBook](https://rust-lang.github.io/mdBook/) source for sola-raylib's long-form
docs. Chapters live in `src/`, listed in `src/SUMMARY.md`. Add one by dropping a
`.md` under `src/` and adding a line to `SUMMARY.md`.

## Local workflow

```sh
just setup        # installs mdbook (one-time)
just serve-book   # http://localhost:3030, hot reload
just build-book   # render to ./book/book/
```

`book/book/` is git-ignored; the markdown is the canonical source.

## Why a book and not a GitHub wiki

Wikis live in a separate repo, drift from the code, aren't indexed by search
engines, aren't picked up by PR review, and don't show in `git log`. In-tree
docs land in the same commit as the code change they describe.
