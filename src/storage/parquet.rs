use std::{fs::File, sync::Arc};

use parquet::{
    arrow::arrow_reader::{ParquetRecordBatchReader, ParquetRecordBatchReaderBuilder},
    file::metadata::ParquetMetaData,
};

use crate::errors::AnalytikError;

pub fn get_reader_builder_from_path(
    path: &String,
) -> Result<ParquetRecordBatchReaderBuilder<File>, AnalytikError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::fixture_path;

    #[test]
    fn parquet_metadata_reports_file_shape() {
        let path = fixture_path("events.parquet")
            .to_string_lossy()
            .into_owned();

        let metadata = parquet_to_metadata(&path).unwrap();

        assert_eq!(metadata.file_metadata().num_rows(), 5);
        assert_eq!(metadata.num_row_groups(), 1);
        assert_eq!(metadata.file_metadata().schema_descr().num_columns(), 3);
    }

    #[test]
    fn parquet_reader_streams_expected_batch_sizes() {
        let path = fixture_path("events.parquet")
            .to_string_lossy()
            .into_owned();
        let reader = parquet_to_reader(&path).unwrap();

        let batch_sizes = reader
            .map(|batch| batch.map(|batch| batch.num_rows()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert_eq!(batch_sizes, vec![2, 2, 1]);
    }
}
