### 🐻‍❄️🪚 Azalia
#### *Family of crates for Noelware's Rust projects that implement repetitive code*

**Important** — As of March 16, 2025: we are publishing all Azalia crates onto [crates.io] so it can be easily accessible to projects that might need it.

**Azalia** is a family of Rust crates that is maintained by **Noelware, LLC.** to implement common functionality that is otherwise copied and can be into a open source library instead. While you can cherry-pick which Azalia crates you might need, it's far easier to use the [`azalia`] centralised crate that can toggle other Azalia crates via Cargo's crate features feature.

## Crates
| Name                   | Description                                                                                                            |
| :--------------------- | ---------------------------------------------------------------------------------------------------------------------- |
| [`azalia`]             | Easily consumable and centralised Rust crate to toggle other Azalia crates via Cargo's crates features feature.        |
| [`azalia-config`]      | Useful types and utilities when dealing with application configuration.                                                |
| [`azalia-log`]         | Provides a fancy **tracing** formatter to the alternative (`tracing-subscriber`) and JSON logger that mimics Logstash. |
| [`azalia-remi`]        | A unified **StorageService** structure when dealing with official **remi-rs** crates.                                  |
| [`azalia-serde`]       | Provides **serialization** and **deserialization** for other Rust crates that don't have a concrete impl.              |


## License
**Azalia** by Noelware, LLC. is released under the **MIT License** with love, care, and **Dr. Pepper**. No, seriously! Dr. Pepper is consuming the whole team and we need your help please!!!

[`azalia-proc-macros`]: ./crates/proc-macros
[`azalia-config`]:      ./crates/config
[`azalia-serde`]:       ./crates/serde
[`azalia-remi`]:        ./crates/remi
[`azalia-log`]:         ./crates/log
[`azalia`]:             ./crates/azalia
