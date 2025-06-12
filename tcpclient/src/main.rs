use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut _stream = TcpStream::connect("localhost:3000").unwrap();
    _stream.write("Hello server!".as_bytes()).unwrap();
    let mut buffer = [0; 5];
    _stream.read(&mut buffer).unwrap();
    println!(
        "Got response from server: {:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
