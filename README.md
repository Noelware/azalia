# 🐻‍❄️🪚 `core-rs`
> *Collection of Rust crates that are used by and built for Noelware's projects*

`core-rs` is the main repository that hosts most of Noelware's crates that can be publically used, but for applications/programs only, which is why all the crates available here are not on [`crates.io`](https://crates.io). Which means, library-facing code outside of the scope of this repository cannot be published with `crates.io`.

> [!IMPORTANT]
> The only crate from `core-rs` is available on [`crates.io`](https://crates.io) is [`noelware-serde`](https://docs.rs/noelware-serde) as it is probably a crate that most people would want to use.

## Why aren't you publishing these crates to crates.io?
Because, it felt unneccessary to upload and pollute crates.io with Rust crates that are the scope of Noelware's software projects itself, while you're free to use them and grab the code for library-facing code (with the license attached), since this repository is released under the **MIT License**.

If you don't wish to import from `crates.noelware.cloud`, then you can just use a Git import instead of the commit hash you need. We don't mind!

## Are you going to do this to your public crates already on `crates.io`?
No. We don't plan on moving our public-facing crates like [`remi-rs`](https://github.com/Noelware/remi-rs) to our Cargo registry.

## `no-std` compatibility
All crates except `noelware-config` and `noelware-config-derive` will require `libstd` to be available. If they can be made `no-std` compliant, you can submit an issue and see if we can work it out.

## MSRV compatibility
All crates can be compiled from the latest stable version of Rust and two additional previous versions. While older versions below 1.70 can be compiled, we do recommend following the base Rust version we support.

We test all of our crates on the latest stable and nightly versions of Rust.

## License
**core-rs** by Noelware, LLC. is released under the **MIT License** with love. Please read the [`LICENSE`](./LICENSE) file in the repository attached for more information about on what you can do with the code.

If you're going to use the code from `core-rs` in your public-facing crates, please copy the code ***with the license on top*** of the file.
