//! # embedded-nal - A Network Abstraction Layer for Embedded Systems

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

/// Represents an IPv4 address (e.g. `127.0.0.1`)
#[derive(Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Ipv4Address(u32);

/// Represents an IPv6 address (e.g. `2001:0db8:85a3:0000:0000:8a2e:0370:7334`)
#[derive(Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Ipv6Address {
	group: [u16; 8],
}

/// Represents a TCP or UDP port number
#[derive(Debug, Copy, Clone, Hash, PartialEq, PartialOrd, Eq, Ord)]
pub struct Port(u16);

/// Whether a socket should block when a read/write can't be performed, or return early.
pub enum Mode {
	/// The function call will wait as long as necessary to complete the operation
	Blocking,
	/// The function call will not wait at all to complete the operation, and only do what it can.
	NonBlocking,
	/// The function call will wait only up the given number of milliseconds to complete the operation.
	Timeout(u16),
}

/// Represents an Internet Protocol address, in either V4 or V6 formats.
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum IpAddress {
	/// This is an IPV4 address
	IpV4(Ipv4Address),
	/// This is an IPV6 address
	IpV6(Ipv6Address),
}

/// This trait is implemented by TCP/IP stacks. You could, for example, have an implementation
/// which knows how to send AT commands to an ESP8266 WiFi module. You could have another implemenation
/// which knows how to driver the Rust Standard Library's `std::net` module. Given this trait, you can how
/// write a portable HTTP client which can work with either implementation.
pub trait TcpStack {
	/// The type returned when we create a new TCP socket
	type TcpSocket;
	/// The type returned when we have an error
	type Error: core::fmt::Debug;

	/// Open a new TCP socket. The socket starts in the unconnected state.
	fn open(&self, mode: Mode) -> Result<Self::TcpSocket, Self::Error>;

	/// Connect to the given remote host and port.
	fn connect(&self, host: IpAddress, port: Port) -> Result<Self::TcpSocket, Self::Error>;

	/// Check if this socket is connected
	fn is_connected(&self, socket: &Self::TcpSocket) -> Result<bool, Self::Error>;

	/// Write to the stream. Returns the number of bytes written is returned
	/// (which may be less than `buffer.len()`), or an error.
	fn write(&self, socket: &mut Self::TcpSocket, buffer: &[u8]) -> nb::Result<usize, Self::Error>;

	/// Read from the stream. Returns `Ok(n)`, which means `n` bytes of
	/// data have been received and they have been placed in
	/// `&buffer[0..n]`, or an error.
	fn read(
		&self,
		socket: &mut Self::TcpSocket,
		buffer: &mut [u8],
	) -> nb::Result<usize, Self::Error>;

	/// Close an existing TCP socket.
	fn close(&self, socket: Self::TcpSocket) -> Result<(), Self::Error>;
}

/// This trait is implemented by UDP/IP stacks. You could, for example, have
/// an implementation which knows how to send AT commands to an ESP8266 WiFi
/// module. You could have another implemenation which knows how to driver the
/// Rust Standard Library's `std::net` module. Given this trait, you can how
/// write a portable CoAP client which can work with either implementation.
pub trait UdpStack {
	/// The type returned when we create a new UDP socket
	type UdpSocket;
	/// The type returned when we have an error
	type Error: core::fmt::Debug;

	/// Open a new UDP socket to the given address and port. UDP is connectionless,
	/// so unlike `TcpStack` no `connect()` is required.
	fn open(&self, addr: IpAddress, port: Port, mode: Mode)
		-> Result<Self::UdpSocket, Self::Error>;

	/// Send a datagram to the remote host.
	fn write(&self, socket: &mut Self::UdpSocket, buffer: &[u8]) -> nb::Result<(), Self::Error>;

	/// Read a datagram the remote host has sent to us. Returns `Ok(n)`, which
	/// means a datagram of size `n` has been received and it has been placed
	/// in `&buffer[0..n]`, or an error.
	fn read(
		&self,
		socket: &mut Self::UdpSocket,
		buffer: &mut [u8],
	) -> nb::Result<usize, Self::Error>;

	/// Close an existing UDP socket.
	fn close(&self, socket: Self::UdpSocket) -> Result<(), Self::Error>;
}

impl core::fmt::Debug for Ipv4Address {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "Ipv4Address({})", self)
	}
}

impl core::fmt::Display for Ipv4Address {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let bytes = self.0.to_be_bytes();
		write!(f, "{}.{}.{}.{}", bytes[0], bytes[1], bytes[2], bytes[3])
	}
}

impl core::fmt::Debug for Ipv6Address {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "Ipv6Address({})", self)
	}
}

impl core::fmt::Display for Ipv6Address {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		write!(f, "{:04x}", self.group[0])?;
		for g in self.group.iter().skip(1) {
			write!(f, "::{:04x}", g)?;
		}
		Ok(())
	}
}

// End Of File
