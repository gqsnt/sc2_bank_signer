use crate::bank_parser::value_element::ValueElement;

#[derive(Debug, Clone)]
pub struct Key {
    pub name: String,
    pub values: Vec<ValueElement>,
}
