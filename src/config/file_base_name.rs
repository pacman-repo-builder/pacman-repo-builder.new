use derive_more::{AsRef, Deref, Display, Error, Into};
use pipe_trait::Pipe;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Base name of a path to be used in a configuration file.
#[derive(Debug, Display, Clone, Into, AsRef, Deref, Deserialize, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct FileBaseName(String);

/// Error on fail validation of a [`FileBaseName`].
#[derive(Debug, Display, Error)]
#[non_exhaustive]
pub enum ValidateError {
    #[display("Not a base name")]
    NotAName,
}

impl FileBaseName {
    /// Try converting a string into a base name.
    pub fn try_from_string(name: String) -> Result<Self, ValidateError> {
        FileBaseName::validate(&name)?;
        Ok(FileBaseName(name))
    }

    /// Validate whether a name really is a base name.
    pub fn validate(name: &str) -> Result<(), ValidateError> {
        name.pipe_as_ref(Path::file_name)
            .ok_or(ValidateError::NotAName)?
            .eq(&name.as_ref())
            .then_some(())
            .ok_or(ValidateError::NotAName)
    }
}

impl TryFrom<String> for FileBaseName {
    type Error = ValidateError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        FileBaseName::try_from_string(value)
    }
}
