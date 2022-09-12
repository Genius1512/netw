use netw::Client;

fn main() {
    let mut client = Client::new();
    client.connect("127.0.0.1", 6969).unwrap();
}