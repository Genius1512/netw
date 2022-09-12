use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub struct NetwError {
    text: String,
}

impl NetwError {
    pub fn new(text: &str) -> NetwError {
        NetwError {
            text: text.to_string(),
        }
    }
}

impl fmt::Display for NetwError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Error for NetwError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        &self.text
    }
}