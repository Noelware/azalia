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

use super::JsonVisitor;
use crate::JsonExtension;
use chrono::Local;
use serde_json::{json, Value};
use std::{collections::BTreeMap, process};
use tracing::{Event, Metadata};
use tracing_subscriber::registry::{LookupSpan, SpanRef};

/// Provides a Logstash-style [`WriteFn`](crate::WriteFn) implementation as a stringified JSON object.
pub fn json<S: for<'l> LookupSpan<'l>>(event: &Event, metadata: &Metadata, spans: Vec<SpanRef<'_, S>>) -> String {
    let now = Local::now();
    let thread = std::thread::current();
    let pid = process::id();

    let mut tree = BTreeMap::new();
    let mut visitor = JsonVisitor(&mut tree);
    event.record(&mut visitor);

    let message = tree
        .remove("message")
        .unwrap_or(Value::String(String::from("<none provided>")));

    let mut spans_as_json: Vec<Value> = vec![];
    for span in spans.iter() {
        let ext = span.extensions();
        let storage = ext.get::<JsonExtension>().unwrap();
        let data = &storage.0;

        spans_as_json.push(json!({
            // show `null` if there are no fields available
            "fields": match data.is_empty() {
                true => None,
                false => Some(data)
            },

            "target": span.metadata().target(),
            "level": metadata.level().as_str().to_lowercase(),
            "name": span.metadata().name(),
            "meta": json!({
                "module": span.metadata().module_path(),
                "file": span.metadata().file(),
                "line": span.metadata().line(),
            })
        }));
    }

    serde_json::to_string(&json!({
        "@timestamp": now.to_rfc3339(),
        "message": message,
        "metadata.module": metadata.module_path(),
        "metadata.file": metadata.file(),
        "metadata.line": metadata.line(),
        "thread.name": thread.name().unwrap_or("main"),
        "process.id": pid,
        "spans": spans_as_json,
        "fields": match tree.is_empty() {
            true => None,
            false => Some(tree),
        }
    }))
    .unwrap()
}
