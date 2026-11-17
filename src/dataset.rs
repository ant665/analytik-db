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