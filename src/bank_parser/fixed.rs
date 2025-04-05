#[derive(Debug, Clone)]
pub struct Fixed(pub f32);
impl Fixed {
    pub fn new(value: f32) -> Self {
        Fixed(value)
    }
}

impl std::fmt::Display for Fixed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0.fract() == 0.0 {
            write!(f, "{}", self.0 as i32)
        } else {
            write!(f, "{:.2}", self.0)
        }
    }
}

impl From<&str> for Fixed {
    fn from(value: &str) -> Self {
        Fixed(value.parse().unwrap_or(0.0))
    }
}

