use std::net::TcpStream;
use std::time::SystemTime;
use std::collections::HashMap;
use crate::client::structs::ClientInfo;


struct ServerInfo {
	pub stream: TcpStream,
	pub time_created: SystemTime,
}

impl ServerInfo {
	pub fn new(stream: TcpStream) -> Self {
		Self { stream, time_created: SystemTime::now(), time_last_activity: SystemTime::now() }
	}

	pub fn get_stream(&self) -> &TcpStream {
		&self.stream
	}

	pub fn get_time_created(&self) -> SystemTime {
		self.time_created
	}

	pub fn set_stream(&mut self, stream: TcpStream) {
		self.stream = stream
	}

	pub fn set_time_created(&mut self, time_created: SystemTime) {
		self.time_created = time_created
	}
}

struct ConnectionInfo {
	pub	store: bool,
	pub is_active: bool,
	pub session_id: String,
	pub client: ClientInfo,
	pub server: ServerInfo	,
	pub time_created: SystemTime,
	pub time_last_activity: SystemTime,
}

impl ConnectionInfo {
	pub fn new(client: ClientInfo, server: ServerInfo) -> Self {
		Self { client, server }
	}

	pub fn get_store(&self) -> bool {
		self.store
	}

	pub fn get_session_id(&self) -> &String {
		&self.session_id
	}

	pub fn get_time_created(&self) -> SystemTime {
		self.time_created
	}

	pub fn get_time_last_activity(&self) -> SystemTime {
		self.time_last_activity
	}

	pub fn set_store(&mut self, store: bool) {
		self.store = store
	}

	pub fn set_session_id(&mut self, session_id: String) {
		self.session_id = session_id
	}

	pub fn set_time_created(&mut self, time_created: SystemTime) {
		self.time_created = time_created
	}

	pub fn set_time_last_activity(&mut self, time_last_activity: SystemTime) {
		self.time_last_activity = time_last_activity
	}
}