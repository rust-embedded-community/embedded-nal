# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

No unreleased changes yet

## [0.9.0] - 2025-10-01

- Updated to `embedded-io-async` 0.7

## [0.8.0] - 2024-09-20

- Removed the `ip_in_core` feature, this is now the default.

## [0.7.1] - 2023-11-28

- Use `feature()` on nightly toolchains only. This adds support for 1.75 beta and stable.

## [0.7.0] - 2023-11-10

- [breaking] `Dns::get_host_by_address` now uses `&mut [u8]` instead of `heapless::String`.
- [breaking] Remove unneeded `where Self: 'a` bound in `TcpClient::connect`.
- Bumped to `embedded-nal` 0.8

## [0.6.0] - 2023-10-03

- Bumped to `embedded-io-async` 0.6

## [0.5.0] - 2023-08-07

- Let `&T` for `T: Dns` implement `Dns`
- Bumped to `embedded-nal` 0.7
- Bumped to `embedded-io-async` 0.5

## [0.4.0] - 2023-01-27

- Add traits for UDP

## [0.3.0] - 2022-11-25

- Bump `embedded-io` dependency to `0.4`
- Switch all traits to use [`async_fn_in_trait`](https://blog.rust-lang.org/inside-rust/2022/11/17/async-fn-in-trait-nightly.html) (AFIT). Requires `nightly-2022-11-22` or newer.

## [0.2.0] - 2022-08-03

TcpClient trait for creating shared async TCP/IP stack implementations.
Remove TcpClientStack, TcpFullStack and UDP traits pending traits that support shared use.

## [0.1.0] - 2022-05-04

Initial release to crates.io.

[Unreleased]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.9.0...HEAD
[0.9.0]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.8.0...embedded-nal-async-v0.9.0
[0.8.0]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.7.1...embedded-nal-async-v0.8.0
[0.7.1]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.7.0...embedded-nal-async-v0.7.1
[0.7.0]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.6.0...embedded-nal-async-v0.7.0
[0.6.0]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.5.0...embedded-nal-async-v0.6.0
[0.5.0]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.4.0...embedded-nal-async-v0.5.0
[0.4.0]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.3.0...embedded-nal-async-v0.4.0
[0.1.0]: https://github.com/rust-embedded-community/embedded-nal/releases/tag/embedded-nal-async-v0.1.0
