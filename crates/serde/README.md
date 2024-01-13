# 🐻‍❄️🪚 `noelware-serde`
The **noelware-serde** crate provides blanket `serde` implementations for crates that don't expose any. This uses Cargo's crate features to explicitly enable which implementations you need, rather than adding them all at once.

We only provide implementations to Rust types that are most used by us, so we will probably reject most requests to add more types other than the ones listed.

> [!NOTE]
> This crate is apart of the [`core-rs`](https://github.com/Noelware/core-rs) family of crates by [Noelware, LLC.](https://noelware.org)
>
> This is the only crate that is available on [`crates.io`](https://crates.io/crates/noelware-serde) from Noelware's `core-rs` family.

## Usage
### `tracing::Level` (requires `tracing` feature)
```rust,ignore
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "noelware_serde::tracing")]
    level: tracing::Level,
}
```

### `aws_types::types::Region` (requires `aws` feature)
```rust,ignore
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "noelware_serde::aws::region")]
    region: aws_sdk_s3::types::Region
}
```
