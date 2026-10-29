use arrow::array::RecordBatch;

use crate::{errors::AnalytikError, utils::sample_record_batch};


pub fn schema(opt_batch: Option<&RecordBatch>) -> Result<RecordBatch, AnalytikError> {
    let batch = match opt_batch {
        Some(batch) => batch,
        None => &sample_record_batch(),
    };
    inspect_batch(batch);
    Ok(batch.to_owned())
}

fn inspect_batch(batch: &RecordBatch) -> () {
    println!("\
        RECORD BATCH\n\
        ------------\n\
        Rows: {}\n\
        Columns: {}\n\
        ------------", batch.num_rows(), batch.num_columns()
    );
    println!("\
        Schema:\n\
        {: <15} | {: <15} | {: <15}\n", "name", "type", "nullable"
    );
    for field in batch.schema().fields() {
        println!("{: <15} | {: <15} | {: <15}", field.name(), field.data_type().to_string(), field.is_nullable());
    }
    println!("\
        ------------\n\
        Memory:\n\
        Arrays: {} bytes
    ", batch.get_array_memory_size());
}

#[cfg(test)]
mod tests {
    use arrow::{
        datatypes as arrow_schema,
        array::record_batch,
    };

use super::*;

    #[test]
    fn schema_no_record_batch() {
        assert_eq!(
            schema(None).unwrap(),
            sample_record_batch()
        )
    }

    #[test]
    fn schema_with_record_batch() {
        let exp_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2])
        ).expect("Record Batch could not be created");
        assert_eq!(
            schema(Some(&exp_record_batch)).unwrap(),
            exp_record_batch
        )
    }
}