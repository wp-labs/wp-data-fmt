# wp-data-fmt

![CI](https://github.com/wp-labs/wp-data-fmt/workflows/CI/badge.svg)
[![codecov](https://codecov.io/gh/wp-labs/wp-data-fmt/graph/badge.svg?token=6SVCXBHB6B)](https://codecov.io/gh/wp-labs/wp-data-fmt)


`wp-data-fmt` is the formatting layer that powers WarpParse style connectors.
It turns `wp_model_core::model::DataRecord` instances into text so they can be
sent to logs, key/value stores, SQL databases, or snapshot tests.  The crate
provides concrete formatters (JSON, CSV, KV, raw, ProtoText, SQL) as well as a
`FormatType` enum that makes it easy to pick a formatter at runtime from a
`TextFmt` definition.

## Highlights

- Works directly with the strongly typed `wp_model_core` data model.
- Consistent escaping rules across JSON, CSV, KV, ProtoText, SQL, and raw text
  outputs.
- Helpers such as `SqlInsert` for generating `INSERT`, batch inserts, table
  schemas, and UPSERT statements from existing records.
- Snapshot-friendly output that is covered by integration tests for each
  supported format.

## Installation

Add the crate through Cargo:

```toml
[dependencies]
wp-data-fmt = "0.1"
wp-model-core = "0.7"
```

## Quick start

```rust
use chrono::NaiveDateTime;
use std::net::{IpAddr, Ipv4Addr};
use wp_data_fmt::{DataFormat, FormatType};
use wp_model_core::model::{fmt_def::TextFmt, DataField, DataRecord};

let record = DataRecord {
    items: vec![
        DataField::from_ip("ip", IpAddr::V4(Ipv4Addr::new(192, 168, 1, 2))),
        DataField::from_time(
            "time",
            NaiveDateTime::parse_from_str("2019-08-06 12:12:19", "%Y-%m-%d %H:%M:%S").unwrap(),
        ),
        DataField::from_chars("http/request", "GET /nginx-logo.png HTTP/1.1"),
        DataField::from_digit("http/status", 200),
    ],
};

let fmt = FormatType::from(&TextFmt::Json);
let line = fmt.format_record(&record);
assert_eq!(
    line,
    r#"{"ip":"192.168.1.2","time":"2019-08-06 12:12:19","http/request":"GET /nginx-logo.png HTTP/1.1","http/status":200}"#
);
```

Because every formatter implements the `DataFormat` trait, the same snippet can
be reused for CSV (`FormatType::from(&TextFmt::Csv)`), KV, raw, ProtoText, or
custom SQL encodings.

## SQL helpers

When you already have a `DataRecord` named `record` (and possibly a
`Vec<DataRecord>` called `records`) from your ingestion pipeline, `SqlInsert`
implements `DataFormat` and adds convenience methods:

```rust
use wp_data_fmt::SqlInsert;

let sql = SqlInsert::new_with_json("nginx_logs").format_record(&record);
// INSERT INTO "nginx_logs" ("ip", "time", ...) VALUES (...);

let batch_sql = SqlInsert::new_with_json("nginx_logs").format_batch(&records);
let schema = SqlInsert::new_with_json("nginx_logs").generate_create_table(&records);
let upsert = SqlInsert::new_with_json("nginx_logs").format_upsert(&record, &["ip", "time"]);
```

## Development

```bash
cargo fmt
cargo clippy
cargo test
```

This repository follows the Elastic License 2.0 (see `Cargo.toml` for details).
