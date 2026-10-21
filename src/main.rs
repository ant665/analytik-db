use clap::Parser;
mod cli;
use arrow::{array::{Array, Float64Array, GenericByteArray, Int64Array, PrimitiveArray, StringArray, StringArrayType}, datatypes::{Float64Type, GenericStringType, Int64Type}};
use cli::Cli;

fn demo_db() {
    // Create Int64Array, Float84Array and StringArray
    let int64_arr: PrimitiveArray<Int64Type> = Int64Array::from(vec![1, 2, 3, 4, 5]);
    println!("INT64 ARRAY INFO:");
    println!("Length: {}", int64_arr.len());
    println!("{:?}", int64_arr);

    let float64_arr: PrimitiveArray<Float64Type> = Float64Array::from(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    println!("FLOAT64 ARRAY INFO:");
    println!("Length: {}", float64_arr.len());
    println!("{:?}", float64_arr);

    let string_arr: GenericByteArray<GenericStringType<i32>> = StringArray::from(vec!["Hello", "World", "!"]);
    println!("STRING ARRAY INFO:");
    println!("Length: {}", string_arr.len());
    println!("{:?}", string_arr);

}

fn main() {
    let cli = Cli::parse();
    if cli.command == "demo-db" {
        demo_db();
    }
}
