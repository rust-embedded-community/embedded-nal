//! # embedded-nal - A Network Abstraction Layer for Embedded Systems

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub use nb;
mod dns;
pub use dns::{AddrType, Dns};

pub use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

/// This trait is implemented by TCP/IP stacks. You could, for example, have an implementation
/// which knows how to send AT commands to an ESP8266 WiFi module. You could have another implementation
/// which knows how to driver the Rust Standard Library's `std::net` module. Given this trait, you can how
/// write a portable HTTP client which can work with either implementation.
pub trait TcpClient {
	/// The type returned when we create a new TCP socket
	type TcpSocket;
	/// The type returned when we have an error
	type Error: core::fmt::Debug;

	/// Open a socket for usage as a TCP client.
	///
	/// The socket must be connected before it can be used.
	///
	/// Returns `Ok(socket)` if the socket was successfully created.
	fn socket(&self) -> Result<Self::TcpSocket, Self::Error>;

	/// Connect to the given remote host and port.
	///
	/// Returns `Ok` if the connection was successful. Otherwise, if the connection could not be
	/// completed immediately, this function should return [`nb::Error::WouldBlock`].
	fn connect(
		&self,
		socket: &mut Self::TcpSocket,
		remote: SocketAddr,
	) -> nb::Result<(), Self::Error>;

	/// Check if this socket is connected
	fn is_connected(&self, socket: &Self::TcpSocket) -> Result<bool, Self::Error>;

	/// Write to the stream.
	///
	/// Returns the number of bytes written (which may be less than `buffer.len()`) or an error.
	fn send(&self, socket: &mut Self::TcpSocket, buffer: &[u8]) -> nb::Result<usize, Self::Error>;

	/// Receive data from the stream.
	///
	/// Returns `Ok(n)`, which means `n` bytes of data have been received and
	/// they have been placed in `&buffer[0..n]`, or an error. If a packet has
	/// not been received when called, then [`nb::Error::WouldBlock`]
	/// should be returned.
	fn receive(
		&self,
		socket: &mut Self::TcpSocket,
		buffer: &mut [u8],
	) -> nb::Result<usize, Self::Error>;

	/// Close an existing TCP socket.
	fn close(&self, socket: Self::TcpSocket) -> Result<(), Self::Error>;
}

/// This trait is implemented by TCP/IP stacks that expose TCP server functionality. TCP servers
/// may listen for connection requests to establish multiple unique TCP connections with various
/// clients.
pub trait TcpServer: TcpClient {
	/// Create a new TCP socket and bind it to the specified local port.
	///
	/// Returns `Ok` when a socket is successfully bound to the specified local port. Otherwise, an
	/// `Err(e)` variant is returned.
	fn bind(&self, socket: &mut Self::TcpSocket, local_port: u16) -> Result<(), Self::Error>;

	/// Begin listening for connection requests on a previously-bound socket.
	///
	/// Returns `Ok` if the socket was successfully transitioned to the listening state. Otherwise,
	/// an `Err(e)` variant is returned.
	fn listen(&self, socket: &mut Self::TcpSocket) -> Result<(), Self::Error>;

	/// Accept an active connection request on a listening socket.
	///
	/// Returns `Ok(connection)` if a new connection was created. If no pending connections are
	/// available, this function should return [`nb::Error::WouldBlock`].
	fn accept(
		&self,
		socket: &mut Self::TcpSocket,
	) -> nb::Result<(Self::TcpSocket, SocketAddr), Self::Error>;
}

/// This trait is implemented by UDP/IP stacks. You could, for example, have
/// an implementation which knows how to send AT commands to an ESP8266 WiFi
/// module. You could have another implementation which knows how to driver the
/// Rust Standard Library's `std::net` module. Given this trait, you can how
/// write a portable CoAP client which can work with either implementation.
pub trait UdpClient {
	/// The type returned when we create a new UDP socket
	type UdpSocket;
	/// The type returned when we have an error
	type Error: core::fmt::Debug;

	/// Allocate a socket for further use.
	fn socket(&self) -> Result<Self::UdpSocket, Self::Error>;

	/// Connect a UDP socket with a peer using a dynamically selected port.
	///
	/// Selects a port number automatically and initializes for read/writing.
	fn connect(&self, socket: &mut Self::UdpSocket, remote: SocketAddr) -> Result<(), Self::Error>;

	/// Send a datagram to the remote host.
	///
	/// The remote host used is either the one specified in `UdpStack::connect`
	/// or the last one used in `UdpServerStack::write_to`.
	fn send(&self, socket: &mut Self::UdpSocket, buffer: &[u8]) -> nb::Result<(), Self::Error>;

	/// Read a datagram the remote host has sent to us.
	///
	/// Returns `Ok((n, remote))`, which means a datagram of size `n` has been
	/// received from `remote` and been placed in `&buffer[0..n]`, or an error.
	/// If a packet has not been received when called, then [`nb::Error::WouldBlock`]
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
	/// Bind a UDP socket with a specified port
	fn bind(&self, socket: &mut Self::UdpSocket, local_port: u16) -> Result<(), Self::Error>;

	/// Send a packet to a remote host/port.
	fn send_to(
		&self,
		socket: &mut Self::UdpSocket,
		remote: SocketAddr,
		buffer: &[u8],
	) -> nb::Result<(), Self::Error>;
}

// End Of File
