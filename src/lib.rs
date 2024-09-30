//! # embedded-nal - A Network Abstraction Layer for Embedded Systems
#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

mod stack;

use core::net::IpAddr;
pub use nb;

pub use stack::{
	SharableStack, SharedStack, TcpClientStack, TcpFullStack, UdpClientStack, UdpFullStack,
};

/// This is the host address type to be returned by `gethostbyname`.
///
/// An IPv4 address type always looks for `A` records, while IPv6 address type
/// will look for `AAAA` records
#[derive(Clone, Debug, PartialEq)]
pub enum AddrType {
	/// Result is `A` record
	IPv4,
	/// Result is `AAAA` record
	IPv6,
	/// Result is either a `A` record, or a `AAAA` record
	Either,
}

/// Represents specific errors encountered during TCP operations.
#[non_exhaustive]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum ErrorKind {
	/// The socket has been closed in the direction in which the failing operation was attempted.
	PipeClosed,

	/// The operation requested is not supported.
	Unsupported,

	/// Some other error has occurred.
	Other,
}

/// Marker trait for errors that can be resolved to predefined categorical types.
pub trait Error: core::fmt::Debug {
	/// Determine the type of error that occurred.
	fn kind(&self) -> ErrorKind;
}

/// This trait is an extension trait for [`TcpStack`] and [`UdpStack`] for dns
/// resolutions. It does not handle every DNS record type, but is meant as an
/// embedded alternative to [`ToSocketAddrs`], and is as such meant to resolve
/// an ip address from a hostname, or a hostname from an ip address. This means
/// that it only deals in host address records `A` (IPv4) and `AAAA` (IPv6).
///
/// [`TcpStack`]: crate::trait@TcpStack
/// [`UdpStack`]: crate::trait@UdpStack
/// [`ToSocketAddrs`]:
/// https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html
pub trait NetworkStack {
	/// The type returned when we have an error
	type Error: Error;

	/// Resolve the first ip address of a host, given its hostname and a desired
	/// address record type to look for
	fn get_host_by_name(
		&mut self,
		hostname: &str,
		addr_type: AddrType,
	) -> nb::Result<IpAddr, Self::Error>;

	/// Resolve the hostname of a host, given its ip address.
	///
	/// The hostname is stored at the beginning of `result`, the length is returned.
	///
	/// If the buffer is too small to hold the domain name, an error should be returned.
	///
	/// **Note**: A fully qualified domain name (FQDN), has a maximum length of
	/// 255 bytes according to [`rfc1035`]. Therefore, you can pass a 255-byte long
	/// buffer to guarantee it'll always be large enough.
	///
	/// [`rfc1035`]: https://tools.ietf.org/html/rfc1035
	fn get_host_by_address(
		&mut self,
		addr: IpAddr,
		result: &mut [u8],
	) -> nb::Result<usize, Self::Error>;
}

impl<T: NetworkStack> NetworkStack for &mut T {
	type Error = T::Error;

	fn get_host_by_name(
		&mut self,
		hostname: &str,
		addr_type: AddrType,
	) -> nb::Result<IpAddr, Self::Error> {
		T::get_host_by_name(self, hostname, addr_type)
	}

	fn get_host_by_address(
		&mut self,
		addr: IpAddr,
		result: &mut [u8],
	) -> nb::Result<usize, Self::Error> {
		T::get_host_by_address(self, addr, result)
	}
}
