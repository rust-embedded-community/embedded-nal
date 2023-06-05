# Change Log

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

- Let `&T` for `T: Dns` implement `Dns`

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

[Unreleased]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.4.0...HEAD
[0.4.0]: https://github.com/rust-embedded-community/embedded-nal/compare/embedded-nal-async-v0.3.0...embedded-nal-async-v0.4.0
[0.1.0]: https://github.com/rust-embedded-community/embedded-nal/releases/tag/embedded-nal-async-v0.1.0
