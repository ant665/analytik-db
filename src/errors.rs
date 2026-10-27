use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalytikError {
    #[error("Column '{0}' not found")]
    ColumnNotFound(String),

    #[error("Invalid column type")]
    InvalidColumnType{
        column: String
    },

    #[error("Arrow Operation failed: {0}")]
    Arrow(#[from] arrow::error::ArrowError),
}