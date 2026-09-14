mod handler;
mod router;
mod server;
use server::Server;

fn main() {
    // Start a server
    let server = Server::new("localhost:3001");
    // Run the server
    server.run();
}
