use core::future::Future;
use no_std_net::SocketAddr;

/// This trait is implemented by UDP sockets.
///
/// The socket it represents is both bound (has a local IP address, port and interface) and
/// connected (has a remote IP address and port).
///
/// The term "connected" here refers to the semantics of POSIX datagram sockets, through which datagrams
/// are sent and received without having a remote address per call. It does not imply any process
/// of establishing a connection (which is absent in UDP). While there is typically no POSIX
/// `bind()` call in the creation of such sockets, these are implicitly bound to a suitable local
/// address at connect time.
pub trait ConnectedUdp {
	type Error: embedded_io::Error;

	/// Send the provided data to the connected peer
	fn send(&mut self, data: &[u8]) -> Self::SendFuture<'_>;
	type SendFuture<'a>: Future<Output = Result<(), Self::Error>>
	where
		Self: 'a;

	/// Receive a datagram into the provided buffer.
	///
	/// If the received datagram exceeds the buffer's length, it is received regardless, and the
	/// remaining bytes are discarded. The full datagram size is still indicated in the result,
	/// allowing the recipient to detect that truncation.
	///
	/// ## Compatibility note
	///
	/// This deviates from the sync/nb equivalent trait in that it describes the overflow behavior
	/// (a possibility not considered there). The name deviates from the original `receive()` to
	/// make room for a version that is more zero-copy friendly.
	fn receive_into(&mut self, buffer: &mut [u8]) -> Self::ReceiveIntoFuture<'_>;
	type ReceiveIntoFuture<'a>: Future<Output = Result<usize, Self::Error>>
	where
		Self: 'a;

	// WIP to allow zero-copy operation
	// The plain receive is simple and can be provided -- implementations that don't populate
	// receive calls from scatter-gather can just return a slice of the raw data instead, and rely
	// on the socket still being exclusively owned. receive_oned is harder as providing it requires
	// alloc.
	//
	// fn receive(&mut self, buffer: &mut [u8]) -> impl Future<Output = Result<impl AsRef<u8> + '_, Self::Error>>;
	// fn receive_owned(&mut self) -> impl Future<Output = Result<impl AsRef<u8> + 'static, Self::Error>>;
}

/// This trait is implemented by UDP sockets.
///
/// The socket it represents is both bound (has a local IP address, port and interface) but not
/// connected; its peer IP address is explicit in every call.
///
/// This is similar to a POSIX datagram socket that has been bound to a concrete address.
pub trait BoundUdp {
	type Error: embedded_io::Error;

	/// Send the provided data to the connected peer
	fn send_to(&mut self, remote: SocketAddr, data: &[u8]) -> Self::SendToFuture<'_>;
	type SendToFuture<'a>: Future<Output = Result<(), Self::Error>>
	where
		Self: 'a;

	/// Receive a datagram into the provided buffer.
	///
	/// If the received datagram exceeds the buffer's length, it is received regardless, and the
	/// remaining bytes are discarded. The full datagram size is still indicated in the result,
	/// allowing the recipient to detect that truncation.
	///
	/// The remote address is given in the result along with the number of bytes.
	///
	/// ## Compatibility note
	///
	/// This deviates from the sync/nb equivalent trait in that it describes the overflow behavior
	/// (a possibility not considered there). The name deviates from the original `receive()` to
	/// make room for a version that is more zero-copy friendly.
	fn receive_from_into(&mut self, buffer: &mut [u8]) -> Self::ReceiveFromIntoFuture<'_>;
	type ReceiveFromIntoFuture<'a>: Future<Output = Result<(usize, SocketAddr), Self::Error>>
	where
		Self: 'a;
}

/// This trait is implemented by UDP sockets.
///
/// The socket it represents is neither bound (has no single local IP address, port and interface)
/// nor connected (has no remote IP address and port). Both are explicitly given in every call.
///
/// There may be constraints placed on an unbound socket at creation time that limit the range of
/// local addresses (further than the natural limitation of only using addresses assigned to the
/// host).
///
/// A typical example of this kind of socket is a POSIX datagram socket that has been bound to
/// "any" address (`[::]` or `0.0.0.0`) but to a particular port.
pub trait UnboundUdp {
	type Error: embedded_io::Error;

	/// Send the provided data to the connected peer
	///
	/// ## Sending initial messages
	///
	/// The local address can be left unspecified by leaving any of its component zero -- that
	/// gives the "any" address (`[::]` / `0.0.0.0`), the uncspecified port (0) or the unspecified
	/// zone identifier (0). Unless the operating system provides facilities exceeding this crate's traits for
	/// enumerating local interfaces and addresses, this is the only way to initiate outbound
	/// traffic.
	///
	/// ## Responding to messages
	///
	/// Users who have previously received data from a peer and want to respond have a choice of
	/// sending from the address to which the original datagram was addressed, or from an unbound
	/// address. Both are valid choices in some situations, and the right choice depends on the
	/// protocol used.
	fn send(&mut self, local: SocketAddr, remote: SocketAddr, data: &[u8]) -> Self::SendFuture<'_>;
	type SendFuture<'a>: Future<Output = Result<(), Self::Error>>
	where
		Self: 'a;

	/// Receive a datagram into the provided buffer.
	///
	/// If the received datagram exceeds the buffer's length, it is received regardless, and the
	/// remaining bytes are discarded. The full datagram size is still indicated in the result,
	/// allowing the recipient to detect that truncation.
	///
	/// The local and remote address are given, in that order, in the result along with the number
	/// of bytes.
	fn receive(&mut self, buffer: &mut [u8]) -> Self::ReceiveFuture<'_>;
	type ReceiveFuture<'a>: Future<Output = Result<(usize, SocketAddr, SocketAddr), Self::Error>>
	where
		Self: 'a;
}

/// This trait is implemented by UDP/IP stacks. The trait allows the underlying driver to
/// construct multiple connections that implement the I/O traits from embedded-io.
///
/// Note that stacks with exotic connection creation methods may still not implement this, yet have
/// objects that implement [`ConnectedUdp`] or similar.
pub trait UdpStack {
	/// Error type returned on socket creation failure.
	type Error: embedded_io::Error;

	type Connected<'m>: ConnectedUdp
	where
		Self: 'm;
	type Bound<'m>: BoundUdp
	where
		Self: 'm;
	type Unbound<'m>: UnboundUdp
	where
		Self: 'm;

	/// Create a socket that has a fixed remote address.
	///
	/// The local address is chosen automatically.
	///
	/// While asynchronous traits implemented through GAT can not have provided default methods,
	/// implementers are encouraged to use the hidden `.connect_default()` method if all they would
	/// do is delegating to [`connect_from`] with a suitable unspecified local address.
	fn connect(&self, remote: SocketAddr) -> Self::ConnectFuture<'_>;
	type ConnectFuture<'a>: Future<Output = Result<(SocketAddr, Self::Connected<'a>), Self::Error>>
	where
		Self: 'a;

	/// Create a socket that has a fixed remote address.
	///
	/// The local address is given explicitly, but may be partially unspecified; it is fixed by the
	/// network stack at connection time. The full local address is returned along with the
	/// connected socket, primarily for debugging purposes.
	fn connect_from(&self, local: SocketAddr, remote: SocketAddr) -> Self::ConnectFromFuture<'_>;
	type ConnectFromFuture<'a>: Future<
		Output = Result<(SocketAddr, Self::Connected<'a>), Self::Error>,
	> where
		Self: 'a;

	/// Helper that implements [`connect()`] using [`connect_from()`].
	#[doc(hidden)]
	fn connect_default(&self, remote: SocketAddr) -> Self::ConnectFromFuture<'_> {
		use no_std_net::{Ipv4Addr, Ipv6Addr, SocketAddr::*, SocketAddrV4, SocketAddrV6};

		let local = match remote {
			V4(_) => V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
			V6(_) => V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
		};
		self.connect_from(local, remote)
	}

	/// Create a socket that has a fixed local address.
	///
	/// Note that the giving an unspecified address here is *not* the same as a POSIX `bind()` --
	/// if the underlying stack supports multiple local addresses, it will pick *one* of the
	/// applicable addresses, rather than binding to all of them.
	///
	/// The full local address is returned along with the bound socket; it may then be passed on to
	/// other protocols for advertising purposes.
	fn bind_single(&self, local: SocketAddr) -> Self::BindSingleFuture<'_>;
	type BindSingleFuture<'a>: Future<Output = Result<(SocketAddr, Self::Bound<'a>), Self::Error>>
	where
		Self: 'a;

	/// Create a socket that has no single fixed local address.
	///
	/// The IP address part of the local address is typically left unspecified, and the port is
	/// given. There are use cases for other constellations, and this interface does not rule out
	/// that they can be used, but they are rare (e.g. using the same IP address on different
	/// network interfaces, and listening to datagrams arriving at any of them) or not well
	/// supported by operating systems (e.g., binding to all ports at the same is not possible on
	/// POSIX systems, where giving port 0 to a bind makes the OS pick *some* suitable port).
	///
	/// Caveats:
	///
	/// * There is currently no way to pass in a local address that has an unspecified address
	///   family (which would effectively create a single socket that servers both IPv4 and IPv6);
	///   it is not specified whether stacks that use V6MAPPED IPv4 addresses could simply used
	///   that mechanism.
	///
	/// * It is currently not specified whether this mechanism can be used to join multicast
	///   groups.
	///
	/// * There is currently no hybrid binding that allows emulating what POSIX systems do when
	///   binding to `[::]:0`, that is, picking some available port but then still leaving the
	///   interface and IP address unspecified.
	fn bind_multiple(&self, local: SocketAddr) -> Self::BindMultipleFuture<'_>;
	type BindMultipleFuture<'a>: Future<Output = Result<Self::Unbound<'a>, Self::Error>>
	where
		Self: 'a;
}
