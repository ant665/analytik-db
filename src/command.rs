use arrow::{array::{Float64Array, GenericByteArray, Int64Array, PrimitiveArray, RecordBatch, Scalar, StringArray}, compute::filter_record_batch, datatypes::{DataType, Field, Float64Type, GenericStringType, Int64Type, Schema}};
use arrow::util::pretty::pretty_format_batches;
use arrow::compute::kernels::cmp;
use std::sync::Arc;

use crate::cli::FilterArgs;

pub fn filter(opt_batch: Option<&RecordBatch>, args: &FilterArgs) -> RecordBatch {
    let batch = match opt_batch {
        Some(batch) => batch,
        None => &sample_record_batch(),
    };
    let batch_col = batch
            .column_by_name(&args.column)
            .expect("No column found for the given name");

    let bool_masks = cmp::gt(batch_col, &Scalar::new(Int64Array::from_value(args.gt, 1))).unwrap();
    let filtered_batch = filter_record_batch(batch, &bool_masks).unwrap();
    println!("{}", pretty_format_batches(&[filtered_batch.clone()]).unwrap());
    filtered_batch
}

pub fn schema(opt_batch: Option<&RecordBatch>) -> () {
    let batch = match opt_batch {
        Some(batch) => batch,
        None => &sample_record_batch(),
    };
    inspect_batch(batch);
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

pub fn sample_record_batch() -> RecordBatch {
    let schema: Schema = Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("weight", DataType::Float64, true),
        Field::new("name", DataType::Utf8, false),
    ]);
    let int64_arr: PrimitiveArray<Int64Type> = Int64Array::from(vec![0, 1, 2, 3, 4]);
    let float64_arr: PrimitiveArray<Float64Type> = Float64Array::from(vec![Some(45.8), Some(69.2), Some(55.8), None, Some(78.1)]);
    let string_arr: GenericByteArray<GenericStringType<i32>> = StringArray::from(vec!["Jacques", "Marine", "Eliott", "Patrick", "Natalie"]);
    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(int64_arr), Arc::new(float64_arr), Arc::new(string_arr)],
    ).unwrap();
    batch
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_record_batch_content() {
        let batch = sample_record_batch();
        assert_eq!(batch.num_columns(), 3);
        assert_eq!(batch.num_rows(), 5);
    }
}