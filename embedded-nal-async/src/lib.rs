//! # embedded-nal-async - An async Network Abstraction Layer for Embedded Systems

#![doc(html_root_url = "https://docs.rs/embedded-nal-async/0.1.0")]
#![no_std]
#![feature(generic_associated_types)]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod dns;
mod stack;
//
//pub use nb;
// Needed by embedded-nal trait implementers who build get_host_by_address results, or by trait
// users who pass the results on.
pub use heapless;
pub use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub use dns::Dns;
pub use embedded_nal::AddrType;
pub use stack::{TcpClientStack, TcpFullStack, UdpClientStack, UdpFullStack};
