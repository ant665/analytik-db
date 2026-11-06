use std::{fs::File, sync::Arc};

use arrow::csv::{Reader, ReaderBuilder, infer_schema_from_files};

use crate::errors::AnalytikError;

pub fn csv_to_reader(path: String, infer_rows: usize) -> Result<Reader<File>, AnalytikError> {
    let file = File::open(&path)?;
    let file_schema = Arc::new(infer_schema_from_files(
        &vec![path],
        b',',
        Some(infer_rows),
        true,
    )?);

    let reader = ReaderBuilder::new(file_schema)
        .with_batch_size(2)
        .with_header(true)
        .build(file)?;

    Ok(reader)
}