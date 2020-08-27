//! # embedded-nal - A Network Abstraction Layer for Embedded Systems

#![no_std]
#![deny(missing_docs)]
#![deny(unsafe_code)]

pub use nb;
mod dns;
pub use dns::{AddrType, Dns};

use generic_array::{GenericArray, ArrayLength};

pub use no_std_net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, SocketAddrV4, SocketAddrV6};

/// Whether a socket should block when a read/write can't be performed, or return early.
pub enum Mode {
	/// The function call will wait as long as necessary to complete the operation
	Blocking,
	/// The function call will not wait at all to complete the operation, and only do what it can.
	NonBlocking,
	/// The function call will wait only up the given number of milliseconds to complete the operation.
	Timeout(u16),
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
    /// The default read/write implementation buffer size.
    type Size: ArrayLength<u8>;

	/// Open a new TCP socket. The socket starts in the unconnected state.
	fn open(&self, mode: Mode) -> Result<Self::TcpSocket, Self::Error>;

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

    /// Write to the stream using a closure. The provided function should copy data into the
    /// provided buffer and return the number of bytes written. Returns the number of bytes actually
    /// written.
    fn write_with<F>(&self, socket: &mut Self::TcpSocket, f: F) -> nb::Result<usize, Self::Error>
    where
        F: FnOnce(&mut [u8]) -> usize
    {
        let mut buffer: GenericArray<u8, Self::Size> = GenericArray::default();
        let len = f(&mut buffer);
        self.write(socket, &buffer[..len])
    }

	/// Read from the stream. Returns `Ok(n)`, which means `n` bytes of
	/// data have been received and they have been placed in
	/// `&buffer[0..n]`, or an error.
	fn read(
		&self,
		mut socket: &mut Self::TcpSocket,
		buffer: &mut [u8],
	) -> nb::Result<usize, Self::Error> {
        let len = self.try_recv(&mut socket, buffer)?;
        self.recv_commit(&mut socket, len)?;
        Ok(len)
    }

    /// Attempt to receive bytes from the socket.
    /// Returns the number of bytes actually read. Note that this function should not remove data
    /// from the socket stream. Once data is actually consumed, `recv_commit()` should be called to
    /// consume the data.
    fn try_recv(&self, socket: &mut Self::TcpSocket, buffer: &mut [u8]) -> Result<usize, Self::Error>;

    /// Consume data previously received using `try_recv()`. This dequeues the data from the socket.
    fn recv_commit(&self, socket: &mut Self::TcpSocket, count: usize) -> Result<(), Self::Error>;

    /// Read data directly from the stack RX buffer. Note that the default implementation will
    /// instantiate a buffer on the stack for cases where reading directly from the stack RX buffer is
    /// not possible.
    fn read_from<F>(&self, mut socket: &mut Self::TcpSocket, f: F) -> Result<(), Self::Error>
    where
        F: FnOnce(&[u8]) -> usize
    {
        let mut buffer: GenericArray<u8, Self::Size> = GenericArray::default();

        let len = self.try_recv(&mut socket, &mut buffer)?;
        let read_size = f(&buffer[..len]);
        self.recv_commit(&mut socket, read_size)?;

        Ok(())
    }

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
	fn open(&self, remote: SocketAddr, mode: Mode) -> Result<Self::UdpSocket, Self::Error>;

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

// End Of File
