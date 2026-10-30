use std::{fs::File, sync::Arc};

use arrow::csv::{Reader, ReaderBuilder, infer_schema_from_files};

pub fn csv_to_reader(path: String) -> Reader<File> {
    let file = File::open(&path).unwrap();
    let file_schema = Arc::new(infer_schema_from_files(
        &vec![path],
        b',',
        Some(10),
        true,
    ).unwrap());

    ReaderBuilder::new(file_schema)
        .with_batch_size(2)
        .with_header(true)
        .build(file)
        .unwrap()
}