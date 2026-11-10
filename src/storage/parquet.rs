use std::fs::File;

use parquet::arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder};

use crate::errors::AnalytikError;

pub fn parquet_to_reader(path: String) -> Result<ParquetRecordBatchReader, AnalytikError> {
    let file = File::open(&path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let reader = builder.with_batch_size(2).build()?;
    Ok(reader)
}