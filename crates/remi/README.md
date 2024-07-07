# 🐻‍❄️🪚 `azalia-remi`
The **azalia-remi** crate defines a union-like enum structure for possible [`remi::StorageService`](https://docs.rs/remi) and configuration files without implementing them into the library since it'll create too much fuss, so this is here to combat it (and not to repeat it everytime)

This uses Cargo's crate features to implicitilly allow you to pick out which Remi-based crates to implement into your applications. You can use the `features = ["all"]` in your Cargo.toml's definition of `azalia-remi` to include all crates.

## Usage
```rust,ignore
// > Cargo.toml:
//
// [dependencies]
// azalia-remi = { version = "0.1", features = ["all"], registry = "noelware" }
// tokio = { version = "*", features = ["full"] }

use azalia_remi::{StorageService, Config, remi::StorageService as _, fs::Config as FSConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::Filesystem(FSConfig::default().with_directory("/data"));
    config.init().await?; // initialize the fs version of remi

    // do whatever you want

    Ok(())
}
```

## Crate Features
### `file-format` (disabled by default)
Enables the [`infer`](https://docs.rs/infer) and [`file-format`](https://docs.rs/file-format) crates to allow to look over multiple file formats to detect its content-type. This implicitilly enables the `file-format` feature in [`remi-fs`](https://docs.rs/remi-fs).

### `async-std` (disabled by default)
Enables the use of [`async-std`](https://docs.rs/async-std)'s `fs` module instead of [`tokio`](https://docs.rs/tokio)'s `fs` module. This implicitilly enables the `async-std` feature in [`remi-fs`](https://docs.rs/remi-fs).

### `serde_json` (disabled by default)
Enables the use to detect JSON files when resolving content types. This implicitilly enables the `serde_json` feature in [`remi-fs`](https://docs.rs/remi-fs).

### `serde_yaml` (disabled by default)
Enables the use to detect YAML files when resolving content types. This implicitilly enables the `serde_yaml` feature in [`remi-fs`](https://docs.rs/remi-fs).

### `tracing` (disabled by default)
Enables the use of [`tracing`](https://docs.rs/tracing) for logging and instrumentation. This will add spans to each method on the [`StorageService`](https://docs.rs/remi) trait when enabled. This implicitilly enables the `tracing` feature in [`remi-fs`](https://docs.rs/remi-fs), [`remi-s3`](https://docs.rs/remi-s3), [`remi-gridfs`](https://docs.rs/remi-gridfs), and [`remi-azure`](https://docs.rs/remi-azure) crates.

### `gridfs` (disabled by default)
Enables [`remi-gridfs`](https://docs.rs/remi-gridfs) in the `StorageService` enum of this crate.

### `azure` (disabled by default)
Enables [`remi-azure`](https://docs.rs/remi-azure) in the `StorageService` enum of this crate.

### `serde` (disabled by default)
Enables the use of [`serde`](https://docs.rs/serde) to serialize and deserialize configuration structs that each Remi-based crate. This will enable the use of `serde` on the `Config` enum of this crate and implicitilly enable the `serde` feature in [`remi-fs`](https://docs.rs/remi-fs), [`remi-s3`](https://docs.rs/remi-s3), [`remi-gridfs`](https://docs.rs/remi-gridfs), and [`remi-azure`](https://docs.rs/remi-azure) crates.

### `log` (disabled by default)
Enables the use of [`log`](https://docs.rs/log) to provide logging statements that might be useful. This will implicitilly enable the `log` feature in [`remi-fs`](https://docs.rs/remi-fs), [`remi-s3`](https://docs.rs/remi-s3), [`remi-gridfs`](https://docs.rs/remi-gridfs), and [`remi-azure`](https://docs.rs/remi-azure) crates.

### `all` (disabled by default)
Enables the `gridfs`, `azure`, `gcs`, `s3`, and `fs` features.

### `s3` (disabled by default)
Enables [`remi-s3`](https://docs.rs/remi-s3) in the `StorageService` enum of this crate.

### `fs` (disabled by default)
Enables [`remi-fs`](https://docs.rs/remi-fs) in the `StorageService` enum of this crate.
