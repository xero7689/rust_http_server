use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;

fn main() {
    let bind_address = "127.0.0.1:3000";
    let connection_listener = TcpListener::bind(bind_address).unwrap();
    println!("Running on port 3000..");
    for stream in connection_listener.incoming() {
        let mut _stream = stream.unwrap();
        println!("Connection established!");

        let mut buffer = [0; 1024];
        _stream.read(&mut buffer).unwrap();
        _stream.write(&mut buffer).unwrap();

        println!(
            "Got message from client: {:?}",
            str::from_utf8(&buffer).unwrap()
        );
    }
}
