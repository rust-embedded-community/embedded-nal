# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## Unreleased

* Add blanket impls of all the traits for mutable references.
- Bump dependency version of `no-std-net` to `v0.6`.

## [0.6.0] - 2021-05-25

- Changed self references in dns stack methods to mutable, to follow the network stack implementations.

## [0.5.0] - 2021-05-20

### Changed

- Bump dependency version of `heapless` to `v0.7.0` to utilize const generics.
- Bump MSRV to 1.51.0 to get `min_const_generics` for `heapless`.

## [0.4.0] - 2021-03-05

### Changed
- Changed [`Dns`](./src/dns.rs) methods to return `nb::Result<..>` to allow non-blocking implementations.
- Bump dependency version of `heapless` to `v0.6.1` to address security issue of sub-dependency.
- Bump dependency version of `no-std-net` to `v0.5`.
- Bump MSRV to 1.46.0 to get `const-fn` for `no-std-net`.


## [0.3.0] - 2021-02-15

### Added
- New optional struct [`SharedNal`](./src/stack/share.rs) that can share a single underlying implementation among several users within a thread.

### Changed
- Changed the names of `UdpClient`/`TcpClient` to `UdpClientStack`/`TcpClientStack`
- Changed the names of `UdpServer`/`TcpServer` to `UdpFullStack`/`TcpFullStack`
- Changed the method names `Dns::gethostbyname`/`Dns::gethostbyaddr` to `Dns::get_host_by_name`/`Dns::get_host_by_address`
- Changed self references in all network stack methods to mutable, with the intent of handling sharing in a different layer (see [#43](https://github.com/rust-embedded-community/embedded-nal/issues/43)).

## [0.2.0] - 2020-12-02

### Added
- Added a new `UdpServer` trait with server-specific methods
- Added a new `TcpServer` trait with server-specific methods

### Changed
- Changed the `UdpStack::receive` method to return the packet sender address, along with the packet length
- Changed the name of `UdpStack` to `UdpClient`
- Changed name of `TcpStack` to `TcpClient`
- Changed the `TcpCStack::connect()` function to return an `nb::Result`
- Renamed `open()` functions to `socket()` for both stacks
- Renamed `read()` and `write()` functions to `send()` and `receive()` respectively
- Updated `UdpStack::connect()` to modify an existing socket

### Removed
- Removed `Mode` enum, implementations should instead use `nb::WouldBlock`

## [0.1.0] - 2020-08-26

Initial release to crates.io.

[Unreleased]: https://github.com/rust-embedded-community/embedded-nal/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/rust-embedded-community/embedded-nal/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/rust-embedded-community/embedded-nal/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/rust-embedded-community/embedded-nal/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/rust-embedded-community/embedded-nal/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/rust-embedded-community/embedded-nal/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/rust-embedded-community/embedded-nal/releases/tag/v0.1.0
