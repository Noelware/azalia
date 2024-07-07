# 🐻‍❄️🪚 `azalia-config`
The **azalia-config** crate provides useful Rust macros and traits to make configurations more easier.

## `libstd` compatibility
This crate works without `libstd` present and can be useful for the merge feature of this crate. To disable `libstd` usage in this crate, use `default-features = false` in your **Cargo.toml**.

> [!NOTE]
> This crate requires [`liballoc`](https://doc.rust-lang.org/stable/alloc) to be available.

```toml
# Cargo.toml:
[dependencies]
azalia-config = { version = "*", registry = "noelware", default-features = false }
```
