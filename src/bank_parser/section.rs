use crate::bank_parser::key::Key;

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub keys: Vec<Key>, // Will be sorted later
}