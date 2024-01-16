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

use super::JsonVisitor;
use chrono::Local;
use owo_colors::{colors::CustomColor, FgColorDisplay, OwoColorize, Stream};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fmt::{self, Debug, Write as _},
    process, thread,
};
use tracing::{
    field::{Field, Visit},
    Event, Level, Metadata,
};

pub(crate) struct DefaultVisitor<'s> {
    result: fmt::Result,
    writer: &'s mut (dyn fmt::Write + Send),
}

impl<'s> Visit for DefaultVisitor<'s> {
    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        if self.result.is_err() {
            return;
        }

        // don't emit messages from the `log` crate as those are translated
        // in the generic layer itself.
        if field.name().starts_with("log.") {
            return;
        }

        match field.name() {
            "message" => {
                self.result = write!(self.writer, "{value:?}");
            }

            field => {
                self.result = write!(
                    self.writer,
                    " {}",
                    format!("{field}={value:?}").if_supports_color(Stream::Stdout, |txt| txt.fg_rgb::<134, 134, 134>())
                );
            }
        }
    }
}

// time level   module (thread name): message
/// Provides a default [`WriteFn`](crate::WriteFn) that is soothing to see in your terminal.
pub fn default(event: &Event, metadata: &Metadata, _spans: Vec<Value>) -> String {
    let mut buf = String::new();
    let now = Local::now().format("%B %d, %G - %H:%M:%S %p");
    let (b1, b2) = (
        "[".if_supports_color(Stream::Stdout, gray_fg),
        "]".if_supports_color(Stream::Stdout, gray_fg),
    );

    let _ = write!(
        buf,
        "{} {}     ",
        now.if_supports_color(Stream::Stdout, |x| x.fg_rgb::<134, 134, 134>()),
        match *metadata.level() {
            Level::TRACE => "TRACE"
                .if_supports_color(Stream::Stdout, |x| x.fg_rgb::<163, 182, 138>())
                .bold()
                .to_string(),

            Level::DEBUG => "DEBUG"
                .if_supports_color(Stream::Stdout, |x| x.fg_rgb::<163, 182, 138>())
                .bold()
                .to_string(),

            Level::INFO => "INFO "
                .if_supports_color(Stream::Stdout, |x| x.fg_rgb::<178, 157, 243>())
                .bold()
                .to_string(),

            Level::WARN => "WARN "
                .if_supports_color(Stream::Stdout, |x| x.fg_rgb::<243, 243, 134>())
                .bold()
                .to_string(),

            Level::ERROR => "ERROR"
                .if_supports_color(Stream::Stdout, |x| x.fg_rgb::<153, 75, 104>())
                .bold()
                .to_string(),
        },
    );

    let _ = write!(
        buf,
        "{} {}{}{}: ",
        metadata.module_path().unwrap_or("«unknown»"),
        b1,
        thread::current().name().unwrap_or("main"),
        b2
    );

    let mut visitor = DefaultVisitor {
        result: Ok(()),
        writer: &mut buf,
    };

    event.record(&mut visitor);
    buf
}

/// Provides a Logstash-style [`WriteFn`](crate::WriteFn) implementation as a stringified JSON object.
pub fn json(event: &Event, metadata: &Metadata, spans: Vec<Value>) -> String {
    let now = Local::now();
    let thread = std::thread::current();
    let pid = process::id();

    let mut tree = BTreeMap::new();
    let mut visitor = JsonVisitor(&mut tree);
    event.record(&mut visitor);

    let message = tree
        .remove("message")
        .unwrap_or(Value::String(String::from("<none provided>")));

    serde_json::to_string(&json!({
        "@timestamp": now.to_rfc3339(),
        "message": message,
        "metadata.module": metadata.module_path(),
        "metadata.file": metadata.file(),
        "metadata.line": metadata.line(),
        "thread.name": thread.name().unwrap_or("main"),
        "process.id": pid,
        "spans": spans,
        "fields": match tree.is_empty() {
            true => None,
            false => Some(tree),
        }
    }))
    .unwrap()
}

#[cold]
fn __assert_dyn() {
    use crate::WriteFn;

    let _: &dyn WriteFn = &default;
    let _: &dyn WriteFn = &json;
}

fn gray_fg<'a>(x: &'a &'a str) -> FgColorDisplay<'a, CustomColor<134, 134, 134>, &str> {
    x.fg_rgb::<134, 134, 134>()
}
