use arrow::{array::{Array, Float64Array, GenericByteArray, Int64Array, PrimitiveArray, RecordBatch, StringArray}, datatypes::{DataType, Field, Float64Type, GenericStringType, Int64Type, Schema}};
use clap::Parser;
use std::sync::Arc;

mod cli;
use cli::Cli;

fn demo_db() -> RecordBatch {
    // Create Schema with three Fields
    // Define types and nullability of columns
    let schema: Schema = Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("weight", DataType::Float64, true),
        Field::new("name", DataType::Utf8, false),
    ]);

    // Build a RecordBatch with 3 arrays

    let int64_arr: PrimitiveArray<Int64Type> = Int64Array::from(vec![0, 1, 2, 3, 4]);
    println!("INT64 ARRAY INFO:");
    println!("Length: {}", int64_arr.len());
    println!("{:?}", int64_arr);

    let float64_arr: PrimitiveArray<Float64Type> = Float64Array::from(vec![Some(45.8), Some(69.2), Some(55.8), None, Some(78.1)]);
    println!("FLOAT64 ARRAY INFO:");
    println!("Length: {}", float64_arr.len());
    println!("{:?}", float64_arr);

    let string_arr: GenericByteArray<GenericStringType<i32>> = StringArray::from(vec!["Jacques", "Marine", "Eliott", "Patrick", "Natalie"]);
    println!("STRING ARRAY INFO:");
    println!("Length: {}", string_arr.len());
    println!("{:?}", string_arr);

    let batch = RecordBatch::try_new(
        Arc::new(schema),
        vec![Arc::new(int64_arr), Arc::new(float64_arr), Arc::new(string_arr)],
    ).unwrap();

    // Display the len of rows and columns    
    println!("RECORD BATCH:");
    println!(": {:?}", batch.schema());
    println!("Number of columns: {}", batch.num_columns());
    println!("Number of rows: {}", batch.num_rows());

    batch
}

fn main() {
    let cli = Cli::parse();
    if cli.command == "demo-db" {
        demo_db();
    }
}

#[cfg(test)]
mod tests {
    use super::demo_db;

    #[test]
    fn demo_db_display() {
        let batch = demo_db();
        assert_eq!(batch.num_columns(), 3);
        assert_eq!(batch.num_rows(), 5);
    }
}