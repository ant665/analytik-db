use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalytikError {
    #[error("Column '{0}' not found")]
    ColumnNotFound(String),

    #[error("Invalid column type: {0}")]
    InvalidColumnType(String),

    #[error("Arrow Operation failed: {0}")]
    Arrow(#[from] arrow::error::ArrowError),

    #[error("I/O operation failed: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parquet operation failed: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),
}