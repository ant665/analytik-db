use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalytikError {
    #[error("Column '{0}' not found")]
    ColumnNotFound(String),

    #[error("Invalid column type: {0}")]
    InvalidColumnType(String),

    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("Invalid file extension: {0}")]
    InvalidFileExtension(String),

    #[error("Metadata inspection is not supported for {0} files")]
    MetadataUnsupported(String),

    #[error("Arrow Operation failed: {0}")]
    Arrow(#[from] arrow::error::ArrowError),

    #[error("I/O operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parquet operation failed: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("Invalid file type: {0}")]
    InvalidFileType(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("The name of the dataset cannot be empty")]
    EmptyName,

    #[error("The name length should be between 1 and 30 characters")]
    WrongSizeName,

    #[error("The name should only contain alphanumeric characters with dashes and/or underscores")]
    InvalidCharacters,

}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DatasetError {
    #[error("Invalid file extension: {0}")]
    InvalidFileExtension(String),

    #[error("Validation Error: {0}")]
    Validation(#[from] ValidationError),
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CatalogError {
    #[error("There is already a dataset with the same name in this catalog")]
    DuplicateDatasetName,

    #[error("No dataset {0} found in catalog")]
    DatasetNotFound(String),

    #[error("Validation Error: {0}")]
    Validation(#[from] ValidationError),
}