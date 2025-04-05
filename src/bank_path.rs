use std::path::Path;
use crate::{AppResult, Args};

#[derive(Debug)]
pub enum BankPathError{
    InvalidBankFileName,
    BankNotFound,
    MissingAuthorHandle,
    MissingPlayerHandle,
    InvalidAuthorHandle,
    InvalidPlayerHandle,
}

impl std::fmt::Display for BankPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BankPathError::InvalidBankFileName => write!(f, "Invalid bank file name"),
            BankPathError::BankNotFound => write!(f, "Bank not found"),
            BankPathError::MissingAuthorHandle => write!(f, "Missing author Handle"),
            BankPathError::MissingPlayerHandle => write!(f, "Missing player Handle"),
            BankPathError::InvalidAuthorHandle => write!(f, "Invalid author Handle"),
            BankPathError::InvalidPlayerHandle => write!(f, "Invalid player Handle"),
        }
    }
}



#[derive(Debug, Clone)]
pub struct BankPath {
    pub full_path: String,
    pub bank_name: String,
    pub author_handle: String,
    pub player_handle: String,
}

impl BankPath{
    pub fn new(args:&Args) -> AppResult<Self>{
        let path = Path::new(&args.bank_path);
        let bank_name = args.bank_name.clone().unwrap_or(
            path
                .file_name()
                .ok_or(BankPathError::BankNotFound)?
                .to_str()
                .ok_or(BankPathError::InvalidBankFileName)?
                .trim_end_matches(".SC2Bank").to_string()
        );



        let author_handle = args.author_handle.clone().unwrap_or(
            path
                .parent()
                .and_then(|p| p.file_name())
                .ok_or(BankPathError::MissingAuthorHandle)?
                .to_str()
                .ok_or(BankPathError::InvalidAuthorHandle)?.to_string()
        );

        let player_handle = args.player_handle.clone().unwrap_or(
            path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.file_name())
                .ok_or(BankPathError::MissingPlayerHandle)?
                .to_str()
                .ok_or(BankPathError::InvalidPlayerHandle)?.to_string()
        );
        Ok(BankPath {
            full_path:args.bank_path.clone(),
            bank_name,
            author_handle,
            player_handle,
        })
    }
}


impl std::fmt::Display for BankPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Bank File Details:")?;
        writeln!(f, "  Bank File Name: '{}'", self.bank_name)?;
        writeln!(f, "  Author Handle:   {}", self.author_handle)?;
        writeln!(f, "  Player Handle:   {}", self.player_handle)?;
        write!(f, "  Full Path:       {}", self.full_path)
    }
}