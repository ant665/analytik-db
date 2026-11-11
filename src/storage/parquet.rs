use std::{fs::File, sync::Arc};

use parquet::{arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder}, file::metadata::ParquetMetaData};

use crate::errors::AnalytikError;

pub fn get_reader_builder_from_path(path: &String) -> Result<ParquetRecordBatchReaderBuilder<File>, AnalytikError> {
    let file = File::open(path)?;
    Ok(ParquetRecordBatchReaderBuilder::try_new(file)?)
}

pub fn parquet_to_metadata(path: &String) -> Result<Arc<ParquetMetaData>, AnalytikError> {
    let builder = get_reader_builder_from_path(path)?;
    Ok(Arc::clone(builder.metadata()))
}

pub fn parquet_to_reader(path: &String) -> Result<ParquetRecordBatchReader, AnalytikError> {
    let builder = get_reader_builder_from_path(path)?;
    let reader = builder.with_batch_size(2).build()?;
    Ok(reader)
}