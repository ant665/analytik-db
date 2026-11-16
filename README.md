# Analytik DB

Analytik DB is a small Arrow-backed analytical data engine written in Rust. It is a learning and portfolio project focused on columnar data, streaming ingestion, explicit ownership, bounded memory, and testable storage boundaries.

The project currently reads CSV and Parquet, converts CSV to Parquet incrementally, inspects decoded Arrow batches, and reads Parquet footer metadata without scanning data pages. It is not intended to be a production database.

## Current capabilities

- Bounded-sample CSV schema inference
- Incremental CSV reading with configurable batch size
- Streaming CSV-to-Parquet conversion
- Incremental Parquet `RecordBatch` reading
- Arrow schema, row, and batch inspection
- Parquet footer, row-group, compression, and column-chunk inspection
- In-memory Arrow filtering, projection, and numeric statistics
- Typed I/O, Arrow, Parquet, and domain errors
- Fixture-based correctness and failure-path tests

## Architecture

```text
CLI (Clap)
 ├── inspect
 │    ├── CSV reader ────────> Arrow RecordBatches ──> batch inspection
 │    └── Parquet
 │         ├── batch reader ─> Arrow RecordBatches ──> batch inspection
 │         └── metadata ─────> footer/row-group inspection
 └── convert
      └── CSV reader ────────> ArrowWriter ──────────> Parquet
```

CSV and Parquet readers share inspection logic through Arrow's `RecordBatchReader`. Parquet metadata inspection uses the file footer and does not decode record batches.

## Requirements

- A recent Rust toolchain supporting the 2024 edition
- Cargo

The main dependencies are Apache Arrow, Parquet, Clap, and `thiserror`.

## Build and test

```bash
git clone <repository-url>
cd analytik-db

cargo build
cargo test
```

Build the optimized binary with:

```bash
cargo build --release
```

The executable is written to `target/release/analytik-db`.

## CLI usage

Show the available commands:

```bash
cargo run -- --help
```

### Inspect a CSV file

```bash
cargo run -- inspect \
  --file-name tests/fixtures/events.csv \
  --infer-rows 10 \
  --batch-size 2
```

Example output:

```text
STATS
------------
Batches: 3
Rows: 5
Columns: 3
------------

SCHEMA
------------
name            | type            | nullable
______________________________________________
event_id        | Int64           | true
user_id         | Utf8            | true
value           | Float64         | true
```

### Inspect a Parquet file by scanning batches

```bash
cargo run -- inspect \
  --file-name tests/fixtures/events.parquet \
  --batch-size 2
```

This constructs a `ParquetRecordBatchReader`, decodes batches, and reports the resulting Arrow schema, row count, and batch count.

### Inspect Parquet metadata

```bash
cargo run -- inspect \
  --file-name tests/fixtures/events.parquet \
  --metadata
```

Metadata mode reads the Parquet footer without scanning record batches. It reports:

- rows, columns, and row groups;
- physical column types;
- compression codecs;
- value counts;
- compressed and uncompressed column-chunk sizes.

`--metadata` is Parquet-specific and returns a typed error for CSV files.

### Convert CSV to Parquet

```bash
cargo run --release -- convert \
  --input tests/fixtures/events.csv \
  --output /tmp/events.parquet \
  --infer-rows 10 \
  --batch-size 8000
```

The converter consumes one CSV batch at a time and passes it directly to `ArrowWriter`. Reader errors are propagated rather than treated as the end of the input.

### Arrow operation demonstrations

The following commands currently operate on the built-in in-memory sample batch. Connecting them to registered file-backed datasets is planned work.

```bash
cargo run -- filter --column id --gt 1
cargo run -- select --columns name,id
cargo run -- stats --column weight
```

## Testing

The test suite covers:

- CSV and Parquet batch boundaries;
- malformed CSV values after valid batches;
- conversion error propagation;
- CSV-to-Parquet row and schema preservation;
- Parquet footer metadata;
- CSV and Parquet inspection;
- missing and unsupported files;
- filter row alignment and null behavior;
- projection order;
- nullable numeric statistics.

Latest local result on 2026-11-16:

```text
running 34 tests
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Test inputs live under `tests/fixtures`. Generated outputs use temporary directories so tests do not modify repository fixtures.

## Memory-bounded conversion

The conversion path does not collect all `RecordBatch` values:

```rust
for batch in reader {
    writer.write(&batch?)?;
}

writer.close()?;
```

Each CSV batch is decoded, written, and then released. `ArrowWriter` buffers the current Parquet row group and flushes completed groups rather than retaining the whole input.

### Measurements

The benchmark data used eight mixed-type columns: integer identifiers, an epoch timestamp, a floating-point value, an integer count, a boolean, a categorical string, and a nullable string.

Measurements used the optimized binary, schema inference over 10 rows, and a CSV batch size of 8000. Peak RSS is Linux's `VmHWM` resident-memory high-water mark for the conversion process.

| CSV rows | CSV size | Parquet size | Peak RSS | Elapsed | Output rows | Row groups |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 100,000 | 6.7 MiB | 4.4 MiB | 25.9 MiB | 51 ms | 100,000 | 1 |
| 1,000,000 | 68.0 MiB | 35.3 MiB | 49.2 MiB | 351 ms | 1,000,000 | 1 |
| 5,000,000 | 344.3 MiB | 176.5 MiB | 60.9 MiB | 1.67 s | 5,000,000 | 5 |

Input size grew by approximately 51 times while peak resident memory grew by approximately 2.4 times. Output row counts were verified using Parquet footer metadata.

These are single-run development measurements, not general performance claims. Results depend on hardware, filesystem caching, allocator behavior, schema, batch size, and Parquet writer configuration.

### Reproduce a measurement

Build once so Cargo and compilation memory are excluded:

```bash
cargo build --release
```

With GNU `time` installed:

```bash
/usr/bin/time -v target/release/analytik-db convert \
  --input /tmp/analytik-db-data/events-1m.csv \
  --output /tmp/analytik-db-data/events-1m.parquet \
  --infer-rows 10 \
  --batch-size 8000
```

Read `Maximum resident set size (kbytes)` and divide it by 1024 to obtain MiB. Verify the result without scanning data pages:

```bash
target/release/analytik-db inspect \
  --file-name /tmp/analytik-db-data/events-1m.parquet \
  --metadata
```

Measurement environment:

```text
Rust: rustc 1.99.1 (58cf8f9c3 2026-10-13)
OS: Linux 7.3.0-4-cachyos x86_64
System memory: 31 GiB
Date: 2026-11-16
```

## Project status and roadmap

Completed milestones:

- Arrow arrays, schemas, record batches, filtering, projection, and statistics
- Incremental CSV ingestion and schema inference
- Streaming CSV-to-Parquet conversion
- Parquet reading and metadata inspection
- Measured memory-bounded conversion

Planned milestones:

- Persistent dataset catalog
- DataFusion SQL integration
- Custom Arrow chunk store and temporal index
- Latest-at and range queries
- Compaction, concurrent ingestion, and backpressure
- Benchmarks, profiling, CI, and release documentation

## Known limitations

- Filter, projection, and statistics commands still use an in-memory demonstration batch.
- CSV metadata mode is unsupported because CSV has no footer metadata.
- Schema inference is sample-based and may infer an unsuitable type when later rows differ.
- Failed conversion may leave a partial output file.
- This project has not been designed or audited for production use.
