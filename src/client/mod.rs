mod structs;

use structs::ClientInfo;
use std::net::TcpStream;

fn init_client_info() -> ClientInfo {
    
	ClientInfo {
        fd: 0,
        ip: String::new(),
        port: 0,
        timestamp: 0,
    }
}

pub fn main() {

}
