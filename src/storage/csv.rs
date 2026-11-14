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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fixture_path;

    #[test]
    fn csv_reader_streams_expected_batch_sizes() {
        let reader = csv_to_reader(
            fixture_path("events.csv").to_string_lossy().into_owned(),
            10,
        )
        .unwrap();

        let batch_sizes = reader
            .map(|batch| batch.map(|batch| batch.num_rows()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(batch_sizes, vec![2, 2, 1]);
    }

    #[test]
    fn csv_reader_reports_malformed_value_after_valid_batches() {
        let reader = csv_to_reader(
            fixture_path("malformed.csv").to_string_lossy().into_owned(),
            10,
        )
        .unwrap();

        let result = reader.collect::<Result<Vec<_>, _>>();

        assert!(result.is_err());
    }
}
