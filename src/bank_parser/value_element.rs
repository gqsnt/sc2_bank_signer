use crate::bank_parser::attribute::Attribute;

#[derive(Debug, Clone)]
pub struct ValueElement {
    pub tag_name: String,
    pub attributes: Vec<Attribute>,
}