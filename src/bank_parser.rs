use crate::bank_parser::attribute::Attribute;
use crate::bank_parser::key::Key;
use crate::bank_parser::section::Section;
use crate::bank_parser::value_element::ValueElement;
use crate::bank_path::BankPath;
use crate::AppResult;
use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::BufReader;
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
    pub sections: Vec<Section>, // Will be sorted later
    pub current_signature: Option<String>,
    pub signature: String,
}

impl BankParser {
    pub fn new(path: String, real_name: Option<String>) -> AppResult<Self> {
        let bank_path = BankPath::new(path, real_name)?;
        println!("Bank path: {:?}", bank_path);
        let file = File::open(bank_path.full_path.clone())?;
        let reader = BufReader::new(file);
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
                            // Finalize previous section if any
                            if let Some(section) = current_section.take() {
                                if !section.keys.is_empty() {
                                    // Only add sections with keys
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
                            // Finalize previous key if any (shouldn't happen here ideally)
                            if let Some(key) = current_key.take() {
                                if let Some(section) = current_section.as_mut() {
                                    if !key.values.is_empty() {
                                        // Only add keys with values
                                        section.keys.push(key);
                                    }
                                }
                            }
                            // Start new key
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
                                // Key outside section - ignore or error? Ignoring for now.
                                println!("Warning: Found Key outside of Section context.");
                            }
                        }
                        "Signature" => {
                            current_signature = attributes
                                .iter()
                                .find(|e| e.name.local_name == "value")
                                .map(|sig| sig.value.clone());
                        }
                        // Any other element inside a Key is a ValueElement
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
                            // else: Element is not directly inside Key, ignore for signature?
                        }
                    }
                }
                XmlEvent::EndElement { name } => {
                    match name.local_name.as_str() {
                        "Key" => {
                            if let Some(key) = current_key.take() {
                                if let Some(section) = current_section.as_mut() {
                                    if !key.values.is_empty() {
                                        // Only add keys with values
                                        // Sort value elements within the key *before* adding the key
                                        section.keys.push(key);
                                    }
                                }
                            }
                        }
                        "Section" => {
                            if let Some(section) = current_section.take() {
                                if !section.keys.is_empty() {
                                    // Only add sections with keys
                                    // Sort keys within the section *before* adding the section
                                    sections.push(section);
                                }
                            }
                        }
                        _ => {} // Ignore other end tags
                    }
                }
                _ => {} // Ignore other events like Characters, CData, etc. for signature
            }
        }
        let mut bank_data = BankParser {
            bank_path,
            sections,
            current_signature,
            signature: String::new(),
        };
        bank_data.compute_signature();
        Ok(bank_data)
    }

    pub fn compare_signature(&self) -> bool {
        if let Some(current_signature) = &self.current_signature {
            if current_signature != &self.signature {
                println!(
                    "Signature mismatch: {} vs {}",
                    current_signature, self.signature
                );
                false
            } else {
                println!("Signature matches: {}", self.signature);
                true
            }
        } else {
            println!("No signature found in the XML file.");
            println!("New signature: {}", self.signature);
            false
        }
    }

    pub fn compute_signature(&mut self) {
        let mut sections = self
            .sections
            .clone()
            .into_iter()
            .map(|mut s| {
                s.keys
                    .iter_mut()
                    .for_each(|key| key.values.sort_by(|a, b| a.tag_name.cmp(&b.tag_name)));
                s.keys.sort_by(|a, b| a.name.cmp(&b.name));
                s
            })
            .collect::<Vec<_>>();
        sections.sort_by(|a, b| a.name.cmp(&b.name));
        println!("{:#?}", sections);
        let mut pitems: Vec<String> = vec![
            self.bank_path.author_handle.clone(),
            self.bank_path.player_handle.clone(),
            self.bank_path.bank_name.clone(),
        ];
        for section in sections {
            pitems.push(section.name);
            for key in section.keys {
                pitems.push(key.name);
                for value_element in key.values {
                    pitems.push(value_element.tag_name);
                    // BTreeMap ensures attributes are iterated in sorted order by key name
                    for attr in value_element.attributes {
                        let (attr_name, attr_value) = attr.to_name_value();
                        pitems.push(attr_name);
                        // Conditional value add based on Python logic:
                        // Skip adding the *value* if the *key name* matches the special text key.
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
