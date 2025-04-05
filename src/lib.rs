use clap::Parser;
use crate::bank_parser::BankParserError;
use crate::bank_path::BankPathError;

pub mod bank_path;
pub mod bank_parser;



/// A simple CLI tool to resign a bank file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Filepath to the bank file
    pub bank_path: String,

    /// Real bank file name (if modified in path)
    #[arg(short, long)]
    pub name: Option<String>,
}


pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError{
    XmlReaderError(xml::reader::Error),
    IoError(std::io::Error),
    BankPathError(BankPathError),
    BankParseError(BankParserError),
}

impl From<xml::reader::Error> for AppError {
    fn from(err: xml::reader::Error) -> Self {
        AppError::XmlReaderError(err)
    }
}

impl From<BankParserError> for AppError {
    fn from(err: BankParserError) -> Self {
        AppError::BankParseError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err)
    }
}

impl From<BankPathError> for AppError {
    fn from(err: BankPathError) -> Self {
        AppError::BankPathError(err)
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::XmlReaderError(e) => write!(f, "XML Reader Error: {}", e),
            AppError::IoError(err) => write!(f, "IO Error: {}", err),
            AppError::BankPathError(err) => write!(f, "Bank Path Error: {}", err),
            AppError::BankParseError(err) => write!(f, "Bank Parse Error: {}", err),
        }
    }
}



