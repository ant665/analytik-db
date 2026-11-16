use std::{
    ffi::OsStr,
    fmt::{Display, Formatter},
    path::Path,
};

use arrow::{array::RecordBatchReader, datatypes::SchemaRef};

use crate::{
    cli::InspectArgs,
    errors::AnalytikError,
    storage::{csv_to_reader, parquet_to_metadata, parquet_to_reader},
};

#[derive(Debug, PartialEq)]
pub struct BatchInspection {
    schema: SchemaRef,
    batches_number: usize,
    rows_number: usize,
}

#[derive(Debug, PartialEq)]
pub struct ParquetColumnInspection {
    name: String,
    physical_type: String,
    compression: String,
    values_number: i64,
    compressed_size: i64,
    uncompressed_size: i64,
}

#[derive(Debug, PartialEq)]
pub struct ParquetRowGroupInspection {
    index: usize,
    rows_number: i64,
    compressed_size: i64,
    uncompressed_size: i64,
    columns: Vec<ParquetColumnInspection>,
}

#[derive(Debug, PartialEq)]
pub struct ParquetMetadataInspection {
    rows_number: i64,
    row_groups_number: usize,
    columns_number: usize,
    row_groups: Vec<ParquetRowGroupInspection>,
}

#[derive(Debug, PartialEq)]
pub enum Inspection {
    Batches(BatchInspection),
    ParquetMetadata(ParquetMetadataInspection),
}

impl Display for BatchInspection {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let fields = self.schema.fields();
        let mut schema_formatted: String = String::new();
        for field in fields {
            schema_formatted.push_str(&format!(
                "{: <15} | {: <15} | {: <15}\n",
                field.name(),
                field.data_type().to_string(),
                field.is_nullable()
            ));
        }

        write!(
            f,
            "\
            STATS\n\
            ------------\n\
            Batches: {}\n\
            Rows: {}\n\
            Columns: {}\n\
            ------------\n\n\
            SCHEMA\n\
            ------------\n\
            {: <15} | {: <15} | {: <15}\n\
            ______________________________________________\n\
            {}
            ",
            self.batches_number,
            self.rows_number,
            fields.len(),
            "name",
            "type",
            "nullable",
            schema_formatted
        )
    }
}

impl Display for ParquetMetadataInspection {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "PARQUET METADATA")?;
        writeln!(f, "----------------")?;
        writeln!(f, "Rows: {}", self.rows_number)?;
        writeln!(f, "Row groups: {}", self.row_groups_number)?;
        writeln!(f, "Columns: {}", self.columns_number)?;

        for row_group in &self.row_groups {
            writeln!(f)?;
            writeln!(f, "ROW GROUP {}", row_group.index)?;
            writeln!(f, "------------")?;
            writeln!(f, "Rows: {}", row_group.rows_number)?;
            writeln!(f, "Compressed size: {} bytes", row_group.compressed_size)?;
            writeln!(
                f,
                "Uncompressed size: {} bytes",
                row_group.uncompressed_size
            )?;
            writeln!(
                f,
                "{:<15} | {:<12} | {:<12} | {:>8} | {:>10} | {:>12}",
                "name", "type", "compression", "values", "compressed", "uncompressed"
            )?;

            for column in &row_group.columns {
                writeln!(
                    f,
                    "{:<15} | {:<12} | {:<12} | {:>8} | {:>10} | {:>12}",
                    column.name,
                    column.physical_type,
                    column.compression,
                    column.values_number,
                    column.compressed_size,
                    column.uncompressed_size
                )?;
            }
        }

        Ok(())
    }
}

impl Display for Inspection {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Self::Batches(inspection) => inspection.fmt(f),
            Self::ParquetMetadata(inspection) => inspection.fmt(f),
        }
    }
}

fn record_batches_schema_inspection(
    reader: impl RecordBatchReader,
) -> Result<BatchInspection, AnalytikError> {
    let batch_schema = reader.schema();
    let mut rows_counter: usize = 0;
    let mut batches_counter: usize = 0;
    for batch in reader {
        let current_batch = &batch?;
        batches_counter += 1;
        rows_counter += current_batch.num_rows();
    }
    Ok(BatchInspection {
        schema: batch_schema,
        batches_number: batches_counter,
        rows_number: rows_counter,
    })
}

fn parquet_metadata_inspection(
    filename: &String,
) -> Result<ParquetMetadataInspection, AnalytikError> {
    let metadata = parquet_to_metadata(filename)?;
    let row_groups = metadata
        .row_groups()
        .iter()
        .enumerate()
        .map(|(index, row_group)| ParquetRowGroupInspection {
            index,
            rows_number: row_group.num_rows(),
            compressed_size: row_group.compressed_size(),
            uncompressed_size: row_group.total_byte_size(),
            columns: row_group
                .columns()
                .iter()
                .map(|column| ParquetColumnInspection {
                    name: column.column_path().string(),
                    physical_type: column.column_type().to_string(),
                    compression: column.compression().to_string(),
                    values_number: column.num_values(),
                    compressed_size: column.compressed_size(),
                    uncompressed_size: column.uncompressed_size(),
                })
                .collect(),
        })
        .collect();

    Ok(ParquetMetadataInspection {
        rows_number: metadata.file_metadata().num_rows(),
        row_groups_number: metadata.num_row_groups(),
        columns_number: metadata.file_metadata().schema_descr().num_columns(),
        row_groups,
    })
}

pub fn inspect(args: &InspectArgs) -> Result<Inspection, AnalytikError> {
    let filename: String = args.file_name.clone();
    let extension = Path::new(&args.file_name)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| AnalytikError::InvalidFileExtension(args.file_name.clone()))?
        .to_ascii_lowercase();

    match extension.as_str() {
        "csv" => {
            if args.metadata {
                return Err(AnalytikError::MetadataUnsupported("CSV".to_owned()));
            }
            let reader = csv_to_reader(filename, args.infer_rows, args.batch_size)?;
            Ok(Inspection::Batches(record_batches_schema_inspection(
                reader,
            )?))
        }
        "parquet" => {
            if args.metadata {
                return Ok(Inspection::ParquetMetadata(parquet_metadata_inspection(
                    &filename,
                )?));
            }
            let reader = parquet_to_reader(&filename)?;
            Ok(Inspection::Batches(record_batches_schema_inspection(
                reader,
            )?))
        }
        other => return Err(AnalytikError::InvalidFileType(other.to_string())),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::utils::fixture_path;

    #[test]
    fn inspect_parquet_file() {
        let file_name = fixture_path("events.parquet")
            .to_string_lossy()
            .into_owned();

        let result = inspect(&InspectArgs {
            file_name,
            infer_rows: 10,
            batch_size: 2,
            metadata: false,
        })
        .unwrap();

        let Inspection::Batches(result) = result else {
            panic!("expected batch inspection");
        };

        assert_eq!(result.batches_number, 3);
        assert_eq!(result.rows_number, 5);
        assert_eq!(result.schema.fields().len(), 3);
    }

    #[test]
    fn inspect_csv_file() {
        let file_name = fixture_path("events.csv").to_string_lossy().into_owned();

        let result = inspect(&InspectArgs {
            file_name,
            infer_rows: 10,
            batch_size: 2,
            metadata: false,
        })
        .unwrap();

        let Inspection::Batches(result) = result else {
            panic!("expected batch inspection");
        };

        assert_eq!(result.batches_number, 3);
        assert_eq!(result.rows_number, 5);
        assert_eq!(result.schema.fields().len(), 3);
    }

    #[test]
    fn inspect_parquet_metadata_returns_metadata_model() {
        let file_name = fixture_path("events.parquet")
            .to_string_lossy()
            .into_owned();

        let result = inspect(&InspectArgs {
            file_name,
            infer_rows: 10,
            batch_size: 2,
            metadata: true,
        })
        .unwrap();

        let Inspection::ParquetMetadata(result) = result else {
            panic!("expected Parquet metadata inspection");
        };

        assert_eq!(result.rows_number, 5);
        assert_eq!(result.row_groups_number, 1);
        assert_eq!(result.columns_number, 3);
        assert_eq!(result.row_groups[0].rows_number, 5);
        assert_eq!(result.row_groups[0].columns[0].name, "event_id");
        assert_eq!(result.row_groups[0].columns[0].physical_type, "INT64");
        assert_eq!(result.row_groups[0].columns[0].compression, "UNCOMPRESSED");
        assert_eq!(result.row_groups[0].columns[0].values_number, 5);
        assert!(result.row_groups[0].columns[0].compressed_size > 0);
        assert!(result.row_groups[0].columns[0].uncompressed_size > 0);
    }

    #[test]
    fn inspect_rejects_metadata_mode_for_csv() {
        let result = inspect(&InspectArgs {
            file_name: fixture_path("events.csv").to_string_lossy().into_owned(),
            infer_rows: 10,
            batch_size: 2,
            metadata: true,
        });

        assert!(matches!(
            result,
            Err(AnalytikError::MetadataUnsupported(format)) if format == "CSV"
        ));
    }

    #[test]
    fn inspect_file_not_found() {
        let file_name: String = String::from("file_not_found.csv");

        let result = inspect(&InspectArgs {
            file_name,
            infer_rows: 10,
            batch_size: 2,
            metadata: false,
        });

        match result {
            Err(AnalytikError::Io(error)) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound),
            Err(error) => panic!("expected IO error, got: {error:?}"),
            Ok(_) => panic!("Expected to fail but succeeded"),
        }
    }

    #[test]
    fn inspect_invalid_extension() {
        let file_name: String = String::from("file_invalid");

        let result = inspect(&InspectArgs {
            file_name,
            infer_rows: 10,
            batch_size: 2,
            metadata: false,
        });

        match result {
            Err(AnalytikError::InvalidFileExtension(error)) => {
                assert_eq!(error, "file_invalid".to_string())
            }
            Err(error) => panic!("expected InvalidFileExtension error, got: {error:?}"),
            Ok(_) => panic!("Expected to fail but succeeded"),
        }
    }

    #[test]
    fn inspect_invalid_file_type() {
        let file_name: String = String::from("invalid_file.txt");

        let result = inspect(&InspectArgs {
            file_name,
            infer_rows: 10,
            batch_size: 2,
            metadata: false,
        });

        match result {
            Err(AnalytikError::InvalidFileType(error)) => assert_eq!(error, String::from("txt")),
            Err(error) => panic!("expected InvalidFileType error, got: {error:?}"),
            Ok(_) => panic!("Expected to fail but succeeded"),
        }
    }
}
