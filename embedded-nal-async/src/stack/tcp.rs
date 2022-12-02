use no_std_net::SocketAddr;

/// This trait is implemented by TCP/IP stacks. The trait allows the underlying driver to
/// construct multiple connections that implement the I/O traits from embedded-io.
///
/// The associated connection type should close the connection when dropped.
pub trait TcpConnect {
	/// Error type returned on connect failure.
	type Error: embedded_io::Error;

	/// Type holding state of a TCP connection. Should close the connection when dropped.
	type Connection<'a>: embedded_io::asynch::Read<Error = Self::Error>
		+ embedded_io::asynch::Write<Error = Self::Error>
	where
		Self: 'a;

	/// Connect to the given remote host and port.
	///
	/// Returns `Ok` if the connection was successful.
	async fn connect<'a>(&'a self, remote: SocketAddr) -> Result<Self::Connection<'a>, Self::Error>
	// This bound is required due to an AFIT limitaton: https://github.com/rust-lang/rust/issues/104908
	where
		Self: 'a;
}

impl<T: TcpConnect> TcpConnect for &T {
	type Error = T::Error;

	type Connection<'a> = T::Connection<'a> where Self: 'a;

	async fn connect<'a>(&'a self, remote: SocketAddr) -> Result<Self::Connection<'a>, Self::Error>
	where
		Self: 'a,
	{
		T::connect(self, remote).await
	}
}
