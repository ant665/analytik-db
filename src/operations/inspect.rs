use arrow::array::RecordBatch;

use crate::{cli::InspectArgs, errors::AnalytikError, operations::schema, storage::csv_to_reader, utils::sample_record_batch};



pub fn inspect(args: &InspectArgs) -> Result<RecordBatch, AnalytikError> {
    let mut reader = csv_to_reader(args.file_name.clone());
    while let Some(batch) = reader.next() {
        let _ = schema(Some(&batch.unwrap()));
        
    };
    Ok(sample_record_batch())
}