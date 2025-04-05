use std::io;
use std::path::PathBuf;
use crate::bank_parser::BankParserError;
use crate::bank_path::BankPathError;
use clap::Parser;
use regex::Error as RegexError;

pub mod bank_parser;
pub mod bank_path;

/// A simple CLI tool to validate and resign StarCraft II bank files.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Filepath to the bank file (.SC2Bank)
    #[arg(value_name = "BANK_PATH")]
    pub bank_path: String,

    /// Override bank name (if different from file name without extension)
    #[arg(short = 'n', long)]
    pub bank_name: Option<String>,

    /// Override author handle (e.g., 1-S2-1-AUTHID)
    #[arg(short = 'a', long = "author")]
    pub author_handle: Option<String>,

    /// Override player handle (e.g., 2-S2-1-PLAYER-HANDLE)
    #[arg(short = 'p', long = "player",)]
    pub player_handle: Option<String>,

    /// Write the computed signature back to the file if it differs
    #[arg(short = 'w', long = "write", action)]
    pub write: bool,
}


pub type AppResult<T> = Result<T, AppError>;


#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("XML Parsing Error")]
    XmlReaderError(#[from] xml::reader::Error),

    #[error("IO Error: {0}")]
    IoError(#[from] io::Error),

    #[error("Bank Path Error: {0}")]
    BankPathError(#[from] BankPathError),

    #[error("Bank Content Error: {0}")]
    BankParseError(#[from] BankParserError),

    #[error("Signature tag not found or missing in bank file")]
    SignatureNotFound,

    #[error("Regex Error")]
    RegexError(#[from] RegexError),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf)
}
