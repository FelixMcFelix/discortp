# Changelog
Notable updates will be documented here, adhering to [Semantic Versioning][semver].

## [0.6.0] - 2023-11-17
* Update `libpnet` version to 0.34.
* MSRV is now Rust 1.65.

## [0.5.0] - 2022-06-10
* Update `libpnet` version to 0.31.
* Crate is now `#![no_std]`.
* Source is cleaned up to meet clippy's pedantic setting.
* MSRV is now Rust 1.56.1+.

## [0.4.0] - 2021-05-17
* Update `libpnet` version to 0.28.
* All packet definitions have been converted to proc-macros.
* MSRV is now Rust 1.42.0+.

## [0.3.0] - 2021-03-22
* Update `libpnet` version to 0.27.
* Make remaining (incomplete) packet variants non-exhaustive to allow non-breaking addition.
* Add `RtcpType` variants to simplify decoding of (as-yet unsupported) RTCP types without using the demux utilities.

## [0.2.2] - 2021-02-13
* Added RTCP packet view creation methods.

## [0.2.1] - 2020-06-04
* Repaired dead links in documentation.

## [0.2.0] - 2020-06-03
* Large-scale documentation cleanup.
* RTCP support for Sender/Receiver blocks.
* Demux'ing improvements.
* Discord Keepalives.

## [0.1.1] - 2020-05-04
* Unbreak docs.rs.

## [0.1.0] - 2020-05-04
* Initial version.

[0.6.0]: https://github.com/FelixMcFelix/discortp/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/FelixMcFelix/discortp/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/FelixMcFelix/discortp/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/FelixMcFelix/discortp/compare/v0.2.2...v0.3.0
[0.2.2]: https://github.com/FelixMcFelix/discortp/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/FelixMcFelix/discortp/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/FelixMcFelix/discortp/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/FelixMcFelix/discortp/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/FelixMcFelix/discortp/commit/66fa4a78be2842ff16700c341df9526affa6c7e5

[semver]: http://semver.org