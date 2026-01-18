// Includeing standard tcplistener and tcpstream
use std::net::TcpListener;
use std::net::TcpStream;
// Including chrono for date and time
use chrono::{DateTime, Local};
//including std::io for read and write
use std::io::{Read, Write};


pub struct Server {
	pub information: TcpListener, // the server listener struct
	pub time_created: DateTime<Local>, // the time the server was created
}

// the Result <String, std::io::Error> for the client_read and client_write methods
// means that the methods can return a string or an error

// methods for the server struct
impl Server {

	// constructor for the server struct
	pub fn new(information: TcpListener) -> Self {
		Self { information: information, time_created: Local::now() }
	}

	//private method to read from the client
	fn client_read(&self, client: &mut TcpStream) -> Result<String, std::io::Error> {
		// mutable buffer to read from the client
		let mut buffer = [0; 1024];
		// read from the client
		match client.read(&mut buffer) {
			// if the client disconnected
			Ok(0) => {
				return Ok(String::new());
			}
			// if the client sent data
			Ok(n) => {
				// return the data as a string
				return Ok(String::from_utf8_lossy(&buffer[..n]).to_string());
			}
			// if there was an error reading from the client
			Err(e) => {
				return Err(e);
			}
		}
	}

	//private method to write to the client
	fn client_write(&self, client: &mut TcpStream, message: &str) -> Result<(), std::io::Error> {
		// write to the client
		match client.write(message.as_bytes()) {
			// if the client sent data
			Ok(n) => {
				// return success
				return Ok(());
			}
			// if there was an error writing to the client
			Err(e) => {
				return Err(e);
			}
		}
	}

	//private method to handle the client
	fn handle_client(&self, client: &mut TcpStream) {
		// loop to read from the client each time
		println!("Handling client {}", client.peer_addr().unwrap());
		loop {
			// read from the client
			let message = self.client_read(client);
			match message {
				Ok(message) => {
					// if message is empty, the client disconnected
					if message.is_empty() {
						println!("Client disconnected");
						break;
					}
					// print the message received from the client
					println!("Received {}", message);
					// write to the client
					match self.client_write(client, &message) {
						// if the client sent data
						Ok(()) => {
							// print success
							println!("Sent Back: {}", message);
						}
						// if there was an error writing to the client
						Err(e) => {
							println!("Error: {}", e);
						}
					}
				}
				// if there was an error reading from the client
				Err(e) => {
					println!("Error: {}", e);
					break;
				}
			}
		}
	}

	//public method to run the server
	pub fn run(&self) {
		// loop to accept new clients
		for client in self.information.incoming() {
			// accept the client
			match client {
				Ok(mut client) => {
					// handle the client
					self.handle_client(&mut client);
				}
				Err(e) => {
					println!("Error: {}", e);
				}
			}
		}
	}
}