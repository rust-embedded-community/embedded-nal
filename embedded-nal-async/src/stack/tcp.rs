use core::net::SocketAddr;

/// This trait is implemented by TCP/IP stacks. The trait allows the underlying driver to
/// construct multiple connections that implement the I/O traits from embedded-io-async.
///
/// The associated connection type should close the connection when dropped.
pub trait TcpConnect {
	/// Error type returned on connect failure.
	type Error: embedded_io_async::Error;

	/// Type holding state of a TCP connection. Should close the connection when dropped.
	type Connection<'a>: embedded_io_async::Read<Error = Self::Error>
		+ embedded_io_async::Write<Error = Self::Error>
	where
		Self: 'a;

	/// Connect to the given remote host and port.
	///
	/// Returns `Ok` if the connection was successful.
	async fn connect<'a>(&'a self, remote: SocketAddr)
		-> Result<Self::Connection<'a>, Self::Error>;
}

impl<T: TcpConnect> TcpConnect for &T {
	type Error = T::Error;

	type Connection<'a>
		= T::Connection<'a>
	where
		Self: 'a;

	async fn connect<'a>(
		&'a self,
		remote: SocketAddr,
	) -> Result<Self::Connection<'a>, Self::Error> {
		T::connect(self, remote).await
	}
}
