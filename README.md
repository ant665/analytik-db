# Analytik DB

Analytik DB is a Rust learning project for building a small Arrow-backed analytical data engine. It currently supports CSV inspection, Parquet inspection, streaming CSV-to-Parquet conversion, filtering, projection, and numeric statistics.

## Verification

The automated suite was last run on 2026-11-16:

```text
running 34 tests
test result: ok. 34 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Run it with:

```bash
cargo test
```

## Memory-bounded CSV-to-Parquet conversion

The converter processes the CSV reader as an iterator and writes each Arrow `RecordBatch` directly to `ArrowWriter`. It does not collect the complete input in memory.

The benchmark inputs used eight mixed-type columns: two identifiers, an epoch timestamp, a floating-point value, an integer reading count, a boolean, a categorical string, and a nullable string.

Measurements were taken from the optimized binary with CSV batch size 8000. Peak memory is the Linux `VmHWM` resident-memory high-water mark for the conversion process.

| CSV rows | CSV size | Parquet size | Peak RSS | Elapsed | Output rows | Row groups |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 100,000 | 6.7 MiB | 4.4 MiB | 25.9 MiB | 51 ms | 100,000 | 1 |
| 1,000,000 | 68.0 MiB | 35.3 MiB | 49.2 MiB | 351 ms | 1,000,000 | 1 |
| 5,000,000 | 344.3 MiB | 176.5 MiB | 60.9 MiB | 1.67 s | 5,000,000 | 5 |

The input grew from 6.7 MiB to 344.3 MiB while peak resident memory grew from 25.9 MiB to 60.9 MiB. The writer flushes completed Parquet row groups instead of retaining the entire dataset. Output row counts were verified through Parquet footer metadata.

These are single-run development measurements, not general performance claims. Results depend on the hardware, allocator, operating system, schema, batch size, and Parquet writer configuration.

### Reproduce the measurement

Build once so Cargo and compilation memory are excluded:

```bash
cargo build --release
```

If GNU `time` is installed:

```bash
/usr/bin/time -v target/release/analytik-db convert \
  --input /tmp/analytik-db-data/events-1m.csv \
  --output /tmp/analytik-db-data/events-1m.parquet \
  --infer-rows 10 \
  --batch-size 8000
```

Read `Maximum resident set size (kbytes)` from the output and divide it by 1024 to obtain MiB.

Validate the produced row count without scanning the Parquet data pages:

```bash
target/release/analytik-db inspect \
  --file-name /tmp/analytik-db-data/events-1m.parquet \
  --metadata
```

## Measurement environment

```text
Commit: 5ea8dde plus the current uncommitted batch-size and metadata-inspection work
Rust: rustc 1.99.1 (58cf8f9c3 2026-10-13)
OS: Linux 7.3.0-4-cachyos x86_64
System memory: 31 GiB
Date: 2026-11-16
```
