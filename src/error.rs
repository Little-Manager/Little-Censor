//! Error Module
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Word can't be empty")]
    /// When word is empty
    EmptyWord,
}
