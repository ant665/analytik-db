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

pub fn stats(opt_batch: Option<&RecordBatch>, args: &StatsArgs) -> Result<StatsData, AnalytikError> {
    let batch = match opt_batch {
        Some(batch) => batch,
        None => &sample_record_batch(),
    };

    let batch_col = batch.column_by_name(&args.column)
        .ok_or_else(|| AnalytikError::ColumnNotFound(args.column.clone()))?;


    batch_col.as_any().downcast_ref::<Float64Array>().ok_or_else(|| AnalytikError::InvalidColumnType { column: args.column.clone() })?;

    let max_value = max(&batch_col.as_primitive::<Float64Type>())
        .ok_or_else(|| AnalytikError::InvalidColumnType { column: "".to_string() })?;
    let min_value = min(&batch_col.as_primitive::<Float64Type>())
        .ok_or_else(|| AnalytikError::InvalidColumnType { column: "".to_string() })?;
    let sum = sum(&batch_col.as_primitive::<Float64Type>())
        .ok_or_else(|| AnalytikError::InvalidColumnType { column: "".to_string() })?;
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
    Ok(stats_data)
}
