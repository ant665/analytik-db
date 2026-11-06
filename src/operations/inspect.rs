use std::fmt::{Display, Formatter};

use arrow::{datatypes::SchemaRef};

use crate::{cli::InspectArgs, errors::AnalytikError, storage::csv_to_reader};

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


pub fn inspect(args: &InspectArgs) -> Result<Schema, AnalytikError> {
    let mut reader = csv_to_reader(args.file_name.clone(), args.infer_rows)?;
    let batch_schema = reader.schema();
    let mut rows_counter: usize = 0;
    let mut batches_counter: usize = 0;
    while let Some(batch) = reader.next() {
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn inspect_file_not_found() {
        let file_name: String = String::from("file_not_found");
        let result = inspect(&InspectArgs { file_name, infer_rows: 10 });
        match result {
            Err(AnalytikError::Io(error)) => assert_eq!(error.kind(), std::io::ErrorKind::NotFound),
            Err(error) => panic!("expected IO error, got: {error:?}"),
            Ok(_) => panic!("Expected to fail but succeeded")
        }
    }
}
