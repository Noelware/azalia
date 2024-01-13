# 🐻‍❄️🪚 `noelware-log`
The **noelware-log** crate is the main crate to use with [`tracing`](https://docs.rs/tracing). It has a `WriteLayer` struct that allows to use any [`std::io::Write`](https://doc.rust-lang.org/stable/std/io/trait.Write.html) type in a blocking fashion.

If you want a non-blocking version of `WriteLayer`, it does support any `std::io::Write` types and [`tracing_appender::NonBlocking`](https://docs.rs/tracing-appender) implements it.

## Usage
```rust,ignore
// > Cargo.toml:
//
// [dependencies]
// noelware-log = { version = "*", registry = "noelware" }
// serde_json = "1.0"
// tracing = "*"
// tracing-subscriber = "*"

use noelware_log::WriteLayer;
use serde_json::Value;
use tracing::{Event, Metadata};
use tracing_subscriber::prelude::*;

fn on_event(event: &Event<'_>, metadata: &Metadata<'_>, spans: Vec<Value>) -> String {
    let mut buf = String::new();
    // do something with `event` or `metadata`

    buf
}

fn main() {
    let writer = WriteLayer::new_with(std::io::stdout(), on_event);
    tracing_subscriber::registry()
        .with(writer)
        .init();

    tracing::info!("Hello, world!");
}
```

## Crate Features
### `writers` (disabled by default)
Provides some `WriteFn` implementations for a `WriteLayer`. This is mainly for Noelware's use-case but you can use them as well.
