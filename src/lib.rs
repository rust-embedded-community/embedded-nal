//! # embedded-nal - A Network Abstraction Layer for Embedded Systems

#![doc(html_root_url = "https://docs.rs/embedded-nal/0.3.0")]
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod dns;
mod stack;

pub use nb;
// Needed by embedded-nal trait implementers who build get_host_by_address results, or by trait
// users who pass the results on.
pub use heapless;
pub use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub use dns::{AddrType, Dns};
pub use stack::{
	SharableStack, SharedStack, TcpClientStack, TcpFullStack, UdpClientStack, UdpFullStack,
};
