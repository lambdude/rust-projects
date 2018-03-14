extern crate simple_server;

use simple_server::Server;

fn main() {
    let server = Server::new(|request, mut response| {
        Ok(response.body("Hello, world!".as_bytes())?)
    });

    server.listen("127.0.0.1", "7979");
}
