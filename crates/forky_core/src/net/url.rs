pub const HOST_ALL: [u8; 4] = [0, 0, 0, 0];
pub const HOST_LOCAL: [u8; 4] = [127, 0, 0, 1];

#[derive(Debug, Clone)]
pub struct Url {
	pub ip: IpProtocol,
	pub path: String,
}
impl Url {
	pub fn new(ip: IpProtocol, path: String) -> Self { Self { ip, path } }
}

impl std::fmt::Display for Url {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}/{}", self.ip, self.path)
	}
}

#[derive(Debug, Clone, Copy)]
pub struct Ip {
	pub port: u16,
	pub ip: [u8; 4],
}

impl Default for Ip {
	fn default() -> Self {
		Self {
			port: 8080,
			ip: HOST_ALL,
		}
	}
}

impl std::fmt::Display for Ip {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}.{}.{}.{}:{}",
			self.ip[0], self.ip[1], self.ip[2], self.ip[3], self.port
		)
	}
}

#[derive(Debug, Clone, Copy)]
pub enum Protocol {
	Http,
	Https,
	Ws,
	Wss,
}

impl std::fmt::Display for Protocol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let val: &str = self.into();
		write!(f, "{val}",)
	}
}
impl From<&Protocol> for &'static str {
	fn from(value: &Protocol) -> Self {
		match value {
			Protocol::Http => "http",
			Protocol::Https => "https",
			Protocol::Ws => "ws",
			Protocol::Wss => "wss",
		}
	}
}

#[derive(Debug, Clone, Copy)]
pub struct IpProtocol {
	pub ip: Ip,
	pub protocol: Protocol,
}

impl IpProtocol {
	pub fn new(protocol: Protocol, ip: Ip) -> Self { Self { ip, protocol } }
}

impl std::fmt::Display for IpProtocol {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}://{}", self.protocol, self.ip)
	}
}
