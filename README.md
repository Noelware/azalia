<img src="https://cdn.floofy.dev/images/trans.png" alt="Noelware logo" align="right" width="128" height="128" />
<div align="center">
    <h3>🐻‍❄️🪚 Azalia</h3>
    <h4>Family of crates for Noelware's Rust projects that implement repeative code</h4>
</div>

**Azalia** are a host of crates maintained by the Noelware team that implement repeative code in our codebases.

**Important** — We do not distribute all of the crates listed in this repository onto **crates.io** due to being cluttered by code that mainly is used by us, it is hosted on a separate Cargo registry that we maintain.

**Warning** — All code in the repository is VERY EXPERIMENTAL and things can break at anytime & be removed without any notice.

## Crates
| Name                   | Description                                                                                   |
| ---------------------- | --------------------------------------------------------------------------------------------- |
| [`azalia`]             | Centralised Rust crate that contains all the crates but be enabled via Cargo's crate features |
| [`azalia-config`]      | Provides useful types and utilities when dealing with configuration                           |
| [`azalia-log`]         | Provides a fancy **tracing** formatter and JSON formatter that mimics Logstash's JSON output  |
| [`azalia-proc-macros`] | Provides utilities when dealing with procedural macros.                                       |
| [`azalia-remi`]        | Provides a unified `StorageService` for official `remi-rs` crates by Cargo's crate features.  |
| [`azalia-serde`]       | Provides ser/de implementations for types like `tracing::Level`                               |

## Why aren't these crates published to **crates.io**?
We decided that it was unnecessary to pollute code that, techincally, we would only use into a public registry. We recommended using Cargo's Git resolver when pulling the crates from this repository.

<!-- ## Why aren't these crates published to **crates.io**?
We decided that it was unnecessary to pollute code that, techincally, we would only use into a public registry. We recommended using Cargo's Git resolver when pulling the crates from this repository or link our [Cargo registry](#with-our-cargo-registry). -->

## MSRV Compatibility
For all crates that we host, we support Rust versions 1.70+ and test on stable and nightly branches of Rust. While you could compile the crates with older Rust versions, we do not guarantee that it'll work AND we will not provide support for older Rust versions.

## Usage
### Git Resolver
To use any crate from our repository, you can use Cargo's Git resolver to resolve the dependencies.

Use the `git` keyword when including the `azalia` crate, it is recommended to pin a commit:

```toml
[dependencies.azalia]
version = "0"
git = "https://github.com/Noelware/azalia"
# rev = "..."
```

<!-- ### With our Cargo registry
You can link up any Azalia crate from our public Cargo registry where anyone can pull the crates. You will need to add the `noelware` registry in `.cargo/config.toml`:

```toml
[registries.noelware]
index = "sparse+https://cargo.noelware.org/index"
```

Now, you can pull the `azalia` or any crate from our registry:

```toml
[dependencies.azalia]
version = "0"
registry = "noelware"
```
-->

## License
**Azalia** by Noelware, LLC. is released under the **MIT License** with love. Please read the [`LICENSE`](./LICENSE) file in the repository attached for more information about on what you can do with the code.

[`azalia-proc-macros`]: ./crates/proc-macros
[`azalia-config`]:      ./crates/config
[`azalia-serde`]:       ./crates/serde
[`azalia-remi`]:        ./crates/remi
[`azalia-log`]:         ./crates/log
[`azalia`]:             ./crates/azalia

<!-- [`azalia-proc-macros`]: https://cargo.noelware.org/~/azalia/proc-macros
[`azalia-config`]:      https://cargo.noelware.org/~/azalia/config
[`azalia-serde`]:       https://cargo.noelware.org/~/azalia/serde
[`azalia-remi`]:        https://cargo.noelware.org/~/azalia/remi
[`azalia-log`]:         https://cargo.noelware.org/~/azalia/log
[`azalia`]:             https://cargo.noelware.org/~/azalia -->
