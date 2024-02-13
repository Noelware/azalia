# 🐻‍❄️🪚 `noelware-config`
The **noelware-config** crate provides useful Rust macros and traits to make configurations more easier.

## `no_std` compatibility
This crate works with `no_std` and can be useful for the merge feature of this crate. To enable it, use `default-features = false`.

> [!NOTE]
> This crate requires [`liballoc`](https://doc.rust-lang.org/stable/alloc) to be available.

```toml
# Cargo.toml:
[dependencies]
noelware-config = { version = "*", registry = "noelware", default-features = false }
```
