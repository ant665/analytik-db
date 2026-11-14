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
pub struct Schema {
    schema: SchemaRef,
    batches_number: usize,
    rows_number: usize,
}

impl Display for Schema {
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

fn record_batches_schema_inspection(
    reader: impl RecordBatchReader,
) -> Result<Schema, AnalytikError> {
    let batch_schema = reader.schema();
    let mut rows_counter: usize = 0;
    let mut batches_counter: usize = 0;
    for batch in reader {
        let current_batch = &batch?;
        batches_counter += 1;
        rows_counter += current_batch.num_rows();
    }
    Ok(Schema {
        schema: batch_schema,
        batches_number: batches_counter,
        rows_number: rows_counter,
    })
}

pub fn inspect(args: &InspectArgs) -> Result<Schema, AnalytikError> {
    let filename: String = args.file_name.clone();
    let extension = Path::new(&args.file_name)
        .extension()
        .and_then(OsStr::to_str)
        .ok_or_else(|| AnalytikError::InvalidFileExtension(args.file_name.clone()))?
        .to_ascii_lowercase();

    match extension.as_str() {
        "csv" => {
            let reader = csv_to_reader(filename, args.infer_rows)?;
            record_batches_schema_inspection(reader)
        }
        "parquet" => {
            if args.metadata {
                let metadata = parquet_to_metadata(&filename)?;
                println!("{:?}", metadata);
            }
            let reader = parquet_to_reader(&filename)?;
            record_batches_schema_inspection(reader)
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
            metadata: false,
        })
        .unwrap();

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
            metadata: false,
        })
        .unwrap();

        assert_eq!(result.batches_number, 3);
        assert_eq!(result.rows_number, 5);
        assert_eq!(result.schema.fields().len(), 3);
    }

    #[test]
    fn inspect_file_not_found() {
        let file_name: String = String::from("file_not_found.csv");

        let result = inspect(&InspectArgs {
            file_name,
            infer_rows: 10,
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
            metadata: false,
        });

        match result {
            Err(AnalytikError::InvalidFileType(error)) => assert_eq!(error, String::from("txt")),
            Err(error) => panic!("expected InvalidFileType error, got: {error:?}"),
            Ok(_) => panic!("Expected to fail but succeeded"),
        }
    }
}
