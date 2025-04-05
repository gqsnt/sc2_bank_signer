use crate::bank_parser::fixed::Fixed;
use crate::bank_parser::flag::Flag;

#[derive(Debug, Clone)]
pub enum Attribute {
    String(String),
    Int(i32),
    Fixed(Fixed),
    Flag(Flag),
    Text(String),
}

impl Attribute {
    pub fn is_text(&self) -> bool {
        matches!(self, Attribute::Text(_))
    }

    pub fn try_from_name_value(name: &str, value: &str) -> Option<Self> {
        match name {
            "int" => Some(Attribute::Int(value.parse().unwrap_or(0))),
            "fixed" => Some(Attribute::Fixed(Fixed::from(value))),
            "flag" => Some(Attribute::Flag(Flag::from(value))),
            "text" => Some(Attribute::Text(value.to_string())),
            "string" => Some(Attribute::String(value.to_string())),
            _ => None,
        }
    }

    pub fn to_name_value(&self) -> (String, String) {
        match self {
            Attribute::Int(v) => ("int".to_string(), v.to_string()),
            Attribute::Fixed(v) => ("fixed".to_string(), v.to_string()),
            Attribute::Flag(v) => ("flag".to_string(), v.to_string()),
            Attribute::Text(v) => ("text".to_string(), v.clone()),
            Attribute::String(v) => ("string".to_string(), v.clone()),
        }
    }
}