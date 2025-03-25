# 🐻‍❄️🪚 `azalia-serde`
The **azalia-serde** crate provides blanket `serde` implementations for crates that don't expose any. This uses Cargo's crate features to explicitly enable which implementations you need, rather than adding them all at once.

We only provide implementations to Rust types that are most used by us, so we will probably reject most requests to add more types other than the ones listed.

## Usage
### `tracing::Level` (requires `tracing` feature)
```rust,ignore
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "azalia_serde::tracing")]
    level: tracing::Level,
}
```

### `aws_types::types::Region` (requires `aws` feature)
```rust,ignore
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(with = "azalia_serde::aws::region")]
    region: aws_sdk_s3::types::Region
}
```
