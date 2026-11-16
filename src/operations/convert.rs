use parquet::arrow::ArrowWriter;
use std::fs::File;

use crate::{cli::ConvertArgs, errors::AnalytikError, storage::csv_to_reader};

pub fn convert(args: &ConvertArgs) -> Result<(), AnalytikError> {
    let reader = csv_to_reader(args.input.clone(), args.infer_rows, args.batch_size)?;
    let mut dest: File = File::create(args.output.clone())?;

    let mut writer: ArrowWriter<&mut File> =
        ArrowWriter::try_new(&mut dest, reader.schema(), None)?;

    for batch in reader {
        writer.write(&batch?)?;
    }
    writer.close()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use arrow::array::RecordBatchReader;

    use super::*;
    use crate::{storage::parquet_to_reader, utils::fixture_path};

    #[test]
    fn convert_preserves_rows_and_schema() {
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("events.parquet");

        convert(&ConvertArgs {
            input: fixture_path("events.csv").to_string_lossy().into_owned(),
            output: output.to_string_lossy().into_owned(),
            infer_rows: 10,
            batch_size: 2,
        })
        .unwrap();

        let reader = parquet_to_reader(&output.to_string_lossy().into_owned()).unwrap();
        let schema = reader.schema();
        let row_count = reader
            .map(|batch| batch.map(|batch| batch.num_rows()))
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .into_iter()
            .sum::<usize>();

        assert_eq!(row_count, 5);
        assert_eq!(schema.fields().len(), 3);
        assert_eq!(schema.field(0).name(), "event_id");
        assert_eq!(schema.field(1).name(), "user_id");
        assert_eq!(schema.field(2).name(), "value");
    }

    #[test]
    fn convert_propagates_malformed_csv_error() {
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("malformed.parquet");

        let result = convert(&ConvertArgs {
            input: fixture_path("malformed.csv").to_string_lossy().into_owned(),
            output: output.to_string_lossy().into_owned(),
            infer_rows: 10,
            batch_size: 2,
        });

        assert!(matches!(result, Err(AnalytikError::Arrow(_))));
    }

    #[test]
    fn convert_propagates_missing_input_error() {
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("missing.parquet");

        let result = convert(&ConvertArgs {
            input: fixture_path("missing.csv").to_string_lossy().into_owned(),
            output: output.to_string_lossy().into_owned(),
            infer_rows: 10,
            batch_size: 2,
        });

        assert!(matches!(
            result,
            Err(AnalytikError::Io(error))
                if error.kind() == std::io::ErrorKind::NotFound
        ));
    }
}
