// 🐻‍❄️🪚 core-rs: Collection of Rust crates that are used by and built for Noelware's projects
// Copyright (c) 2024 Noelware, LLC. <team@noelware.org>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! Defines extra [`Serializer`] and [`Deserializer`] functions for [`tracing`]'s types that are most useful to use `serde` with.

use serde::{
    de::{Deserializer, Error},
    ser::Serializer,
    Deserialize,
};
use tracing::Level;

/// [`Serializer`] implementation for [`Level`].
///
/// ## Example
/// ```no_run
/// # use serde::Serialize;
/// # use tracing::Level;
/// #
/// #[derive(Serialize)]
/// pub struct MyStruct {
///     #[serde(serialize_with = "noelware_serde::tracing::serialize")]
///     level: Level,
/// }
/// ```
pub fn serialize<S: Serializer>(filter: &Level, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(match *filter {
        Level::TRACE => "trace",
        Level::DEBUG => "debug",
        Level::ERROR => "error",
        Level::WARN => "warn",
        Level::INFO => "info",
    })
}

/// [`Deserializer`] implementation for [`Level`].
///
/// ## Example
/// ```no_run
/// # use serde::Deserialize;
/// # use tracing::Level;
/// #
/// #[derive(Deserialize)]
/// pub struct MyStruct {
///     #[serde(deserialize_with = "noelware_serde::tracing::deserialize")]
///     level: Level,
/// }
/// ```
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Level, D::Error> {
    let string = String::deserialize(deserializer)?;
    match string.to_lowercase().as_str() {
        "trace" => Ok(Level::TRACE),
        "debug" => Ok(Level::DEBUG),
        "error" => Ok(Level::ERROR),
        "info" => Ok(Level::INFO),
        "warn" => Ok(Level::WARN),
        level => Err(D::Error::custom(format!(
            "wanted [trace, debug, error, info, warn]; received {level} instead"
        ))),
    }
}
