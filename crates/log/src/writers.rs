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

#[cfg(feature = "writers")]
mod r#impl;

#[cfg(feature = "writers")]
pub use r#impl::*;

use serde_json::{json, Value};
use std::{collections::BTreeMap, fmt::Debug};
use tracing::field::{Field, Visit};

/// Reprensets a [`Visit`] implementation for recording [`tracing::Value`]s into JSON values.
pub struct JsonVisitor<'b>(pub(crate) &'b mut BTreeMap<String, Value>);

macro_rules! impl_visitor_instructions {
    ($($name:ident => $ty:ty),*) => {
        $(
            fn $name(&mut self, field: &::tracing::field::Field, value: $ty) {
                self.0.insert(field.name().to_string(), ::serde_json::json!(value));
            }
        )*
    }
}

impl<'b> Visit for JsonVisitor<'b> {
    impl_visitor_instructions! {
        record_f64 => f64,
        record_i64 => i64,
        record_u64 => u64,
        record_i128 => i128,
        record_bool => bool,
        record_str => &str,
        record_u128 => u128
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.0.insert(field.name().to_string(), json!(format!("{value}")));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        self.0.insert(field.name().to_string(), json!(format!("{value:?}")));
    }
}
