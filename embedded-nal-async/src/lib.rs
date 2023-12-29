//! # embedded-nal-async - An async Network Abstraction Layer for Embedded Systems

#![no_std]
#![allow(async_fn_in_trait)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod dns;
mod stack;

pub use dns::Dns;
pub use embedded_nal::AddrType;
pub use stack::TcpConnect;
pub use stack::{ConnectedUdp, UdpStack, UnconnectedUdp};
