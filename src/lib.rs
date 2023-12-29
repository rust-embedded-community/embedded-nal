//! # embedded-nal - A Network Abstraction Layer for Embedded Systems
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod dns;
mod stack;

pub use nb;

pub use dns::{AddrType, Dns};
pub use stack::{
	SharableStack, SharedStack, TcpClientStack, TcpError, TcpErrorKind, TcpFullStack,
	UdpClientStack, UdpFullStack,
};
