
#[derive(Debug, Clone, Copy)]
pub struct Flag(pub bool);

impl Flag {
    pub fn new(value: bool) -> Self {
        Flag(value)
    }
}

impl std::fmt::Display for Flag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", if self.0 { "1" } else { "0" })
    }
}

impl From<&str> for Flag {
    fn from(value: &str) -> Self {
        Flag(value == "1")
    }
}
