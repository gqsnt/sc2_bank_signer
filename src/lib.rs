use clap::Parser;
use crate::bank_parser::BankParserError;
use crate::bank_path::BankPathError;
use regex::Error as RegexError;

pub mod bank_path;
pub mod bank_parser;



/// A simple CLI tool to resign a bank file
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Filepath to the bank file
    pub bank_path: String,

    /// Real bank file name in key (if modified in path)
    #[arg(short('n'), long)]
    pub bank_name: Option<String>,

    /// Author handle (if not in path)
    #[arg(short, long("author"))]
    pub author_handle: Option<String>,

    /// Player handle (if not in path)
    #[arg(short, long("player"))]
    pub player_handle: Option<String>,


    /// Flag to actually replace the signature in the file
    #[arg(short = 'w', long = "write", action)] // Added flag
    pub write: bool,
}


pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug)]
pub enum AppError{
    XmlReaderError(xml::reader::Error),
    IoError(std::io::Error),
    BankPathError(BankPathError),
    BankParseError(BankParserError),
    SignatureNotFound,
    RegexError(RegexError),
}

macro_rules! impl_app_error {
    ($enum_type:ident, $error_type:ty) => {
        impl From<$error_type> for AppError {
            fn from(err: $error_type) -> Self {
                AppError::$enum_type(err)
            }
        }
    };
}

impl_app_error!(XmlReaderError, xml::reader::Error);
impl_app_error!(IoError, std::io::Error);
impl_app_error!(BankPathError, BankPathError);
impl_app_error!(BankParseError, BankParserError);
impl_app_error!(RegexError, RegexError);


impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::XmlReaderError(e) => write!(f, "XML Reader Error: {}", e),
            AppError::IoError(err) => write!(f, "IO Error: {}", err),
            AppError::BankPathError(err) => write!(f, "Bank Path Error: {}", err),
            AppError::BankParseError(err) => write!(f, "Bank Parse Error: {}", err),
            AppError::SignatureNotFound => write!(f, "Signature tag not found in bank file"),
            AppError::RegexError(err) => write!(f, "Regex Error: {}", err),
        }
    }
}



