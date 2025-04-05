use std::path::Path;
use crate::{AppResult};

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
    pub fn new(full_path:String, real_name:Option<String>) -> AppResult<Self>{
        let (bank_name, author_handle, player_handle) = {
            let path = Path::new(&full_path);

            let bank_name = if let Some(real_name) = real_name {
                real_name
            } else {
                path
                    .file_name()
                    .ok_or(BankPathError::BankNotFound)?
                    .to_str()
                    .ok_or(BankPathError::InvalidBankFileName)?
                    .trim_end_matches(".SC2Bank").to_string()
            };

            // The game_id is expected to be the name of the parent directory.
            let author_handle = path
                .parent()
                .and_then(|p| p.file_name())
                .ok_or(BankPathError::MissingAuthorHandle)?
                .to_str()
                .ok_or(BankPathError::InvalidAuthorHandle)?.to_string();

            // The player_id is expected to be the parent of the directory containing the game_id.
            let player_handle = path
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent())
                .and_then(|p| p.file_name())
                .ok_or(BankPathError::MissingPlayerHandle)?
                .to_str()
                .ok_or(BankPathError::InvalidPlayerHandle)?.to_string();
            (
                bank_name,
                author_handle,
                player_handle,
            )
        };
        Ok(BankPath {
            full_path,
            bank_name,
            author_handle,
            player_handle,
        })
    }
}
