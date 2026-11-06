use arrow::array::RecordBatch;

use crate::{cli::InspectArgs, errors::AnalytikError, storage::csv_to_reader, utils::sample_record_batch};



pub fn inspect(args: &InspectArgs) -> Result<RecordBatch, AnalytikError> {
    let mut reader = csv_to_reader(args.file_name.clone())?;
    let batch_schema = reader.schema().to_string();
    let mut rows_counter: usize = 0;
    let mut batches_counter: usize = 0;
    while let Some(batch) = reader.next() {
        let current_batch = &batch?;
        batches_counter += 1;
        rows_counter += current_batch.num_rows();
    };
    println!("{}", batch_schema);
    println!("Rows: {}", rows_counter);
    println!("Batches: {}", batches_counter);
    Ok(sample_record_batch())
}