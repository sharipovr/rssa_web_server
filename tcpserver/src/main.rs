use std::net::TcpListener;

fn main() {
    let connection_listener = TcpListener::bind("127.0.0.1:3000").unwrap(); // initialize socket server
    println!("Running on port 3000");

    // Next server listens for incoming connections (on iterator)
    for stream in connection_listener.incoming() {
        let _stream = stream.unwrap(); // if connection happens it unwraps (not proper solution of course), so it became either TCP stream or program panics
        println!("Connection established");
    }
}
