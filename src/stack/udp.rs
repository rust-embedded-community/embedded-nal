use core::net::SocketAddr;

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

	/// Allocate a socket for further use.
	fn socket(&mut self) -> Result<Self::UdpSocket, Self::Error>;

	/// Connect a UDP socket with a peer using a dynamically selected port.
	///
	/// Selects a port number automatically and initializes for read/writing.
	fn connect(
		&mut self,
		socket: &mut Self::UdpSocket,
		remote: SocketAddr,
	) -> Result<(), Self::Error>;

	/// Send a datagram to the remote host.
	///
	/// The remote host used is either the one specified in `UdpStack::connect`
	/// or the last one used in `UdpServerStack::write_to`.
	fn send(&mut self, socket: &mut Self::UdpSocket, buffer: &[u8]) -> nb::Result<(), Self::Error>;

	/// Read a datagram the remote host has sent to us.
	///
	/// Returns `Ok((n, remote))`, which means a datagram of size `n` has been
	/// received from `remote` and been placed in `&buffer[0..n]`, or an error.
	/// If a packet has not been received when called, then [`nb::Error::WouldBlock`]
	/// should be returned.
	fn receive(
		&mut self,
		socket: &mut Self::UdpSocket,
		buffer: &mut [u8],
	) -> nb::Result<(usize, SocketAddr), Self::Error>;

	/// Close an existing UDP socket.
	fn close(&mut self, socket: Self::UdpSocket) -> Result<(), Self::Error>;
}

/// This trait is implemented by UDP/IP stacks.  It provides the ability to
/// listen for packets on a specified port and send replies.
pub trait UdpFullStack: UdpClientStack {
	/// Bind a UDP socket with a specified port
	fn bind(&mut self, socket: &mut Self::UdpSocket, local_port: u16) -> Result<(), Self::Error>;

	/// Send a packet to a remote host/port.
	fn send_to(
		&mut self,
		socket: &mut Self::UdpSocket,
		remote: SocketAddr,
		buffer: &[u8],
	) -> nb::Result<(), Self::Error>;
}

impl<T: UdpClientStack> UdpClientStack for &mut T {
	type Error = T::Error;

	type UdpSocket = T::UdpSocket;

	fn socket(&mut self) -> Result<Self::UdpSocket, Self::Error> {
		T::socket(self)
	}

	fn connect(
		&mut self,
		socket: &mut Self::UdpSocket,
		remote: SocketAddr,
	) -> Result<(), Self::Error> {
		T::connect(self, socket, remote)
	}

	fn send(&mut self, socket: &mut Self::UdpSocket, buffer: &[u8]) -> nb::Result<(), Self::Error> {
		T::send(self, socket, buffer)
	}

	fn receive(
		&mut self,
		socket: &mut Self::UdpSocket,
		buffer: &mut [u8],
	) -> nb::Result<(usize, SocketAddr), Self::Error> {
		T::receive(self, socket, buffer)
	}

	fn close(&mut self, socket: Self::UdpSocket) -> Result<(), Self::Error> {
		T::close(self, socket)
	}
}

impl<T: UdpFullStack> UdpFullStack for &mut T {
	fn bind(&mut self, socket: &mut Self::UdpSocket, local_port: u16) -> Result<(), Self::Error> {
		T::bind(self, socket, local_port)
	}

	fn send_to(
		&mut self,
		socket: &mut Self::UdpSocket,
		remote: SocketAddr,
		buffer: &[u8],
	) -> nb::Result<(), Self::Error> {
		T::send_to(self, socket, remote, buffer)
	}
}
