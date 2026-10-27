use arrow::array::RecordBatch;
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

    batch_col.data_type().is_numeric()
        .ok_or_else(|| AnalytikError::InvalidColumnType { column: args.column.clone() })?;

    let stats_data: StatsData = StatsData {
        row_count: batch_col.len(),
        null_count: batch_col.null_count(),
        max_value: 0.0,
        min_value: 0.0,
        sum: 0.0,
        average: 0.0,
    };
    println!("{}", stats_data);
    Ok(stats_data)
}
