use std::{ffi::OsStr, path::{Path, PathBuf}};

use crate::errors::DatasetError;
use crate::validation::{validate_name, validate_path};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataFormat {
    Parquet,
    Csv,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dataset {
    name: String,
    path: PathBuf,
    format: DataFormat,
}

impl Dataset {
    pub fn new(name: String, file_path: String) -> Result<Self, DatasetError> {
        validate_name(&name)?;
        let path: PathBuf = validate_path(file_path)?;
        let format: DataFormat = Self::get_format_from_file(&path)?;
        Ok(Dataset {
            name,
            path,
            format
        })
    }

    fn get_format_from_file(path: &Path) -> Result<DataFormat, DatasetError> {
        let extension = path
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| DatasetError::InvalidFileExtension(path.display().to_string()))?
            .to_ascii_lowercase();
        match extension.as_str() {
            "csv" => Ok(DataFormat::Csv),
            "parquet" => Ok(DataFormat::Parquet),
            other => Err(DatasetError::InvalidFileExtension(other.to_string())),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn get_format(&self) -> &DataFormat {
        &self.format
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dataset_creation() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let dataset = Dataset::new("test_dataset".to_string(), file_path.display().to_string()).unwrap();
        assert_eq!(
            dataset, 
            Dataset {
                name: "test_dataset".to_string(),
                path: file_path,
                format: DataFormat::Csv
            }
        );
    }

    #[test]
    fn get_format_from_file_successes() {
        let csv_valid_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let parquet_valid_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.parquet");
        assert_eq!(Dataset::get_format_from_file(&csv_valid_path).unwrap(), DataFormat::Csv);
        assert_eq!(Dataset::get_format_from_file(&parquet_valid_path).unwrap(), DataFormat::Parquet);
    }

    #[test]
    fn get_format_from_file_error() {
        let json_valid_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.json");
        assert_eq!(Dataset::get_format_from_file(&json_valid_path).unwrap_err(), DatasetError::InvalidFileExtension("json".to_string()));
    }

    #[test]
    fn get_name() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let dataset = Dataset::new("test_dataset".to_string(), file_path.display().to_string()).unwrap();

        assert_eq!(dataset.get_name(), "test_dataset");
    }

    #[test]
    fn get_path() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let dataset = Dataset::new("test_dataset".to_string(), file_path.display().to_string()).unwrap();

        assert_eq!(dataset.get_path(), &file_path);
    }

    #[test]
    fn get_format() {
        let file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures").join("events.csv");
        let dataset = Dataset::new("test_dataset".to_string(), file_path.display().to_string()).unwrap();

        assert_eq!(dataset.get_format(), &DataFormat::Csv);
    }
}
