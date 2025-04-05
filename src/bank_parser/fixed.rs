use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Fixed(pub f32);
impl Fixed {
    pub fn new(value: f32) -> Self {
        Fixed(value)
    }
}

impl std::fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.fract().abs() < f32::EPSILON {
            // Format as integer if effectively whole
            write!(f, "{}", self.0 as i32)
        } else {
            // Default float formatting
            write!(f, "{}", self.0)
        }
    }
}

impl FromStr for Fixed {
    type Err = std::num::ParseFloatError; // Use standard error type

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Return Result, let caller decide on default if needed
        s.parse().map(Fixed)
    }
}

impl From<&str> for Fixed {
    fn from(value: &str) -> Self {
        Fixed::from_str(value).unwrap_or(Fixed(0.0))
    }
}
