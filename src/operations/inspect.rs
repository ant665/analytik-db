use std::{ffi::OsStr, fmt::{Display, Formatter}, path::Path};

use arrow::{array::RecordBatchReader, datatypes::SchemaRef};

use crate::{cli::InspectArgs, errors::AnalytikError, storage::{csv_to_reader, parquet_to_metadata, parquet_to_reader}};

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
            schema_formatted.push_str(&format!("{: <15} | {: <15} | {: <15}\n", 
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
            "name", "type", "nullable",
            schema_formatted
        )
    }
}


fn record_batches_schema_inspection(reader: impl RecordBatchReader) -> Result<Schema, AnalytikError>  {
    let batch_schema = reader.schema();
    let mut rows_counter: usize = 0;
    let mut batches_counter: usize = 0;
    for batch in reader {
        let current_batch = &batch?;
        batches_counter += 1;
        rows_counter += current_batch.num_rows();
    };
    Ok(Schema {
        schema: batch_schema,
        batches_number: batches_counter,
        rows_number: rows_counter
    })
}


pub fn inspect(args: &InspectArgs) -> Result<Schema, AnalytikError> {
    let filename: String = args.file_name.clone();
    match Path::new(&filename).extension().and_then(OsStr::to_str).map(|ext| ext.to_lowercase()).unwrap().as_str() {
        "csv" => {
            let reader = csv_to_reader(filename, args.infer_rows)?;
            record_batches_schema_inspection(reader)
        },
        "parquet" => {
            if args.metadata {
                let metadata = parquet_to_metadata(&filename)?;
                println!("{:?}", metadata);
            }
            let reader = parquet_to_reader(&filename)?;
            record_batches_schema_inspection(reader)
        },
        other => return Err(AnalytikError::InvalidFileType(other.to_string())),
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn inspect_file_not_found() {
        let file_name: String = String::from("file_not_found");
        let result = inspect(&InspectArgs { file_name, infer_rows: 10, metadata: false });
        match result {
            Err(AnalytikError::Io(error)) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound),
            Err(error) => panic!("expected IO error, got: {error:?}"),
            Ok(_) => panic!("Expected to fail but succeeded")
        }
    }
}
