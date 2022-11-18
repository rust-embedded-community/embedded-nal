//! Traits for using UDP on embedded devices
//!
//! ## Notes for implementers
//!
//! * At several places, the APIs expect to provide a local address. Backends that can not obtain
//!   it, such as some AT-command based stacks, <!-- should question whether they may really call
//!   themselves UDP and --> may pretend to have performed some form of network address
//!   translation, and present invalid addresses as the local address.
//!
//! * Implementing [`UdpStack::UniquelyBound`] and [`UdpStack::MultiplyBound`] unconnected sockets
//!   separately allows discarding the local addresses in the bound case. With LTO enabled, all the
//!   overhead compared with a third trait variant between [ConnectedUdp] and [UnconnectedUdp] (in
//!   which the local address is static but the remote address is flexible) should optimized out.
//!   Implementing `UniquelyBound` and `MultiplyBound` with the same type is expected to be a
//!   common choice.

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
	/// Error type returned by send and receive operations.
	type Error: embedded_io::Error;

	/// Send the provided data to the connected peer
	async fn send<'a>(&mut self, data: &[u8]) -> Result<(), Self::Error>;

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
	async fn receive_into(&mut self, buffer: &mut [u8]) -> Result<usize, Self::Error>;

	// WIP to allow zero-copy operation
	// The plain receive is simple and can be provided -- implementations that don't populate
	// receive calls from scatter-gather can just return a slice of the raw data instead, and rely
	// on the socket still being exclusively owned. receive_oned is harder as providing it requires
	// alloc.
	//
	// async fn receive(&mut self, buffer: &mut [u8]) -> utput = Result<impl AsRef<u8> + '_, Self::Error>;
	// async fn receive_owned(&mut self) -> Result<impl AsRef<u8> + 'static, Self::Error>;
}

/// This trait is implemented by UDP sockets.
///
/// The socket it represents is not necessarily bound (may not have a single local IP address, port
/// and interface), and is typically not connected (has no remote IP address and port). Both are
/// addresses are explicitly given in every call.
///
/// If there were constraints in place at socket creation time (typically on the local side), the
/// caller MUST pass in the same (or compatible) values, MAY and pass in unspecified values where
/// applicable. The implementer MAY check them for compatibility, and SHOULD do that in debug mode.
pub trait UnconnectedUdp {
	/// Error type returned by send and receive operations.
	type Error: embedded_io::Error;

	/// Send the provided data to a peer
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
	///
	/// Note that users of sockets created through [`UdpStack::bind_single()`] should always pass
	/// in that single address -- even though they've made their intention clear at construction.
	/// They can pass either the one obtained at socket creation time, or the one obtained at
	/// receive time; these should be equal. This allows implementations of the trait to use a
	/// single kind of socket for both sockets bound to a single and sockets bound to multiple
	/// addresses.
	async fn send(
		&mut self,
		local: SocketAddr,
		remote: SocketAddr,
		data: &[u8],
	) -> Result<(), Self::Error>;

	/// Receive a datagram into the provided buffer.
	///
	/// If the received datagram exceeds the buffer's length, it is received regardless, and the
	/// remaining bytes are discarded. The full datagram size is still indicated in the result,
	/// allowing the recipient to detect that truncation.
	///
	/// The local and remote address are given, in that order, in the result along with the number
	/// of bytes.
	async fn receive_into(& mut self, buffer: & mut [u8]) -> Result<(usize, SocketAddr, SocketAddr), Self::Error>;
}

/// This trait is implemented by UDP/IP stacks. The trait allows the underlying driver to
/// construct multiple connections that implement the I/O traits from embedded-io.
///
/// Note that stacks with exotic connection creation methods may still not implement this, yet have
/// objects that implement [`ConnectedUdp`] or similar.
pub trait UdpStack {
	/// Error type returned on socket creation failure.
	type Error: embedded_io::Error;

	/// Eventual socket return type of the [`.connect()`] method
	type Connected: ConnectedUdp;
	/// Eventual socket return type of the [`.bind_single()`] method
	type UniquelyBound: UnconnectedUdp;
	/// Eventual return type of the [`.bind_multiple()`] method
	type MultiplyBound: UnconnectedUdp;

	/// Create a socket that has a fixed remote address.
	///
	/// The local address is chosen automatically.
	///
    /// There is a provided implementation that implements this from the maximally unspecified
    /// local address and [`.connect_from()`], but may be provided more efficiently by
    /// implementers.
	async fn connect(&self, remote: SocketAddr) -> Result<(SocketAddr, Self::Connected), Self::Error> {
		use no_std_net::{Ipv4Addr, Ipv6Addr, SocketAddr::*, SocketAddrV4, SocketAddrV6};

		let local = match remote {
			V4(_) => V4(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0)),
			V6(_) => V6(SocketAddrV6::new(Ipv6Addr::UNSPECIFIED, 0, 0, 0)),
		};
		self.connect_from(local, remote).await
    }

	/// Create a socket that has a fixed remote address.
	///
	/// The local address is given explicitly, but may be partially unspecified; it is fixed by the
	/// network stack at connection time. The full local address is returned along with the
	/// connected socket, primarily for debugging purposes.
	async fn connect_from(&self, local: SocketAddr, remote: SocketAddr) -> Result<(SocketAddr, Self::Connected), Self::Error>;

	/// Create a socket that has a fixed local address.
	///
	/// Note that the giving an unspecified address here is *not* the same as a POSIX `bind()` --
	/// if the underlying stack supports multiple local addresses, it will pick *one* of the
	/// applicable addresses, rather than binding to all of them.
	///
	/// The full local address is returned along with the bound socket; it may then be passed on to
	/// other protocols for advertising purposes.
	async fn bind_single(&self, local: SocketAddr) -> Result<(SocketAddr, Self::UniquelyBound), Self::Error>;

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
	async fn bind_multiple(&self, local: SocketAddr) -> Result<Self::MultiplyBound, Self::Error>;

}
