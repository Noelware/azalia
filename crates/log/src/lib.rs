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
mod writers;

#[cfg(not(feature = "writers"))]
pub use writers::JsonVisitor;

use std::{io::Write, sync::RwLock};
use tracing::{span, Event, Metadata, Subscriber};
use tracing_subscriber::{
    registry::{LookupSpan, SpanRef},
    Layer,
};

/// Represents a function-based trait to create a [`String`] buffer with pieces you might need. This shouldn't
/// be implemented directly, but can be written with the following function signature:
///
/// ```rust,ignore
/// fn(&tracing::Event, &tracing::Metadata, Vec<serde_json::Value>) -> Result<String, std::fmt::Error>
/// ```
pub trait WriteFn<S: for<'l> LookupSpan<'l>>: Send {
    fn buffer(&self, event: &Event, metadata: &Metadata, spans: Vec<SpanRef<'_, S>>) -> String;
}

impl<S: for<'l> LookupSpan<'l>, F> WriteFn<S> for F
where
    F: Fn(&Event, &Metadata, Vec<SpanRef<'_, S>>) -> String + Send,
{
    fn buffer(&self, event: &Event, metadata: &Metadata, spans: Vec<SpanRef<'_, S>>) -> String {
        (self)(event, metadata, spans)
    }
}

/// Represents a [`Layer`] for writing to a type that implements [`Write`], with a optional
/// [`WriteFn`] to go alongside with this type.
pub struct WriteLayer<S: for<'l> LookupSpan<'l>> {
    writer: RwLock<Box<dyn Write + Send + Sync>>,
    write_fn: Option<Box<dyn WriteFn<S> + Send + Sync>>,
}

impl<S: for<'l> LookupSpan<'l>> WriteLayer<S> {
    /// Creates a new [`WriteLayer`] without a [`WriteFn`].
    pub fn new<W: Write + Send + Sync + 'static>(writer: W) -> WriteLayer<S> {
        WriteLayer {
            writer: RwLock::new(Box::new(writer)),
            write_fn: None,
        }
    }

    /// Creates a new [`WriteLayer`] with a specified [`WriteFn`].
    pub fn new_with<W: Write + Send + Sync + 'static, F: WriteFn<S> + Send + Sync + 'static>(
        writer: W,
        fn_: F,
    ) -> WriteLayer<S> {
        WriteLayer {
            writer: RwLock::new(Box::new(writer)),
            write_fn: Some(Box::new(fn_)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct JsonExtension(pub(crate) std::collections::BTreeMap<String, serde_json::Value>);
impl<S: Subscriber + for<'l> LookupSpan<'l>> Layer<S> for WriteLayer<S> {
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
        let mut writer = self.writer.write().unwrap();
        if let Some(ref fn_) = self.write_fn {
            cfg_if::cfg_if! {
                if #[cfg(feature = "log")] {
                    use tracing_log::NormalizeEvent;

                    let metadata = event.normalized_metadata();
                    let metadata = metadata.as_ref().unwrap_or_else(|| event.metadata());
                } else {
                    let metadata = event.metadata();
                }
            };

            let mut spans: Vec<SpanRef<'_, S>> = vec![];
            if let Some(scope) = ctx.event_scope(event) {
                for span in scope.from_root() {
                    spans.push(span);
                }
            }

            let buf = fn_.buffer(event, metadata, spans);
            let _ = write!(writer, "{buf}");
            let _ = writeln!(writer);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::WriteLayer;
    use std::io;
    use tracing::Dispatch;
    use tracing_subscriber::{layer::SubscriberExt, registry, Layer, Registry};

    fn __assert_is_layer<S>(_: &dyn Layer<S>) {
        /* no body here */
    }

    fn __assert_is_dispatchable(_: impl Into<Dispatch>) {
        /* no body here :3 */
    }

    #[test]
    fn assertions() {
        __assert_is_layer::<Registry>(&WriteLayer::new(io::stdout()));
        __assert_is_dispatchable(registry().with(WriteLayer::new(io::stdout())));

        #[cfg(feature = "writers")]
        __assert_is_dispatchable(registry().with(WriteLayer::new_with(
            io::stdout(),
            crate::writers::default::Writer::default(),
        )));

        #[cfg(feature = "writers")]
        __assert_is_dispatchable(registry().with(WriteLayer::new_with(io::stdout(), crate::writers::json)));
    }
}
