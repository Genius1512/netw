use std::error::Error;
use std::net::{Ipv4Addr, TcpStream};
use std::str::FromStr;

pub struct Client {
    current_ip: Option<Ipv4Addr>,
    current_port: Option<u16>,

    stream: Option<TcpStream>,
}

impl Client {
    /// Creates a new client
    pub fn new() -> Client {
        Client {
            current_ip: None,
            current_port: None,

            stream: None,
        }
    }

    pub fn connect(&mut self, ip: &str, port: u16) -> Result<(), Box<dyn Error>> {
        self.current_ip = Some(Ipv4Addr::from_str(ip)?);
        self.current_port = Some(port);

        self.stream = Some(TcpStream::connect(format!("{}:{}",
            self.current_ip.unwrap().to_string(),
            self.current_port.unwrap(),
        ))?);

        Ok(())
    }
}
