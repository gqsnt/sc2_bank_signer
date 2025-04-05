use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
pub struct Flag(pub bool);

impl Flag {
    pub fn new(value: bool) -> Self {
        Flag(value)
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.0 { '1' } else { '0' })
    }
}

impl FromStr for Flag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Flag(true)),
            "0" => Ok(Flag(false)),
            _ => Err("Invalid flag value".to_string()),
        }
    }
}

impl From<&str> for Flag {
    fn from(value: &str) -> Self {
        Flag(value == "1")
    }
}