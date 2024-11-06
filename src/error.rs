use std::path::PathBuf;

use thiserror::Error;

// errors
#[derive(Debug, Error)]
pub enum Errors {
    #[error(transparent)]
    Validation(#[from] ValidationError),

    #[error(transparent)]
    Rename(#[from] RenameError),

    #[error("I/O error occurred: {0}, File: {1}")]
    Io(std::io::Error, PathBuf),
}

#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("InputFolder is not a Directory: {0}.")]
    InputPathIsNotDirectory(PathBuf),

    #[error("png does not exist in the {0} folder")]
    NoPngFilesInSubDir(PathBuf),
}

#[derive(Debug, Error)]
pub enum RenameError {
    #[error("Regex error: {0}")]
    RegexError(#[from] regex::Error),
}

impl Errors {
    pub fn from_io_error(error: std::io::Error, path: PathBuf) -> Self {
        Errors::Io(error, path)
    }
}

// Custom error type to encapsulate multiple validation errors
#[derive(Debug, Error)]
pub struct MultipleErrors {
    errors: Vec<Errors>,
}

impl MultipleErrors {
    pub fn new(errors: Vec<Errors>) -> Self {
        Self { errors }
    }
}

impl std::fmt::Display for MultipleErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Errors occurred:\n")?;
        for error in &self.errors {
            writeln!(f, "  - {}", error)?;
        }
        Ok(())
    }
}

impl IntoIterator for MultipleErrors {
    type Item = Errors;
    type IntoIter = std::vec::IntoIter<Errors>;

    fn into_iter(self) -> Self::IntoIter {
        self.errors.into_iter()
    }
}
