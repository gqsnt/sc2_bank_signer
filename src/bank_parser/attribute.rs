
use crate::bank_parser::fixed::Fixed;
use crate::bank_parser::flag::Flag;


const TEXT_ATTRIBUTE: &str = "text";
const STRING_ATTRIBUTE: &str = "string";
const INTEGER_ATTRIBUTE: &str = "int";
const FIXED_ATTRIBUTE: &str = "fixed";
const FLAG_ATTRIBUTE: &str = "flag";

#[derive(Debug, Clone)]
pub enum Attribute {
    String(String),
    Int(i32),
    Fixed(Fixed),
    Flag(Flag),
    Text(String),
    Custom(String, String),
}

impl Attribute {
    pub fn is_text(&self) -> bool {
        matches!(self, Attribute::Text(_))
    }

    pub fn from_xml_attribute(name: &str, value: &str) -> Self {
        match name {
            INTEGER_ATTRIBUTE => Attribute::Int(value.parse().unwrap_or(0)),
            FIXED_ATTRIBUTE => Attribute::Fixed(Fixed::from(value)),
            FLAG_ATTRIBUTE => Attribute::Flag(Flag::from(value)),
            TEXT_ATTRIBUTE => Attribute::Text(value.to_string()),
            STRING_ATTRIBUTE => Attribute::String(value.to_string()),
            _ => Attribute::Custom(name.to_string(), value.to_string()),
        }
    }

    pub fn value(&self) -> String {
        match self {
            Attribute::Int(v) => v.to_string(),
            Attribute::Fixed(v) => v.to_string(),
            Attribute::Flag(v) => v.to_string(),
            Attribute::Text(v) => v.clone(),
            Attribute::String(v) => v.clone(),
            Attribute::Custom(_, value) => value.clone(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Attribute::Int(_) => INTEGER_ATTRIBUTE.to_string(),
            Attribute::Fixed(_) => FIXED_ATTRIBUTE.to_string(),
            Attribute::Flag(_) => FLAG_ATTRIBUTE.to_string(),
            Attribute::Text(_) => TEXT_ATTRIBUTE.to_string(),
            Attribute::String(_) => STRING_ATTRIBUTE.to_string(),
            Attribute::Custom(name, _) => name.clone(), // Borrow name here
        }
    }
}
