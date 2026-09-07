use std::io::{Read, Write}; // Bring Read and Write traits into scope, vecasue TcpStream implemets them
use std::net::TcpListener;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap(); // initialize socket server
    println!("Running on port 3000");

    // Next server listens for incoming connections (on iterator)
    for stream in connection_listener.incoming() {
        // If connection happens it unwraps (not proper solution of course), so it became either TCP stream or program panics.
        // Also make stream mutable so we can read and write to it
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024]; // 1KB empty buffer
        stream.read(&mut buffer).unwrap(); // Read from the incoming stream
        stream.write(&mut buffer).unwrap(); // Echo back what is received to the same client/connection
    }
}
