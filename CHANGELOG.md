# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]
...

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

[Unreleased]: https://github.com/rust-embedded-community/embedded-nal/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/rust-embedded-community/embedded-nal/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/rust-embedded-community/embedded-nal/releases/tag/v0.1.0
