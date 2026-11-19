use std::path::{Path, PathBuf};

use crate::errors::ValidationError;

pub fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.is_empty() {
        return Err(ValidationError::EmptyName);
    }
    if name.len() > 30 {
        return Err(ValidationError::WrongSizeName);
    }
    let has_invalid_character = name.chars().any(|character| {
        !(
            character.is_ascii_alphanumeric()
            || character == '-'
            || character == '_'
        )
    });
    if has_invalid_character {
        return Err(ValidationError::InvalidCharacters);
    }
    Ok(())
}

pub fn validate_path(file_path: String) -> Result<PathBuf, ValidationError> {
    let path = Path::new(&file_path);
    if !path.exists() {
        return Err(ValidationError::FileNotFound(file_path));
    }
    if !path.is_file() {
        return Err(ValidationError::InvalidFileType(file_path));
    }
    Ok(path.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fixture_path;

    #[test]
    fn validate_name_successes() {
        assert_eq!(validate_name("test").unwrap(), ());
        assert_eq!(validate_name("_test_").unwrap(), ());
        assert_eq!(validate_name("-test-").unwrap(), ());
        assert_eq!(validate_name("TesT94").unwrap(), ());
        assert_eq!(validate_name("TesT-_000").unwrap(), ());
    }

    #[test]
    fn validate_name_errors() {
        assert_eq!(validate_name("").unwrap_err(), ValidationError::EmptyName);
        assert_eq!(validate_name("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap_err(), ValidationError::WrongSizeName);
        assert_eq!(validate_name("test!").unwrap_err(), ValidationError::InvalidCharacters);
        assert_eq!(validate_name("test test").unwrap_err(), ValidationError::InvalidCharacters);
    }

    #[test]
    fn validate_path_success() {
        assert_eq!(validate_path(
            fixture_path("events.csv").to_string_lossy().into_owned()).unwrap(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv")
        );
    }

    #[test]
    fn validate_path_errors() {
        assert_eq!(
            validate_path("unknown_file".to_string()).unwrap_err(),
            ValidationError::FileNotFound("unknown_file".to_string())
        );
        assert_eq!(
            validate_path(env!("CARGO_MANIFEST_DIR").to_string()).unwrap_err(),
            ValidationError::InvalidFileType(env!("CARGO_MANIFEST_DIR").to_string())
        );
    }
}