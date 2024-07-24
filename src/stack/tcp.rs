use crate::NetworkStack;
use core::net::SocketAddr;

/// This trait is implemented by TCP/IP stacks. You could, for example, have an implementation
/// which knows how to send AT commands to an ESP8266 WiFi module. You could have another implementation
/// which knows how to driver the Rust Standard Library's `std::net` module. Given this trait, you can
/// write a portable HTTP client which can work with either implementation.
pub trait TcpClientStack: NetworkStack {
	/// The type returned when we create a new TCP socket
	type TcpSocket;

	/// Open a socket for usage as a TCP client.
	///
	/// The socket must be connected before it can be used.
	///
	/// Returns `Ok(socket)` if the socket was successfully created.
	fn socket(&mut self) -> Result<Self::TcpSocket, Self::Error>;

	/// Connect to the given remote host and port.
	///
	/// Returns `Ok` if the connection was successful. Otherwise, if the connection could not be
	/// completed immediately, this function should return [`nb::Error::WouldBlock`].
	fn connect(
		&mut self,
		socket: &mut Self::TcpSocket,
		remote: SocketAddr,
	) -> nb::Result<(), Self::Error>;

	/// Write to the stream.
	///
	/// Returns the number of bytes written (which may be less than `buffer.len()`) or an error.
	fn send(
		&mut self,
		socket: &mut Self::TcpSocket,
		buffer: &[u8],
	) -> nb::Result<usize, Self::Error>;

	/// Receive data from the stream.
	///
	/// Returns `Ok(n)`, which means `n` bytes of data have been received and
	/// they have been placed in `&buffer[0..n]`, or an error. If a packet has
	/// not been received when called, then [`nb::Error::WouldBlock`]
	/// should be returned.
	fn receive(
		&mut self,
		socket: &mut Self::TcpSocket,
		buffer: &mut [u8],
	) -> nb::Result<usize, Self::Error>;

	/// Close an existing TCP socket.
	fn close(&mut self, socket: Self::TcpSocket) -> Result<(), Self::Error>;
}

/// This trait is implemented by TCP/IP stacks that expose TCP server functionality. TCP servers
/// may listen for connection requests to establish multiple unique TCP connections with various
/// clients.
pub trait TcpFullStack: TcpClientStack {
	/// Create a new TCP socket and bind it to the specified local port.
	///
	/// Returns `Ok` when a socket is successfully bound to the specified local port. Otherwise, an
	/// `Err(e)` variant is returned.
	fn bind(&mut self, socket: &mut Self::TcpSocket, local_port: u16) -> Result<(), Self::Error>;

	/// Begin listening for connection requests on a previously-bound socket.
	///
	/// Returns `Ok` if the socket was successfully transitioned to the listening state. Otherwise,
	/// an `Err(e)` variant is returned.
	fn listen(&mut self, socket: &mut Self::TcpSocket) -> Result<(), Self::Error>;

	/// Accept an active connection request on a listening socket.
	///
	/// Returns `Ok(connection)` if a new connection was created. If no pending connections are
	/// available, this function should return [`nb::Error::WouldBlock`].
	fn accept(
		&mut self,
		socket: &mut Self::TcpSocket,
	) -> nb::Result<(Self::TcpSocket, SocketAddr), Self::Error>;
}

impl<T: TcpClientStack> TcpClientStack for &mut T {
	type TcpSocket = T::TcpSocket;

	fn socket(&mut self) -> Result<Self::TcpSocket, Self::Error> {
		T::socket(self)
	}

	fn connect(
		&mut self,
		socket: &mut Self::TcpSocket,
		remote: SocketAddr,
	) -> nb::Result<(), Self::Error> {
		T::connect(self, socket, remote)
	}

	fn send(
		&mut self,
		socket: &mut Self::TcpSocket,
		buffer: &[u8],
	) -> nb::Result<usize, Self::Error> {
		T::send(self, socket, buffer)
	}

	fn receive(
		&mut self,
		socket: &mut Self::TcpSocket,
		buffer: &mut [u8],
	) -> nb::Result<usize, Self::Error> {
		T::receive(self, socket, buffer)
	}

	fn close(&mut self, socket: Self::TcpSocket) -> Result<(), Self::Error> {
		T::close(self, socket)
	}
}
