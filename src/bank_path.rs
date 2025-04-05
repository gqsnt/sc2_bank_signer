use std::fmt;
use crate::{AppError, AppResult, Args};
use std::path::{Path, PathBuf};

#[derive(Debug, thiserror::Error, Clone)]
pub enum BankPathError {
    #[error("Invalid bank file name: '{0}'")]
    InvalidBankFileName(String),
    #[error("Bank file not found or not a file: '{0}'")]
    BankNotFound(PathBuf),
    #[error("Could not determine Author Handle from path structure near '{0}'")]
    MissingAuthorHandle(PathBuf),
    #[error("Could not determine Player Handle from path structure near '{0}'")]
    MissingPlayerHandle(PathBuf),
    #[error("Invalid UTF-8 encoding in path component near '{0}'")]
    InvalidPathEncoding(PathBuf),
}


#[derive(Debug, Clone)]
pub struct BankPath {
    pub full_path: PathBuf,
    pub bank_name: String,
    pub author_handle: String,
    pub player_handle: String,
}

impl BankPath {
    pub fn new(args: &Args) -> AppResult<Self> {
        let path = PathBuf::from(&args.bank_path);

        if !path.is_file() {
            return Err(AppError::BankPathError(BankPathError::BankNotFound(path)));
        }
        let canonical_path = path.canonicalize().unwrap_or(path.clone());


        // --- Bank Name Extraction ---
        let bank_name = match &args.bank_name {
            Some(name) => name.clone(),
            None => path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .map(|s| s.to_string())
                .ok_or_else(|| BankPathError::InvalidBankFileName(path.display().to_string()))?,
        };


        // --- Handle Extraction Helper ---
        // Extracts parent dir name as String, handling potential errors
        let get_parent_dir_name = |p: &Path, err_missing: fn(PathBuf) -> BankPathError| -> Result<String, BankPathError> {
            p.parent()
                .and_then(|parent| parent.file_name())
                .and_then(|name| name.to_str())
                .map(|s| s.to_string())
                .ok_or_else(|| err_missing(p.to_path_buf())) // Use closure for lazy path cloning
        };

        // --- Author Handle Extraction ---
        let author_handle = match &args.player_handle {
            Some(handle) => handle.clone(),
            None => get_parent_dir_name(&path, BankPathError::MissingPlayerHandle)?
        };

        // --- Player Handle Extraction ---
        let player_handle = match &args.author_handle {
            Some(handle) => handle.clone(),
            None => {
                let parent_path = path.parent()
                    .and_then(|p|p.parent())
                    .ok_or_else(|| BankPathError::MissingAuthorHandle(path.clone()))?;
                get_parent_dir_name(parent_path, BankPathError::MissingAuthorHandle)?
            }
        };


        Ok(BankPath {
            full_path: canonical_path,
            bank_name,
            author_handle,
            player_handle,
        })
    }
}

impl fmt::Display for BankPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Bank File Details:")?;
        writeln!(f, "  Bank Name:     '{}'", self.bank_name)?; // Renamed label slightly
        writeln!(f, "  Author Handle: {}", self.author_handle)?;
        writeln!(f, "  Player Handle: {}", self.player_handle)?;
        // Use path.display() for correct OS-specific path formatting
        write!(f,   "  Full Path:     {}", self.full_path.display())
    }
}