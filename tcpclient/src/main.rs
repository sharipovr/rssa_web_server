use std::io::{Read, Write};
use std::net::TcpStream;
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:3000").unwrap(); // initiates connection to server
    stream.write("Hello world".as_bytes()).unwrap(); // Write "Hello" to TCP connection to server
    let mut buffer = [0; 5];
    stream.read(&mut buffer).unwrap(); // Read bytes received from server
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );
}
