use derive_more::{AsRef, Deref, Display, Error, Into};
use pipe_trait::Pipe;
use serde::{Deserialize, Serialize};
use split_first_char::SplitFirstChar;

/// Name of a pacman repository.
#[derive(Debug, Display, Clone, Into, AsRef, Deref, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct RepoName(String);

/// Error on fail validation of a [`RepoName`].
#[derive(Debug, Display, Error)]
#[non_exhaustive]
pub enum ValidateError {
    #[display("Repository name was empty")]
    Empty,
    #[display("Expecting the first char to be lowercase alphabet but received {_0:?}")]
    InvalidFirstChar(#[error(not(source))] char),
    #[display("Found invalid character {_0:?}")]
    InvalidChar(#[error(not(source))] char),
}

impl RepoName {
    /// Try converting a string into a name of a pacman repository.
    pub fn try_from_string(name: String) -> Result<Self, ValidateError> {
        RepoName::validate(&name)?;
        Ok(RepoName(name))
    }

    /// Whether the name is a valid name for a pacman repository.
    pub fn validate(name: &str) -> Result<(), ValidateError> {
        let (head, tail) = name.split_first_char().ok_or(ValidateError::Empty)?;
        head.pipe(RepoName::validate_first_char)
            .map_err(ValidateError::InvalidFirstChar)?;
        tail.chars()
            .try_for_each(RepoName::validate_char)
            .map_err(ValidateError::InvalidChar)
    }

    /// Whether the character is valid as the first character of a pacman repository name.
    fn validate_first_char(char: char) -> Result<(), char> {
        char.is_ascii_lowercase().then_some(()).ok_or(char)
    }

    /// Whether the character is valid as a character of a pacman repository name.
    fn validate_char(char: char) -> Result<(), char> {
        match char {
            'a'..='z' | '0'..='9' | '-' | '_' => Ok(()),
            _ => Err(char),
        }
    }
}

impl TryFrom<String> for RepoName {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        RepoName::try_from_string(value)
    }
}
