//! # embedded-nal - A Network Abstraction Layer for Embedded Systems
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(feature = "ip_in_core", feature(ip_in_core))]

mod dns;
mod stack;

pub use nb;
// Needed by embedded-nal trait implementers who build get_host_by_address results, or by trait
// users who pass the results on.
pub use heapless;

#[cfg(not(any(feature = "ip_in_core", feature = "no-std-net")))]
compile_error!("You must select the ip_in_core feature or the no-std-net feature");

#[cfg(feature = "ip_in_core")]
pub use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
#[cfg(not(feature = "ip_in_core"))]
pub use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub use dns::{AddrType, Dns};
pub use stack::{
	SharableStack, SharedStack, TcpClientStack, TcpError, TcpErrorKind, TcpFullStack,
	UdpClientStack, UdpFullStack,
};
