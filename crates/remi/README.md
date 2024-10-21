<div align="center">
    <h3>🐻‍❄️🪚 <code>azalia-remi</code></h3>
    <h4>Unified storage services for <a href="https://docs.rs/remi/*/remi/trait.StorageService.html">remi::StorageService</a> of official crates</h4>
    <hr />
</div>

**azalia-remi** adds a unified storage service on top of **remi-rs** for allow configuring multiple storage services but only uses one from what the end user wants.

This uses Cargo's crate features to implicitilly allow you to pick out which Remi-based crates to implement into your applications. You can use the `features = ["all"]` in your Cargo.toml's definition of `azalia-remi` to include all crates.

## Example
```rust,no_run
// Cargo.toml:
//
// [dependencies]
// tokio = { version = "*", features = ["full"] }
//
// azalia-remi = { version = "^0", features = ["fs"], registry = "noelware" }
// # OR
// azalia-remi = { version = "^0", features = ["fs"], git = "https://github.com/Noelware/azalia" }

use azalia_remi::{
    StorageService,
    Config,

    core::StorageService as _,
    fs::StorageConfig
};

#[tokio::main]
async fn main() {
    let service = Config::Filesystem(StorageConfig::default().with_directory("/data"));
    service.init().await?; // initialize the fs version of remi

    // do whatever you want

    Ok(())
}
```
