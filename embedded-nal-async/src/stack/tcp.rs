use core::future::Future;
use no_std_net::SocketAddr;

/// This trait is implemented by TCP/IP stacks. You could, for example, have an implementation
/// which knows how to send AT commands to an ESP8266 WiFi module. You could have another implementation
/// which knows how to driver the Rust Standard Library's `std::net` module. Given this trait, you can
/// write a portable HTTP client which can work with either implementation.
pub trait TcpClientStack {
	/// The type returned when we create a new TCP socket
	type TcpSocket;
	/// The type returned when we have an error
	type Error: core::fmt::Debug;

	/// Future returned by `socket` function.
	type SocketFuture<'m>: Future<Output = Result<Self::TcpSocket, Self::Error>> + 'm
	where
		Self: 'm;

	/// Open a socket for usage as a TCP client.
	///
	/// The socket must be connected before it can be used.
	///
	/// Returns `Ok(socket)` if the socket was successfully created.
	fn socket<'m>(&'m mut self) -> Self::SocketFuture<'m>;

	/// Future returned by `connect` function.
	type ConnectFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;

	/// Connect to the given remote host and port.
	///
	/// Returns `Ok` if the connection was successful.
	fn connect<'m>(
		&'m mut self,
		socket: &'m mut Self::TcpSocket,
		remote: SocketAddr,
	) -> Self::ConnectFuture<'m>;

	/// Future returned by `is_connected` function.
	type IsConnectedFuture<'m>: Future<Output = Result<bool, Self::Error>> + 'm
	where
		Self: 'm;

	/// Check if this socket is connected
	fn is_connected<'m>(&'m mut self, socket: &'m Self::TcpSocket) -> Self::IsConnectedFuture<'m>;

	/// Future returned by `send` function.
	type SendFuture<'m>: Future<Output = Result<usize, Self::Error>> + 'm
	where
		Self: 'm;

	/// Write to the stream.
	///
	/// Returns the number of bytes written (which may be less than `buffer.len()`) or an error.
	fn send<'m>(
		&'m mut self,
		socket: &'m mut Self::TcpSocket,
		buffer: &'m [u8],
	) -> Self::SendFuture<'m>;

	/// Future returned by `receive` function.
	type ReceiveFuture<'m>: Future<Output = Result<usize, Self::Error>> + 'm
	where
		Self: 'm;

	/// Receive data from the stream.
	///
	/// Returns `Ok(n)`, which means `n` bytes of data have been received and
	/// they have been placed in `&buffer[0..n]`, or an error.
	fn receive<'m>(
		&'m mut self,
		socket: &'m mut Self::TcpSocket,
		buffer: &'m mut [u8],
	) -> Self::ReceiveFuture<'m>;

	/// Future returned by `close` function.
	type CloseFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;

	/// Close an existing TCP socket.
	fn close<'m>(&'m mut self, socket: Self::TcpSocket) -> Self::CloseFuture<'m>;
}

/// This trait is implemented by TCP/IP stacks that expose TCP server functionality. TCP servers
/// may listen for connection requests to establish multiple unique TCP connections with various
/// clients.
pub trait TcpFullStack: TcpClientStack {
	/// Future returned by `bind` function.
	type BindFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;
	/// Create a new TCP socket and bind it to the specified local port.
	///
	/// Returns `Ok` when a socket is successfully bound to the specified local port. Otherwise, an
	/// `Err(e)` variant is returned.
	fn bind<'m>(
		&'m mut self,
		socket: &'m mut Self::TcpSocket,
		local_port: u16,
	) -> Self::BindFuture<'m>;

	/// Future returned by `listen` function.
	type ListenFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;

	/// Begin listening for connection requests on a previously-bound socket.
	///
	/// Returns `Ok` if the socket was successfully transitioned to the listening state. Otherwise,
	/// an `Err(e)` variant is returned.
	fn listen<'m>(&'m mut self, socket: &'m mut Self::TcpSocket) -> Self::ListenFuture<'m>;

	/// Future returned by `accept` function.
	type AcceptFuture<'m>: Future<Output = Result<(Self::TcpSocket, SocketAddr), Self::Error>> + 'm
	where
		Self: 'm;

	/// Accept an active connection request on a listening socket.
	///
	/// Returns `Ok(connection)` if a new connection was created. If no pending connections are
	/// available, this function should return [`nb::Error::WouldBlock`].
	fn accept<'m>(&'m mut self, socket: &'m mut Self::TcpSocket) -> Self::AcceptFuture<'m>;
}

impl<T: TcpClientStack> TcpClientStack for &mut T {
	type Error = T::Error;
	type TcpSocket = T::TcpSocket;

	type SocketFuture<'m> = T::SocketFuture<'m> where Self: 'm;
	fn socket<'m>(&'m mut self) -> Self::SocketFuture<'m> {
		T::socket(self)
	}

	type ConnectFuture<'m> = T::ConnectFuture<'m> where Self: 'm;
	fn connect<'m>(
		&'m mut self,
		socket: &'m mut Self::TcpSocket,
		remote: SocketAddr,
	) -> Self::ConnectFuture<'m> {
		T::connect(self, socket, remote)
	}

	type IsConnectedFuture<'m> = T::IsConnectedFuture<'m> where Self: 'm;
	fn is_connected<'m>(&'m mut self, socket: &'m Self::TcpSocket) -> Self::IsConnectedFuture<'m> {
		T::is_connected(self, socket)
	}

	type SendFuture<'m> = T::SendFuture<'m> where Self: 'm;
	fn send<'m>(
		&'m mut self,
		socket: &'m mut Self::TcpSocket,
		buffer: &'m [u8],
	) -> Self::SendFuture<'m> {
		T::send(self, socket, buffer)
	}

	type ReceiveFuture<'m> = T::ReceiveFuture<'m> where Self: 'm;
	fn receive<'m>(
		&'m mut self,
		socket: &'m mut Self::TcpSocket,
		buffer: &'m mut [u8],
	) -> Self::ReceiveFuture<'m> {
		T::receive(self, socket, buffer)
	}

	type CloseFuture<'m> = T::CloseFuture<'m> where Self: 'm;
	fn close<'m>(&'m mut self, socket: Self::TcpSocket) -> Self::CloseFuture<'m> {
		T::close(self, socket)
	}
}

/// This trait is implemented by TCP/IP stacks. The trait allows the underlying driver to
/// construct multiple connections that implement the I/O traits from embedded-io.
///
/// The associated connection type should close the connection when dropped.
pub trait TcpConnect {
	/// Error type returned on connect failure.
	type Error: embedded_io::Error;

	/// Type holding state of a TCP connection. Should close the connection when dropped.
	type Connection<'m>: embedded_io::asynch::Read<Error = Self::Error>
		+ embedded_io::asynch::Write<Error = Self::Error>
	where
		Self: 'm;
	/// Future returned by `connect` function.
	type ConnectFuture<'m>: Future<Output = Result<Self::Connection<'m>, Self::Error>> + 'm
	where
		Self: 'm;

	/// Connect to the given remote host and port.
	///
	/// Returns `Ok` if the connection was successful.
	fn connect<'m>(&'m self, remote: SocketAddr) -> Self::ConnectFuture<'m>;
}

impl<T: TcpConnect> TcpConnect for &T {
	type Error = T::Error;

	type Connection<'m> = T::Connection<'m> where Self: 'm;

	type ConnectFuture<'m> = T::ConnectFuture<'m> where Self: 'm;

	fn connect<'m>(&'m self, remote: SocketAddr) -> Self::ConnectFuture<'m> {
		T::connect(self, remote)
	}
}
