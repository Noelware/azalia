# 🐻‍❄️🪚 Azalia
**Azalia** is a family of crates that implement common code that has been copied and pasted each time Noelware has wrote Rust code in our projects, so we decided to make them into crates that can be digestable into code.

> [!IMPORTANT]
> We do not distribute *all* Azalia crates onto `crates.io` so we don't clutter up `crates.io`, they are hosted on [cargo.noelware.cloud](https://cargo.noelware.cloud) and indexed from [crates.noelware.cloud](https://crates.noelware.cloud), `cargo.noelware.cloud` is just the frontend to list all the crates avaliable and provide documentation for them.
>
> Also, all **Azalia** crates are heavily experimental and can change at anytime, so don't expect everything to be stablised.

## :question: Why aren't you publishing these crates to crates.io?
Because, it felt unneccessary to upload and pollute crates.io with Rust crates that are the scope of Noelware's software projects itself, while you're free to use them and grab the code for library-facing code (with the license attached), since this repository is released under the **MIT License**.

If you don't wish to import from `crates.noelware.cloud`, then you can just use Cargo's Git dependencies feature instead, we don't mind.

## :question: Are you going to do this to your public crates already on `crates.io`?
No. We don't plan on moving our public-facing crates like [`remi-rs`](https://github.com/Noelware/remi-rs) to our Cargo registry.

## libstd compat
All crates except `azalia-config` and `azalia-config-derive` will require `libstd` to be available. If `libstd` is disabled, some crates might require `liballoc` to be avaliable as well.

## MSRV compatibility
All crates can be compiled from the latest stable version of Rust and two additional previous versions. While older versions below 1.70 can be compiled, we do recommend following the base Rust version we support.

We test all of our crates on the latest stable and nightly versions of Rust.

## License
**Azalia** by Noelware, LLC. is released under the **MIT License** with love. Please read the [`LICENSE`](./LICENSE) file in the repository attached for more information about on what you can do with the code.

If you're going to use the code from `core-rs` in your public-facing crates, please copy the code ***with the license on top*** of the file.
