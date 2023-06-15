use crate::{nb, SocketAddr, TcpClientStack, TcpFullStack, UdpClientStack, UdpFullStack};
use core::cell::RefCell;

/// Sharable wrapper for a network stack implementation.
///
/// An implementation of the stack traits that can contain (and provide provide
/// single-threaded shared access to) another stack implementation. A direct
/// implementation can only be used when owned or with a mutable reference.
/// This implementation will store another implementation internally, and yield
/// an arbitrary number of shared references to it, which themselves implement
/// the stack traits.
///
/// ```
/// use embedded_nal::SharableStack;
/// # use embedded_nal::{UdpClientStack, SocketAddr, SocketAddrV4, Ipv4Addr, nb};
/// # struct SomeNalDriver {}
/// # impl SomeNalDriver {
/// #   fn new() -> Self { Self {} }
/// # }
/// # impl UdpClientStack for SomeNalDriver {
/// #   type Error = ();
/// #   type UdpSocket = ();
/// #   fn socket(&mut self) -> Result<Self::UdpSocket, Self::Error> {
/// #     Ok(())
/// #   }
/// #   fn connect(
/// #       &mut self,
/// #       socket: &mut Self::UdpSocket,
/// #       remote: SocketAddr,
/// #   ) -> Result<(), Self::Error> {
/// #     Ok(())
/// #   }
/// #   fn send(&mut self, socket: &mut Self::UdpSocket, buffer: &[u8]) -> nb::Result<(), Self::Error> {
/// #     Ok(())
/// #   }
/// #   fn receive(
/// #       &mut self,
/// #       socket: &mut Self::UdpSocket,
/// #       buffer: &mut [u8],
/// #   ) -> nb::Result<(usize, SocketAddr), Self::Error> {
/// #     Ok((0, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 0))))
/// #   }
/// #   fn close(&mut self, socket: Self::UdpSocket) -> Result<(), Self::Error> {
/// #     Ok(())
/// #   }
/// # }
/// let mut driver = SomeNalDriver::new();
/// // Driver can only be used in one place at a time.
/// let mut sharable_driver = SharableStack::new(driver);
/// // Sharable driver can't do anything on its own, but it can create many usable copies.
/// let mut shared_driver0 = sharable_driver.acquire();
/// let mut shared_driver1 = sharable_driver.acquire();
/// // These shared copies can be passed around to other parts of an application's code, and used
/// // independently.
/// let mut socket0 = shared_driver0.socket()?;
/// shared_driver0.connect(&mut socket0, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
/// // ...
///
/// // ... and somewhere else
/// let mut socket1 = shared_driver1.socket()?;
/// shared_driver1.connect(&mut socket1, SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8443)));
/// // ...
/// # Ok::<(), ()>(())
/// ```
pub struct SharableStack<T> {
	stack: RefCell<T>,
}

impl<T> SharableStack<T> {
	/// Create a new SharedStack that contains and uses some other stack implementation.
	pub fn new(stack: T) -> Self {
		SharableStack {
			stack: RefCell::new(stack),
		}
	}

	/// Returns a shared reference to the driver that can be used as a first-class implementation.
	pub fn acquire(&self) -> SharedStack<T> {
		SharedStack { stack: &self.stack }
	}
}

/// Single-thread shared reference to an internal network stack implementation.
///
/// This can only be created by calling [`SharableStack::acquire()`]
pub struct SharedStack<'a, T> {
	stack: &'a RefCell<T>,
}

macro_rules! forward {
    ($func:ident($($v:ident: $IT:ty),*) -> $T:ty) => {
        fn $func(&mut self, $($v: $IT),*) -> $T {
            self.stack.borrow_mut().$func($($v),*)
        }
    }
}

impl<'a, T> UdpClientStack for SharedStack<'a, T>
where
	T: UdpClientStack,
{
	type Error = T::Error;
	type UdpSocket = T::UdpSocket;

	forward! {socket() -> Result<Self::UdpSocket, Self::Error>}
	forward! {connect(socket: &mut Self::UdpSocket, address: SocketAddr) -> Result<(), Self::Error>}
	forward! {send(socket: &mut Self::UdpSocket, data: &[u8]) -> Result<(), nb::Error<<T as UdpClientStack>::Error>>}
	forward! {receive(socket: &mut Self::UdpSocket, data: &mut [u8]) -> Result<(usize, SocketAddr), nb::Error<<T as UdpClientStack>::Error>>}
	forward! {close(socket: Self::UdpSocket) -> Result<(), Self::Error>}
}

impl<'a, T> UdpFullStack for SharedStack<'a, T>
where
	T: UdpFullStack,
{
	forward! {bind(socket: &mut Self::UdpSocket, local_port: u16) -> Result<(), Self::Error>}
	forward! {send_to(socket: &mut Self::UdpSocket, remote: SocketAddr, buffer: &[u8]) -> Result<(), nb::Error<<T as UdpClientStack>::Error>>}
}

impl<'a, T> TcpClientStack for SharedStack<'a, T>
where
	T: TcpClientStack,
{
	type TcpSocket = T::TcpSocket;
	type Error = T::Error;

	forward! {socket() -> Result<Self::TcpSocket, Self::Error>}
	forward! {connect(socket: &mut Self::TcpSocket, address: SocketAddr) -> Result<(), nb::Error<<T as TcpClientStack>::Error>>}
	forward! {send(socket: &mut Self::TcpSocket, data: &[u8]) -> Result<usize, nb::Error<<T as TcpClientStack>::Error>>}
	forward! {receive(socket: &mut Self::TcpSocket, data: &mut [u8]) -> Result<usize, nb::Error<<T as TcpClientStack>::Error>>}
	forward! {close(socket: Self::TcpSocket) -> Result<(), Self::Error>}
}

impl<'a, T> TcpFullStack for SharedStack<'a, T>
where
	T: TcpFullStack,
{
	forward! {bind(socket: &mut Self::TcpSocket, port: u16) -> Result<(), <T as TcpClientStack>::Error>}
	forward! {listen(socket: &mut Self::TcpSocket) -> Result<(), <T as TcpClientStack>::Error>}
	forward! {accept(socket: &mut Self::TcpSocket) -> Result<(<T as TcpClientStack>::TcpSocket, SocketAddr), nb::Error<<T as TcpClientStack>::Error>>}
}
