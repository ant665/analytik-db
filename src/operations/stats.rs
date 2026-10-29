use arrow::{
    array::{AsArray, Float64Array, RecordBatch}, compute::{max, min, sum}, datatypes::Float64Type,
};
use std::fmt::{Display, Formatter};

use crate::cli::StatsArgs;
use crate::errors::AnalytikError;
use crate::utils::sample_record_batch;


pub struct StatsData {
    row_count: usize,
    null_count: usize,
    max_value: f64,
    min_value: f64,
    sum: f64,
    average:f64,
}

impl Display for StatsData {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f, 
            "Count: {}\nNull Count: {}\nMax Value: {}\nMin Value: {}\nSum: {}\nAverage: {}\n",
            self.row_count, self.null_count, self.max_value, self.min_value, self.sum, self.average
        )
    }
}

pub fn stats(opt_batch: Option<&RecordBatch>, args: &StatsArgs) -> Result<RecordBatch, AnalytikError> {
    let batch = match opt_batch {
        Some(batch) => batch,
        None => &sample_record_batch(),
    };

    let batch_col = batch.column_by_name(&args.column)
        .ok_or_else(|| AnalytikError::ColumnNotFound(args.column.clone()))?;


    batch_col.as_any().downcast_ref::<Float64Array>().ok_or_else(|| AnalytikError::InvalidColumnType(args.column.clone()))?;

    let max_value = max(&batch_col.as_primitive::<Float64Type>())
        .ok_or_else(|| AnalytikError::InvalidColumnType(batch_col.data_type().to_string()))?;
    let min_value = min(&batch_col.as_primitive::<Float64Type>())
        .ok_or_else(|| AnalytikError::InvalidColumnType(batch_col.data_type().to_string()))?;
    let sum = sum(&batch_col.as_primitive::<Float64Type>())
        .ok_or_else(|| AnalytikError::InvalidColumnType(batch_col.data_type().to_string()))?;
    let average = sum / (batch_col.len() as f64 - batch_col.null_count() as f64);

    let stats_data: StatsData = StatsData {
        row_count: batch_col.len(),
        null_count: batch_col.null_count(),
        max_value,
        min_value,
        sum,
        average,
    };
    println!("{}", stats_data);
    Ok(batch.to_owned())
}

#[cfg(test)]
mod tests {
    use arrow::{
        datatypes as arrow_schema,
        array::record_batch,
    };

use super::*;

    #[test]
    fn stats_no_record_batch() {
        assert_eq!(
            stats(None, &StatsArgs { column: String::from("weight") }).unwrap(),
            sample_record_batch()
        )
    }

    #[test]
    fn stats_with_record_batch() {
        let exp_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2]),
            ("weight", Float64, [10.5, 20.7, 30.4]),
            ("name", Utf8, ["Hugin", "Munin", "Odin"])
        ).expect("Record Batch could not be created");
        assert_eq!(
            stats(Some(&exp_record_batch), &StatsArgs { column: String::from("weight") }).unwrap(),
            exp_record_batch
        )
    }

    #[test]
    fn stats_invalid_type_column() {
        let invalid_col_type_name = String::from("name");
        let input_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2]),
            ("weight", Float64, [10.5, 20.7, 30.4]),
            ("name", Utf8, ["Hugin", "Munin", "Odin"])
        ).expect("Record Batch could not be created");
        assert_eq!(
            stats(Some(&input_record_batch), &StatsArgs { column: invalid_col_type_name.clone() }).err().unwrap().to_string(),
            AnalytikError::InvalidColumnType(invalid_col_type_name).to_string()
        )
    }
 
    #[test]
    fn stats_unknown_column() {
        let input_record_batch: RecordBatch = record_batch!(
            ("id", Int64, [0,1,2]),
            ("weight", Float64, [10.5, 20.7, 30.4]),
            ("name", Utf8, ["Hugin", "Munin", "Odin"])
        ).expect("Record Batch could not be created");
        assert_eq!(
            stats(Some(&input_record_batch), &StatsArgs { column: String::from("unknown") }).err().unwrap().to_string(),
            AnalytikError::ColumnNotFound("unknown".to_string()).to_string()
        )
    }
}