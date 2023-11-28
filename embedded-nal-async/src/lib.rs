//! # embedded-nal-async - An async Network Abstraction Layer for Embedded Systems

#![no_std]
#![cfg_attr(nightly, allow(stable_features, unknown_lints))]
#![cfg_attr(nightly, feature(async_fn_in_trait, impl_trait_projections))]
#![allow(async_fn_in_trait)]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(feature = "ip_in_core", feature(ip_in_core))]

mod dns;
mod stack;

#[cfg(feature = "ip_in_core")]
pub use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};
#[cfg(not(feature = "ip_in_core"))]
pub use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

pub use dns::Dns;
pub use embedded_nal::AddrType;
pub use stack::TcpConnect;
pub use stack::{ConnectedUdp, UdpStack, UnconnectedUdp};
