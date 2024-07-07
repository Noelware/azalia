// 🐻‍❄️🪚 Azalia: Family of crates that implement common Rust code
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

use owo_colors::{OwoColorize, Stream};
use std::fmt;
use tracing::field::Visit;

pub struct Visitor<'s, W: fmt::Write + Send> {
    pub result: fmt::Result,
    pub writer: &'s mut W,
    pub stream: Stream,
    pub colors: bool,
}

impl<'s, W: fmt::Write + Send> Visit for Visitor<'s, W> {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        // Don't even do anything if the `result` was poisoned by an internal formatting error
        if self.result.is_err() {
            return;
        }

        // don't write if the field starts with `log.`
        if field.name().starts_with("log.") {
            return;
        }

        match field.name() {
            "message" => {
                self.result = write!(self.writer, "{value:?}");
            }

            name => {
                let value = match self.colors {
                    true => format!("{name}={value:?}")
                        .if_supports_color(self.stream, |x| x.fg_rgb::<134, 134, 134>())
                        .to_string(),

                    false => format!("{name}={value:?}"),
                };

                self.result = write!(self.writer, " {value}");
            }
        }
    }
}
