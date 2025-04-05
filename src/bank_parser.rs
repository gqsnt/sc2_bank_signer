use crate::bank_parser::attribute::Attribute;
use crate::bank_parser::key::Key;
use crate::bank_parser::section::Section;
use crate::bank_parser::value_element::ValueElement;
use crate::bank_path::BankPath;
use crate::{AppError, AppResult, Args};
use regex::Regex;
use sha1::{Digest, Sha1};
use std::fs;
use xml::reader::XmlEvent;
use xml::EventReader;

pub mod attribute;
pub mod fixed;
pub mod flag;
pub mod key;
pub mod section;
pub mod value_element;

#[derive(Debug, Clone, thiserror::Error)]
pub enum BankParserError {
    #[error("Section tag missing 'name' attribute")]
    SectionTagMissingName,
    #[error("Key tag missing 'name' attribute")]
    KeyTagMissingName,
}


#[derive(Debug, Clone)]
pub struct BankParser {
    pub bank_path: BankPath,
    pub sections: Vec<Section>,
    pub current_signature: Option<String>,
    pub signature: String,
}
impl BankParser {
    /// Parses the bank file, calculates the signature, and returns a BankParser instance.
    pub fn new(args: &Args) -> AppResult<Self> {
        let bank_path = BankPath::new(args)?;
        let file_content = fs::read_to_string(&bank_path.full_path)?;

        let reader = std::io::Cursor::new(file_content);
        let parser = EventReader::new(reader);

        let mut sections: Vec<Section> = Vec::new();
        let mut current_signature: Option<String> = None;
        let mut current_section: Option<Section> = None;
        let mut current_key: Option<Key> = None;

        for event in parser {
            match event? {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    let tag_name = name.local_name.as_str();
                    match tag_name {
                        "Section" => {
                            if current_section.is_some() {
                                log::error!("Section already opened");
                            }
                            current_section = Some(Section {
                                name:  attributes
                                    .iter()
                                    .find(|attr| attr.name.local_name == "name")
                                    .map(|attr| attr.value.clone())
                                    .ok_or(BankParserError::SectionTagMissingName)?,
                                keys: Vec::new(),
                            });
                        }
                        "Key" => {
                            if current_key.is_some(){
                                log::error!("Key already opened");
                            }
                            if current_section.is_some() {
                                current_key = Some(Key {
                                    name:  attributes
                                        .iter()
                                        .find(|attr| attr.name.local_name == "name")
                                        .map(|attr| attr.value.clone())
                                        .ok_or(BankParserError::KeyTagMissingName)?,
                                    values: Vec::new(),
                                });
                            } else {
                                // Log potentially invalid structure
                                log::warn!("Found Key tag outside of a Section context.");
                            }
                        }
                        "Signature" => {
                            current_signature = attributes
                                .iter()
                                .find(|attr| attr.name.local_name == "value")
                                .map(|attr| attr.value.clone());
                        }
                        _ => {
                            if let Some(key) = current_key.as_mut() {
                                key.values.push(ValueElement {
                                    tag_name: tag_name.to_string(),
                                    attributes: attributes
                                        .iter()
                                        .map(|attr| Attribute::from_xml_attribute(&attr.name.local_name, &attr.value))
                                        .collect(),
                                });
                            }
                        }
                    }
                }
                XmlEvent::EndElement { name } => {
                    match name.local_name.as_str() {
                        "Key" => {
                            if let Some(mut key) = current_key.take() {
                                // Sort ValueElements within the key *before* adding to section
                                key.values.sort_by(|a, b| a.tag_name.cmp(&b.tag_name));
                                if !key.values.is_empty() {
                                    if let Some(section) = current_section.as_mut() {
                                        section.keys.push(key);
                                    } else {
                                        log::error!("Finished Key processing but no active Section!");
                                    }
                                }
                            }
                        }
                        "Section" => {
                            if let Some(mut section) = current_section.take() {
                                // Sort Keys within the section *before* adding to global list
                                section.keys.sort_by(|a, b| a.name.cmp(&b.name));
                                if !section.keys.is_empty() {
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

    /// Replaces the signature value in the original bank file content.
    /// Assumes the caller has already verified that replacement is desired.
    pub fn replace_signature(&self) -> AppResult<()> {
        if self.current_signature.is_none() {
            log::warn!("Attempted to replace signature, but no <Signature> tag was found during initial parsing.");
            return Err(AppError::SignatureNotFound);
        }

        log::info!(
            "Attempting to replace signature in file: {}",
            self.bank_path.full_path.display() // Use display()
        );
        let file_path = &self.bank_path.full_path;

        let content = fs::read_to_string(file_path)?;

        // Use a static regex or lazy_static for slight performance gain if called often
        // static SIG_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r#"(<Signature\s+value=")([^"]*)("\s*/>)"#).unwrap());
        // For now, compile it each time is fine.
        let re = Regex::new(r#"(<Signature\s+value=")([^"]*)("\s*/>)"#)?;

        // Construct the replacement text using the *newly computed* signature
        let replacement_string = format!("${{1}}{}${{3}}", self.signature);

        let new_content = re.replacen(&content, 1, replacement_string); // Returns String

        if new_content == content {
            log::warn!("Signature replacement resulted in no changes. File not overwritten.");
            return Ok(());
        }

        fs::write(file_path, new_content.as_bytes())?;

        log::info!("Successfully replaced signature in {}", file_path.display());
        Ok(())
    }

    /// Compares the signature found in the file (if any) with the newly computed one.
    pub fn compare_signature(&self) -> bool {
        match &self.current_signature {
            Some(file_sig) => {
                if file_sig == &self.signature {
                    log::info!("Signature MATCHES: {}", self.signature);
                    true
                } else {
                    log::warn!("Signature MISMATCH:");
                    log::warn!("  File:     {}", file_sig);
                    log::warn!("  Computed: {}", self.signature);
                    false
                }
            }
            None => {
                log::info!("No existing signature found in the XML file.");
                log::info!("Computed signature: {}", self.signature);
                false
            }
        }
    }

    /// Computes the signature string based on the parsed bank data.
    fn compute_signature(&mut self) {
        let mut pitems: Vec<String> = Vec::new();

        pitems.push(self.bank_path.author_handle.clone());
        pitems.push(self.bank_path.player_handle.clone());
        pitems.push(self.bank_path.bank_name.clone());

        for section in &self.sections {
            pitems.push(section.name.clone());
            for key in &section.keys {
                pitems.push(key.name.clone());
                for value_element in &key.values {
                    pitems.push(value_element.tag_name.clone());

                    let mut attrs_to_sort: Vec<(String, &Attribute)> = value_element
                        .attributes
                        .iter()
                        .map(|attr| (attr.name(), attr))
                        .collect();

                    attrs_to_sort.sort_unstable_by(|(name_a, _), (name_b, _)| name_a.cmp(name_b));
                    for (attr_name, attr) in attrs_to_sort {
                        pitems.push(attr_name);
                        if !attr.is_text() {
                            pitems.push(attr.value());
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