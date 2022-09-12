use netw::Server;

fn main() {
    let mut server = Server::new("127.0.0.1", 6969).unwrap();
    server.accept("con").unwrap();
}