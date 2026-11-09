use std::fs::File;
use parquet::arrow::ArrowWriter;

use crate::{cli::ConvertArgs, errors::AnalytikError, storage::csv_to_reader};

pub fn convert(args: &ConvertArgs) -> Result<(), AnalytikError>{
    let mut reader = csv_to_reader(args.input.clone(), 10)?;
    let mut dest: File = File::create(args.output.clone())?;

    let mut writer: ArrowWriter<&mut File> = ArrowWriter::try_new(&mut dest, reader.schema(), None)?;

    while let Some(Ok(batch)) = reader.next() {
        writer.write(&batch)?;
    };
    writer.close()?;
    Ok(())
}