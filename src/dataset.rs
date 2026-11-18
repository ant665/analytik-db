use std::{ffi::OsStr, path::{Path, PathBuf}};

use crate::errors::DatasetError::{self, InvalidCharacters};

#[derive(Debug, Clone, PartialEq, Eq)]
enum FileFormat {
    Parquet,
    Csv,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dataset {
    name: String,
    path: PathBuf,
    format: FileFormat,
}

impl Dataset {
    pub fn new(name: String, file_path: String) -> Result<Self, DatasetError> {
        Self::validate_name(&name)?;
        let path: PathBuf = Self::validate_path(file_path)?;
        let format: FileFormat = Self::get_format_from_file(&path)?;
        Ok(Dataset {
            name, 
            path, 
            format 
        })
    }

    fn validate_name(name: &str) -> Result<(), DatasetError> {
        if name.is_empty() {
            return Err(DatasetError::EmptyName);
        }
        if name.len() > 30 {
            return Err(DatasetError::WrongSizeName);
        }
        let has_invalid_character = name.chars().any(|character| {
            !(
                character.is_ascii_alphanumeric()
                || character == '-'
                || character == '_'
            )
        });
        if has_invalid_character {
            return Err(InvalidCharacters);
        }
        Ok(())
    }

    fn validate_path(file_path: String) -> Result<PathBuf, DatasetError> {
        let path = Path::new(&file_path);
        if !path.exists() {
            return Err(DatasetError::FileNotFound(file_path));
        }
        if !path.is_file() {
            return Err(DatasetError::InvalidFileType(file_path));
        }
        Ok(path.to_owned())
    }

    fn get_format_from_file(path: &Path) -> Result<FileFormat, DatasetError> {
        let extension = path
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| DatasetError::InvalidFileExtension(path.display().to_string()))?
            .to_ascii_lowercase();
        match extension.as_str() {
            "csv" => Ok(FileFormat::Csv),
            "parquet" => Ok(FileFormat::Parquet),
            other => Err(DatasetError::InvalidFileExtension(other.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fixture_path;

    #[test]
    fn dataset_creation() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let dataset = Dataset::new("test_dataset".to_string(), file_path.display().to_string()).unwrap();
        assert_eq!(
            dataset, 
            Dataset {
                name: "test_dataset".to_string(),
                path: file_path,
                format: FileFormat::Csv
            }
        );
    }

    #[test]
    fn validate_name_successes() {
        assert_eq!(Dataset::validate_name("test").unwrap(), ());
        assert_eq!(Dataset::validate_name("_test_").unwrap(), ());
        assert_eq!(Dataset::validate_name("-test-").unwrap(), ());
        assert_eq!(Dataset::validate_name("TesT94").unwrap(), ());
        assert_eq!(Dataset::validate_name("TesT-_000").unwrap(), ());
    }

    #[test]
    fn validate_name_errors() {
        assert_eq!(Dataset::validate_name("").unwrap_err(), DatasetError::EmptyName);
        assert_eq!(Dataset::validate_name("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap_err(), DatasetError::WrongSizeName);
        assert_eq!(Dataset::validate_name("test!").unwrap_err(), DatasetError::InvalidCharacters);
        assert_eq!(Dataset::validate_name("test test").unwrap_err(), DatasetError::InvalidCharacters);
    }

    #[test]
    fn validate_path_success() {
        assert_eq!(Dataset::validate_path(
            fixture_path("events.csv").to_string_lossy().into_owned()).unwrap(),
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv")
        );
    }

    #[test]
    fn validate_path_errors() {
        assert_eq!(
            Dataset::validate_path("unknown_file".to_string()).unwrap_err(),
            DatasetError::FileNotFound("unknown_file".to_string())
        );
        assert_eq!(
            Dataset::validate_path(env!("CARGO_MANIFEST_DIR").to_string()).unwrap_err(),
            DatasetError::InvalidFileType(env!("CARGO_MANIFEST_DIR").to_string())
        );
    }

    #[test]
    fn get_format_from_file_successes() {
        let csv_valid_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let parquet_valid_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.parquet");
        assert_eq!(Dataset::get_format_from_file(&csv_valid_path).unwrap(), FileFormat::Csv);
        assert_eq!(Dataset::get_format_from_file(&parquet_valid_path).unwrap(), FileFormat::Parquet);
    }

    #[test]
    fn get_format_from_file_error() {
        let json_valid_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.json");
        assert_eq!(Dataset::get_format_from_file(&json_valid_path).unwrap_err(), DatasetError::InvalidFileExtension("json".to_string()));
    }
}
