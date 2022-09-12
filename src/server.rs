use std::collections::HashMap;
use std::error::Error;
use std::net::{Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::str::FromStr;

pub struct Server {
    ip: Ipv4Addr,
    port: u16,

    listener: TcpListener,

    connections: HashMap<String, (TcpStream, SocketAddr)>,
}

impl Server {
    /// Creates a new Server
    ///
    /// * `ip` - The IP to listen on
    /// * `port` - The port to listen on
    /// * `max_connections` - the maximum number of connections the server supports
    ///
    /// # Panics
    /// - When given an invalid ip address, as it is converted to a std::net::Ipv4Addr
    pub fn new(ip: &str, port: u16) -> Result<Server, Box<dyn Error>> {
        println!("{}:{}", ip, port);

        Ok(Server {
            ip: Ipv4Addr::from_str(ip).unwrap(),
            port,

            listener: TcpListener::bind(format!(
                "{}:{}",
                ip,
                port,
            ))?,

            connections: HashMap::new(),
        })
    }

    pub fn accept(&mut self, id: &str) -> Result<(), Box<dyn Error>> {
        if self.connections.contains_key(id) {
            return Err(Box::new(crate::NetwError::new("ID is already in use")));
        }

        let (connection, addr) = self.listener.accept()?;

        self.connections.insert(
            id.to_string(),
            (connection, addr)
        );

        Ok(())
    }
}
