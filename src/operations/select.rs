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
        col_idx.push(schema.index_of(&col).map_err(|_| AnalytikError::ColumnNotFound(col.to_string()))?);
    }
    let projected = batch.project(&col_idx).unwrap();
    println!("{}", pretty_format_batches(from_ref(&projected)).unwrap());
    Ok(projected)
}

#[cfg(test)]
mod tests {
    use arrow::{
        datatypes as arrow_schema,
        array::record_batch,
    };

use super::*;

    #[test]
    fn projection_no_record_batch() {
        assert_eq!(
            projection(None, &SelectArgs { columns: vec![String::from("id"),String::from("weight"),String::from("name"),] }).unwrap(),
            sample_record_batch()
        )
    }

    #[test]
    fn projection_with_record_batch() {
        let exp_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2])
        ).expect("Record Batch could not be created");
        assert_eq!(
            projection(Some(&exp_record_batch), &SelectArgs { columns: vec![String::from("id")] }).unwrap(),
            exp_record_batch
        )
    }

    #[test]
    fn projection_only_one_column() {
        let input_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2]),
            ("weight", Float64, [10.5, 20.7, 30.4]),
            ("name", Utf8, ["Hugin", "Munin", "Odin"])
        ).expect("Record Batch could not be created");
        let exp_record_batch: RecordBatch = record_batch!(
            ("weight", Float64, [10.5, 20.7, 30.4])
        ).expect("Record Batch could not be created");
        assert_eq!(
            projection(Some(&input_record_batch), &SelectArgs { columns: vec![String::from("weight")] }).unwrap(),
            exp_record_batch
        )
    }

    #[test]
    fn projection_multiple_columns() {
        let input_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2]),
            ("weight", Float64, [10.5, 20.7, 30.4]),
            ("name", Utf8, ["Hugin", "Munin", "Odin"])
        ).expect("Record Batch could not be created");
        let exp_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2]),
            ("weight", Float64, [10.5, 20.7, 30.4])
        ).expect("Record Batch could not be created");
        assert_eq!(
            projection(Some(&input_record_batch), &SelectArgs { columns: vec![String::from("id"),String::from("weight")] }).unwrap(),
            exp_record_batch
        )
    }

    #[test]
    fn projection_unknown_column() {
        let input_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2]),
            ("weight", Float64, [10.5, 20.7, 30.4]),
            ("name", Utf8, ["Hugin", "Munin", "Odin"])
        ).expect("Record Batch could not be created");
        assert_eq!(
            projection(Some(&input_record_batch), &SelectArgs { columns: vec![String::from("unknown")] }).err().unwrap().to_string(),
            AnalytikError::ColumnNotFound("unknown".to_string()).to_string()
        )
    }
}