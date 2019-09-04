use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct Fluck {
    e: String
}

impl Fluck {
    fn new(msg: &str) -> Fluck {
        Fluck{ e: msg.to_string() }
    }
}

impl fmt::Display for Fluck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.e)
    }
}

impl Error for Fluck {
    fn description(&self) -> &str {
        &self.e
    }
}

