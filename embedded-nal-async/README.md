<!--
[![crates.io](https://img.shields.io/crates/d/embedded-nal-async.svg)](https://crates.io/crates/embedded-nal-async)
[![crates.io](https://img.shields.io/crates/v/embedded-nal-async.svg)](https://crates.io/crates/embedded-nal-async)
[![Documentation](https://docs.rs/embedded-nal-async/badge.svg)](https://docs.rs/embedded-nal-async)
-->

# `embedded-nal-async`

An asynchronous Nardware Abstraction Layer (NAL) for embedded systems.

This crate contains asynchronous versions of the [`embedded-nal`] traits and shares its scope and [design goals].
The purpose of this crate is to iterate over these trait versions before integrating them into [`embedded-nal`].

**NOTE** These traits are still experimental. At least one breaking change to this crate is expected in the future (changing from GATs to `async fn`), but there might be more.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
