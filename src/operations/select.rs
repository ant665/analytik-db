use arrow::{
    array::RecordBatch,
    util::pretty::pretty_format_batches,
};
use std::slice::from_ref;

use crate::cli::SelectArgs;
use crate::errors::AnalytikError;
use crate::utils::sample_record_batch;

pub fn projection(opt_batch: Option<&RecordBatch>, args: &SelectArgs) -> Result<RecordBatch, AnalytikError> {
    let batch = match opt_batch {
        Some(batch) => batch,
        None => &sample_record_batch(),
    };
    let mut col_idx: Vec<usize> = Vec::new();
    let schema = batch.schema();
    for col in &args.columns {
        col_idx.push(schema.index_of(&col).expect(&AnalytikError::ColumnNotFound(col.to_string()).to_string()));
    }
    let projected = batch.project(&col_idx).unwrap();
    println!("{}", pretty_format_batches(from_ref(&projected)).unwrap());
    Ok(projected)
}
