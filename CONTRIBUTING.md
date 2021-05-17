# Contributing
Contributions are always welcome -- RTP (and its associated extensions) cover decades of work and standardisation, so there's inevitably something missing.
Feel free to start a discussion or open a PR!

## Pull Requests
DiscoRTP has 2 branches:

 * `stable` -- the latest in-development branch featuring bug fixes and non-breaking API changes.
 * `breaking` -- the superset of commits merged to stable with breaking changes. This includes bumping up the minimum supported rust version.

Bugfixes and API-respecting changes (including new functionality which does not change existing user-facing aspects) should target `stable`.
Significant rework (or a necessary compiler upgrade) should target `breaking`.
PRs should avoid bumping the minimum supported rust compiler version where possible, and any changes targeting `stable` **must** compile on the version indicated in [README.md].

### Formatting
PRs *must* pass through `rustfmt` (as described below) before merging.

## Issues
Issues can be used for discussion of bugs, library design, or feature requests.

### Bug Reports
Please include your Rust compiler version (`rustc -V`), the version of DiscoRTP, and a minimal test case if possible.

## Code Style
We use the nightly `rustfmt` to automatically format code, like so:

```sh
cargo +nightly fmt
```

At a high level, this enforces:

 * Unix line endings,
 * 100-char width,
 * Tabs (\t) indentation,
 * Trailing commas in match blocks.

Documentation, comments, and function names should adhere to UK English where possible (*i.e.*, "colour" and not "color").

[README.md]: README.md
