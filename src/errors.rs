use thiserror::Error;

#[derive(Debug, Error)]
pub enum AnalytikError {
    #[error("Column '{0}' not found")]
    ColumnNotFound(String),
    #[error("Invalid column type")]
    InvalidColumnType{
        column: String
    },
}