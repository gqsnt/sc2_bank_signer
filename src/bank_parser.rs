use std::fs;
use crate::bank_parser::attribute::Attribute;
use crate::bank_parser::key::Key;
use crate::bank_parser::section::Section;
use crate::bank_parser::value_element::ValueElement;
use crate::bank_path::BankPath;
use crate::{AppError, AppResult, Args};
use sha1::{Digest, Sha1};
use std::io::BufReader;
use regex::Regex;
use xml::reader::XmlEvent;
use xml::EventReader;

pub mod attribute;
pub mod fixed;
pub mod flag;
pub mod key;
pub mod section;
pub mod value_element;

#[derive(Debug, Clone)]
pub enum BankParserError {
    SectionTagMissingName,
    KeyTagMissingName,
}

impl std::fmt::Display for BankParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BankParserError::SectionTagMissingName => {
                write!(f, "Section tag missing 'name' attribute")
            }
            BankParserError::KeyTagMissingName => write!(f, "Key tag missing 'name' attribute"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BankParser {
    pub bank_path: BankPath,
    pub sections: Vec<Section>,
    pub current_signature: Option<String>,
    pub signature: String,
}

impl BankParser {
    pub fn new(args:&Args) -> AppResult<Self> {
        let bank_path = BankPath::new(args)?;
        let file_content = fs::read_to_string(bank_path.full_path.clone())?;
        let reader = BufReader::new(file_content.as_bytes());
        let parser = EventReader::new(reader);
        let mut sections: Vec<Section> = Vec::new();
        let mut current_signature = None::<String>;
        let mut current_section: Option<Section> = None;
        let mut current_key: Option<Key> = None;
        for e in parser {
            match e? {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    match name.local_name.as_str() {
                        "Section" => {
                            if let Some(mut section) = current_section.take() {
                                section.keys.sort_by(|a, b| a.name.cmp(&b.name));
                                if !section.keys.is_empty() {
                                    sections.push(section);
                                }
                            }
                            // Start new section
                            let section_name = attributes
                                .iter()
                                .find(|attr| attr.name.local_name == "name")
                                .map(|attr| attr.value.clone())
                                .ok_or(BankParserError::SectionTagMissingName)?;
                            current_section = Some(Section {
                                name: section_name,
                                keys: vec![],
                            });
                        }
                        "Key" => {
                            if let Some(key) = current_key.take() {
                                if let Some(section) = current_section.as_mut() {
                                    if !key.values.is_empty() {
                                        section.keys.push(key);
                                    }
                                }
                            }
                            if current_section.is_some() {
                                let key_name = attributes
                                    .iter()
                                    .find(|attr| attr.name.local_name == "name")
                                    .map(|attr| attr.value.clone())
                                    .ok_or(BankParserError::KeyTagMissingName)?;
                                current_key = Some(Key {
                                    name: key_name,
                                    values: vec![],
                                });
                            } else {
                                println!("Warning: Found Key outside of Section context.");
                            }
                        }
                        "Signature" => {
                            current_signature = attributes
                                .iter()
                                .find(|e| e.name.local_name == "value")
                                .map(|sig| sig.value.clone());
                        }
                        tag_name => {
                            if let Some(key) = current_key.as_mut() {
                                let mut value_attrs = Vec::new();
                                for attr in attributes {
                                    if let Some(attr) = Attribute::try_from_name_value(
                                        &attr.name.local_name,
                                        &attr.value,
                                    ) {
                                        value_attrs.push(attr);
                                    }
                                }
                                key.values.push(ValueElement {
                                    tag_name: tag_name.to_string(),
                                    attributes: value_attrs,
                                });
                            }
                        }
                    }
                }
                XmlEvent::EndElement { name } => {
                    match name.local_name.as_str() {
                        "Key" => {
                            if let Some(mut key) = current_key.take() {
                                if let Some(section) = current_section.as_mut() {
                                    // Sort value elements within the key *before* adding the key
                                    key.values.sort_by(|a,b| a.tag_name.cmp(&b.tag_name));
                                    if !key.values.is_empty() {
                                        // Only add keys with values
                                        section.keys.push(key);
                                    }
                                }
                            }
                        }
                        "Section" => {
                            if let Some(mut section) = current_section.take() {
                                // Sort keys within the section *before* adding the section
                                section.keys.sort_by(|a, b| a.name.cmp(&b.name));
                                if !section.keys.is_empty() {
                                    // Only add sections with keys
                                    sections.push(section);
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        if let Some(mut section) = current_section.take() {
            // Sort keys within the section *before* adding the section
            section.keys.sort_by(|a, b| a.name.cmp(&b.name));
            if !section.keys.is_empty() {
                sections.push(section);
            }
        }

        // Sort sections globally
        sections.sort_by(|a, b| a.name.cmp(&b.name));


        let mut bank_data = BankParser {
            bank_path,
            sections,
            current_signature,
            signature: String::new(),
        };

        bank_data.compute_signature();

        Ok(bank_data)
    }

    pub fn replace_signature(&self) -> AppResult<()> {
        if self.current_signature.is_none(){
            return Err(AppError::SignatureNotFound);
        }
        println!("Attempting to replace signature in file: {}", self.bank_path.full_path);
        let file_path = &self.bank_path.full_path;


        let content = fs::read_to_string(file_path)?;

        // Regex to find <Signature value="..."/>, capturing the old value part
        // It handles potential whitespace variations around 'value' and before '/>'
        let re = Regex::new(r#"(<Signature\s+value=")([^"]*)("\s*/>)"#)?;

        // Construct the replacement text using the newly computed signature
        // We use captures to keep the surrounding parts (<Signature value=" and "/>)
        let replacement_string = format!("${{1}}{}${{3}}", self.signature);

        // Perform the replacement, ensuring only the first match is replaced
        let new_content = re.replacen(&content, 1, replacement_string);


        // Write the modified content back to the file, overwriting it
        fs::write(file_path, new_content.as_bytes())?;

        println!("Successfully replaced signature in {}", file_path);
        Ok(())
    }

    pub fn compare_signature(&self) -> bool {
        if let Some(current_signature) = &self.current_signature {
            if current_signature != &self.signature {
                println!("Signature MISMATCH:");
                println!("  File:     {}", current_signature);
                println!("  Computed: {}", self.signature);
                false
            } else {
                println!("Signature MATCHES: {}", self.signature);
                true
            }
        } else {
            println!("No existing signature found in the XML file.");
            println!("Computed signature: {}", self.signature);
            false
        }
    }



    pub fn compute_signature(&mut self) {
        let mut pitems: Vec<String> = vec![
            self.bank_path.author_handle.clone(),
            self.bank_path.player_handle.clone(),
            self.bank_path.bank_name.clone(),
        ];

        for section in &self.sections {
            pitems.push(section.name.clone());
            for key in &section.keys {
                pitems.push(key.name.clone());
                for value_element in &key.values {
                    pitems.push(value_element.tag_name.clone());
                    let mut sorted_attributes = value_element.attributes.clone();
                    sorted_attributes.sort_by(|a,b| {
                        let (a_name, _) = a.to_name_value();
                        let (b_name, _) = b.to_name_value();
                        a_name.cmp(&b_name)
                    });

                    for attr in sorted_attributes {
                        let (attr_name, attr_value) = attr.to_name_value();
                        pitems.push(attr_name);
                        if !attr.is_text() {
                            pitems.push(attr_value);
                        }
                    }
                }
            }
        }
        let payload = pitems.join("");
        let mut hasher = Sha1::new();
        hasher.update(payload.as_bytes());
        let hash_result = hasher.finalize();
        self.signature = hex::encode_upper(hash_result);
    }
}
