use std::net::TcpStream;

struct ClientInfo {
    stream: TcpStream,
}

impl ClientInfo {
	pub fn new(stream: TcpStream) -> Self {
		Self { stream }
	}

	pub fn get_stream(&self) -> &TcpStream {
		&self.stream
	}

	pub fn set_stream(&mut self, stream: TcpStream) {
		self.stream = stream
	}
}