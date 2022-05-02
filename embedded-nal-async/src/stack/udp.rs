use core::future::Future;
use no_std_net::SocketAddr;

/// This trait is implemented by UDP/IP stacks. You could, for example, have
/// an implementation which knows how to send AT commands to an ESP8266 WiFi
/// module. You could have another implementation which knows how to driver the
/// Rust Standard Library's `std::net` module. Given this trait, you can how
/// write a portable CoAP client which can work with either implementation.
pub trait UdpClientStack {
	/// The type returned when we create a new UDP socket
	type UdpSocket;
	/// The type returned when we have an error
	type Error: core::fmt::Debug;

	/// Future returned by `socket` function.
	type SocketFuture<'m>: Future<Output = Result<Self::UdpSocket, Self::Error>> + 'm
	where
		Self: 'm;

	/// Open a socket for usage as a UDP client.
	///
	/// Returns `Ok(socket)` if the socket was successfully created.
	fn socket<'m>(&'m mut self) -> Self::SocketFuture<'m>;

	/// Future returned by `connect` function.
	type ConnectFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;

	/// Connect a UDP socket with a peer using a dynamically selected port.
	///
	/// Selects a port number automatically and initializes for read/writing.
	fn connect<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		remote: SocketAddr,
	) -> Self::ConnectFuture<'m>;

	/// Future returned by `send` function.
	type SendFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;

	/// Send a datagram to the remote host.
	///
	/// The remote host used is either the one specified in `UdpStack::connect`
	/// or the last one used in `UdpServerStack::write_to`.
	fn send<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		buffer: &'m [u8],
	) -> Self::SendFuture<'m>;

	/// Future returned by `receive` function.
	type ReceiveFuture<'m>: Future<Output = Result<(usize, SocketAddr), Self::Error>> + 'm
	where
		Self: 'm;

	/// Read a datagram the remote host has sent to us.
	///
	/// Returns `Ok((n, remote))`, which means a datagram of size `n` has been
	/// received from `remote` and been placed in `&buffer[0..n]`, or an error.
	/// If a packet has not been received when called, then [`nb::Error::WouldBlock`]
	/// should be returned.
	fn receive<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		buffer: &'m mut [u8],
	) -> Self::ReceiveFuture<'m>;

	/// Future returned by `close` function.
	type CloseFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;

	/// Close an existing UDP socket.
	fn close<'m>(&'m mut self, socket: Self::UdpSocket) -> Self::CloseFuture<'m>;
}

/// This trait is implemented by UDP/IP stacks.  It provides the ability to
/// listen for packets on a specified port and send replies.
pub trait UdpFullStack: UdpClientStack {
	/// Future returned by `bind` function.
	type BindFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;

	/// Bind a UDP socket with a specified port
	fn bind<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		local_port: u16,
	) -> Self::BindFuture<'m>;

	/// Future returned by `send_to` function.
	type SendToFuture<'m>: Future<Output = Result<(), Self::Error>> + 'm
	where
		Self: 'm;

	/// Send a packet to a remote host/port.
	fn send_to<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		remote: SocketAddr,
		buffer: &'m [u8],
	) -> Self::SendToFuture<'m>;
}

impl<T: UdpClientStack> UdpClientStack for &mut T {
	type Error = T::Error;
	type UdpSocket = T::UdpSocket;

	type SocketFuture<'m> = T::SocketFuture<'m> where Self: 'm;
	fn socket<'m>(&'m mut self) -> Self::SocketFuture<'m> {
		T::socket(self)
	}

	type ConnectFuture<'m> = T::ConnectFuture<'m> where Self: 'm;
	fn connect<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		remote: SocketAddr,
	) -> Self::ConnectFuture<'m> {
		T::connect(self, socket, remote)
	}

	type SendFuture<'m> = T::SendFuture<'m> where Self: 'm;
	fn send<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		buffer: &'m [u8],
	) -> Self::SendFuture<'m> {
		T::send(self, socket, buffer)
	}

	type ReceiveFuture<'m> = T::ReceiveFuture<'m> where Self: 'm;
	fn receive<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		buffer: &'m mut [u8],
	) -> Self::ReceiveFuture<'m> {
		T::receive(self, socket, buffer)
	}

	type CloseFuture<'m> = T::CloseFuture<'m> where Self: 'm;
	fn close<'m>(&'m mut self, socket: Self::UdpSocket) -> Self::CloseFuture<'m> {
		T::close(self, socket)
	}
}

impl<T: UdpFullStack> UdpFullStack for &mut T {
	type BindFuture<'m> = T::BindFuture<'m> where Self: 'm;
	fn bind<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		local_port: u16,
	) -> Self::BindFuture<'m> {
		T::bind(self, socket, local_port)
	}

	type SendToFuture<'m> = T::SendToFuture<'m> where Self: 'm;
	fn send_to<'m>(
		&'m mut self,
		socket: &'m mut Self::UdpSocket,
		remote: SocketAddr,
		buffer: &'m [u8],
	) -> Self::SendToFuture<'m> {
		T::send_to(self, socket, remote, buffer)
	}
}
