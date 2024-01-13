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

#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "writers")]
#[cfg_attr(docsrs, doc(cfg(feature = "writers")))]
pub mod writers;

#[cfg(not(feature = "writers"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "writers"))))]
mod writers;

#[cfg(not(feature = "writers"))]
#[cfg_attr(docsrs, doc(cfg(not(feature = "writers"))))]
pub use writers::JsonVisitor;

use serde_json::{json, Value};
use std::{io::Write, sync::RwLock};
use tracing::{span, Event, Metadata, Subscriber};
use tracing_subscriber::{registry::LookupSpan, Layer};

/// Represents a function-based trait to create a [`String`] buffer with pieces you might need. This shouldn't
/// be implemented directly, but can be written with the following function signature:
///
/// ```rust,ignore
/// fn(&tracing::Event, &tracing::Metadata, Vec<serde_json::Value>) -> Result<String, std::fmt::Error>
/// ```
pub trait WriteFn: Send {
    fn buffer(&self, event: &Event, metadata: &Metadata, spans: Vec<Value>) -> String;
}

impl<F> WriteFn for F
where
    F: Fn(&Event, &Metadata, Vec<Value>) -> String + Send,
{
    fn buffer(&self, event: &Event, metadata: &Metadata, spans: Vec<Value>) -> String {
        (self)(event, metadata, spans)
    }
}

/// Represents a [`Layer`] for writing to a type that implements [`Write`], with a optional
/// [`WriteFn`] to go alongside with this type.
pub struct WriteLayer {
    writer: RwLock<Box<dyn Write + Send + Sync>>,
    write_fn: Option<Box<dyn WriteFn>>,
}

impl WriteLayer {
    /// Creates a new [`WriteLayer`] without a [`WriteFn`].
    pub fn new<W: Write + Send + Sync + 'static>(writer: W) -> WriteLayer {
        WriteLayer {
            writer: RwLock::new(Box::new(writer)),
            write_fn: None,
        }
    }

    /// Creates a new [`WriteLayer`] with a specified [`WriteFn`].
    pub fn new_with<W: Write + Send + Sync + 'static, F: WriteFn + 'static>(writer: W, fn_: F) -> WriteLayer {
        WriteLayer {
            writer: RwLock::new(Box::new(writer)),
            write_fn: Some(Box::new(fn_)),
        }
    }
}

pub(crate) struct JsonExtension(pub(crate) std::collections::BTreeMap<String, serde_json::Value>);
impl<S: Subscriber + for<'l> LookupSpan<'l>> Layer<S> for WriteLayer {
    fn on_new_span(&self, attrs: &span::Attributes<'_>, id: &span::Id, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let span = ctx.span(id).unwrap();
        let mut data = std::collections::BTreeMap::new();

        let mut visitor = crate::writers::JsonVisitor(&mut data);
        attrs.record(&mut visitor);

        span.extensions_mut().insert(JsonExtension(data));
    }

    fn on_record(&self, span: &span::Id, values: &span::Record<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        let span = ctx.span(span).unwrap();
        let mut exts = span.extensions_mut();
        let data: &mut JsonExtension = exts.get_mut::<JsonExtension>().unwrap();

        let mut visitor = crate::writers::JsonVisitor(&mut data.0);
        values.record(&mut visitor);
    }

    fn on_event(&self, event: &Event<'_>, ctx: tracing_subscriber::layer::Context<'_, S>) {
        #[cfg(feature = "log")]
        use tracing_log::NormalizeEvent;

        #[cfg(feature = "log")]
        let metadata = event.normalized_metadata();

        #[cfg(feature = "log")]
        let metadata = metadata.as_ref().unwrap_or_else(|| event.metadata());

        #[cfg(not(feature = "log"))]
        let metadata = event.metadata();

        let mut spans: Vec<Value> = vec![];
        if let Some(scope) = ctx.event_scope(event) {
            for span in scope.from_root() {
                let ext = span.extensions();
                let storage = ext.get::<JsonExtension>().unwrap();
                let data = &storage.0;

                spans.push(json!({
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
        }

        let mut writer = self.writer.write().unwrap();
        if let Some(ref fn_) = self.write_fn {
            let buf = fn_.buffer(event, metadata, spans);
            let _ = write!(writer, "{buf}");
            let _ = writeln!(writer);
        }
    }
}
