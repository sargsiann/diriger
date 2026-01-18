mod server;
use server::Server;
use std::net::TcpListener;

fn main() {
    let server = Server::new(TcpListener::bind("127.0.0.1:8080").unwrap());
    println!("Server started at: {}", server.time_created.format("%Y-%m-%d %H:%M:%S"));
    server.run();
}