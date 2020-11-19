//! # embedded-nal - A Network Abstraction Layer for Embedded Systems

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub use nb;
mod dns;
pub use dns::{AddrType, Dns};

pub use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

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
	fn open(&self) -> Result<Self::TcpSocket, Self::Error>;

	/// Connect to the given remote host and port.
	fn connect(
		&self,
		socket: Self::TcpSocket,
		remote: SocketAddr,
	) -> Result<Self::TcpSocket, Self::Error>;

	/// Check if this socket is connected
	fn is_connected(&self, socket: &Self::TcpSocket) -> Result<bool, Self::Error>;

	/// Write to the stream. Returns the number of bytes written is returned
	/// (which may be less than `buffer.len()`), or an error.
	fn write(&self, socket: &mut Self::TcpSocket, buffer: &[u8]) -> nb::Result<usize, Self::Error>;

	/// Read from the stream.
	///
	/// Returns `Ok(n)`, which means `n` bytes of data have been received and
	/// they have been placed in `&buffer[0..n]`, or an error. If a packet has
	/// not been received when called, then
	/// [`nb::Error::WouldBlock`](https://docs.rs/nb/1.0.0/nb/enum.Error.html#variant.WouldBlock)
	/// should be returned.
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
pub trait UdpClient {
	/// The type returned when we create a new UDP socket
	type UdpSocket;
	/// The type returned when we have an error
	type Error: core::fmt::Debug;

	/// Open a UDP socket with a dynamically selected port.
	///
	/// Selects a port number automatically and initializes for read/writing.
	fn connect(&self, remote: SocketAddr) -> Result<Self::UdpSocket, Self::Error>;

	/// Send a datagram to the remote host.
	///
	/// The remote host used is either the one specified in `UdpStack::connect`
	/// or the last one used in `UdpServerStack::write_to`.
	fn send(&self, socket: &mut Self::UdpSocket, buffer: &[u8]) -> nb::Result<(), Self::Error>;

	/// Read a datagram the remote host has sent to us.
	///
	/// Returns `Ok((n, remote))`, which means a datagram of size `n` has been
	/// received from `remote` and been placed in `&buffer[0..n]`, or an error.
	/// If a packet has not been received when called, then
	/// [`nb::Error::WouldBlock`](https://docs.rs/nb/1.0.0/nb/enum.Error.html#variant.WouldBlock)
	/// should be returned.
	fn receive(
		&self,
		socket: &mut Self::UdpSocket,
		buffer: &mut [u8],
	) -> nb::Result<(usize, SocketAddr), Self::Error>;

	/// Close an existing UDP socket.
	fn close(&self, socket: Self::UdpSocket) -> Result<(), Self::Error>;
}

/// This trait is implemented by UDP/IP stacks.  It provides the ability to
/// listen for packets on a specified port and send replies.
pub trait UdpServer: UdpClient {
	/// Open a new UDP socket with a specified port
	///
	/// Opens a new socket with the specified port number to the given address.
	fn bind(&self, local_port: u16) -> Result<Self::UdpSocket, Self::Error>;

	/// Send a packet to a remote host/port.
	fn send_to(
		&self,
		socket: &mut Self::UdpSocket,
		remote: SocketAddr,
		buffer: &[u8],
	) -> nb::Result<(), Self::Error>;
}

// End Of File
