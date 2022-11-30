use embedded_nal::AddrType;
use heapless::String;
use no_std_net::IpAddr;

/// This trait is an extension trait for [`TcpStack`] and [`UdpStack`] for dns
/// resolutions. It does not handle every DNS record type, but is meant as an
/// embedded alternative to [`ToSocketAddrs`], and is as such meant to resolve
/// an ip address from a hostname, or a hostname from an ip address. This means
/// that it only deals in host address records `A` (IPv4) and `AAAA` (IPv6).
///
/// [`TcpStack`]: crate::trait@TcpStack
/// [`UdpStack`]: crate::trait@UdpStack
/// [`ToSocketAddrs`]:
/// https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html
pub trait Dns {
	/// The type returned when we have an error
	type Error: core::fmt::Debug;

	/// Resolve the first ip address of a host, given its hostname and a desired
	/// address record type to look for
	async fn get_host_by_name(
		&self,
		host: &str,
		addr_type: AddrType,
	) -> Result<IpAddr, Self::Error>;

	/// Resolve the hostname of a host, given its ip address
	///
	/// **Note**: A fully qualified domain name (FQDN), has a maximum length of
	/// 255 bytes [`rfc1035`]
	///
	/// [`rfc1035`]: https://tools.ietf.org/html/rfc1035
	async fn get_host_by_address(&self, addr: IpAddr) -> Result<String<256>, Self::Error>;
}
