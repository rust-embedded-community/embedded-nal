use core::future::Future;
use no_std_net::SocketAddr;

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
