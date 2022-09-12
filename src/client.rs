use std::borrow::BorrowMut;
use std::error::Error;
use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::str::FromStr;
use crate::NetwError;

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

        let mut buf: [u8; 1] = [0u8; 1];
        self.stream.as_mut().unwrap().read(&mut buf)?;
        if buf != [0] {
            return Err(Box::new(NetwError::new("Server did not return success")));
        }
        self.stream.as_ref().unwrap().write(&[0])?;

        Ok(())
    }
}
