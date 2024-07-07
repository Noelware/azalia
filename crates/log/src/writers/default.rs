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

use crate::{JsonExtension, WriteFn};
use chrono::Local;
use owo_colors::{colors::CustomColor, FgColorDisplay, OwoColorize, Stream};
use std::{borrow::Cow, fmt::Write};
use tracing::{Event, Level, Metadata};
use tracing_subscriber::registry::{LookupSpan, SpanRef};

mod visitor;

const DEFAULT_TIMESTAMP: &str = "%B %d, %G - %H:%M:%S %p";

/// Writer that is the default logging writer for Noelware's Rust products and services.
pub struct Writer {
    /// whether to print the timstamp of the record or not.
    pub print_timestamp: bool,

    /// Format for timestamps. It'll use `%B %d, %G - %H:%M:%S %p` as the default format.
    pub timestamp_fmt: Cow<'static, str>,

    /// whether to print the module path where this record came from
    pub print_module: bool,

    /// whether to print the thread name of where the record is coming from
    pub print_thread: bool,

    /// whether to print the log level
    pub print_level: bool,

    /// whether to emit span information. all spans are printed AFTER the thread name
    /// but before the message.
    pub emit_spans: bool,

    /// [`Stream`] for colour detection.
    pub stream: Stream,

    /// whether to print the record with pretty colours or not.
    pub colors: bool,
}

impl Default for Writer {
    fn default() -> Self {
        Self {
            print_timestamp: true,
            timestamp_fmt: Cow::Borrowed(DEFAULT_TIMESTAMP),
            print_thread: true,
            print_module: true,
            print_level: true,
            emit_spans: true,
            stream: Stream::Stdout,
            colors: true,
        }
    }
}

impl Writer {
    /// whether to print the timstamp of the record or not.
    pub fn with_timestamp(mut self, yes: bool) -> Self {
        self.print_timestamp = yes;
        self
    }

    /// whether to print the thread name of where the record is coming from
    pub fn with_thread_name(mut self, yes: bool) -> Self {
        self.print_thread = yes;
        self
    }

    /// whether to print the module path where this record came from
    pub fn with_module(mut self, yes: bool) -> Self {
        self.print_module = yes;
        self
    }

    /// whether to print the log level
    pub fn with_level(mut self, yes: bool) -> Self {
        self.print_level = yes;
        self
    }

    /// whether to emit span information. all spans are printed AFTER the thread name
    /// but before the message.
    pub fn emit_spans(mut self, yes: bool) -> Self {
        self.emit_spans = yes;
        self
    }

    /// Sets the [`Stream`] used for colour detection.
    pub fn with_stream<I: Into<Stream>>(mut self, stream: I) -> Self {
        self.stream = stream.into();
        self
    }

    /// whether to print the record with pretty colours or not.
    pub fn with_colors(mut self, yes: bool) -> Self {
        self.colors = yes;
        self
    }

    /// Sets the format for the timestamp of the thread.
    pub fn with_timestamp_fmt<S: Into<Cow<'static, str>>>(mut self, fmt: S) -> Self {
        self.timestamp_fmt = fmt.into();
        self
    }
}

impl<L: for<'a> LookupSpan<'a>> WriteFn<L> for Writer {
    fn buffer(&self, event: &Event, metadata: &Metadata, spans: Vec<SpanRef<'_, L>>) -> String {
        self.write(event, metadata, spans)
    }
}

impl Writer {
    pub(crate) fn write<L: for<'a> LookupSpan<'a>>(
        &self,
        event: &Event<'_>,
        metadata: &Metadata<'_>,
        spans: Vec<SpanRef<'_, L>>,
    ) -> String {
        let mut buf = String::new();
        if self.print_timestamp {
            self.write_timestamp(&mut buf);
            let _ = write!(buf, " ");
        }

        if self.print_level {
            self.write_level(metadata, &mut buf);
            let _ = write!(buf, " ");
        }

        if self.print_module || self.print_thread {
            self.print_metadata(metadata, &mut buf);
            let _ = write!(buf, " ");
        }

        if self.emit_spans && !spans.is_empty() {
            self.print_spans(&spans, &mut buf);
        }

        let mut visitor = visitor::Visitor {
            result: Ok(()),
            writer: &mut buf,
            colors: self.colors,
            stream: self.stream,
        };

        event.record(&mut visitor);

        buf
    }

    pub(crate) fn write_timestamp<W: Write>(&self, buf: &mut W) {
        let now = Local::now().format(&self.timestamp_fmt);
        if self.colors {
            let _ = write!(
                buf,
                "{}",
                now.if_supports_color(self.stream, |txt| txt.fg_rgb::<134, 134, 134>())
            );

            return;
        }

        let _ = write!(buf, "{now}");
    }

    pub(crate) fn write_level<W: Write>(&self, metadata: &Metadata<'_>, buf: &mut W) {
        if self.colors {
            let level = metadata.level();
            let level = match *level {
                Level::TRACE => format!("{:<5}", level.as_str())
                    .if_supports_color(self.stream, |txt| txt.fg_rgb::<163, 182, 138>())
                    .bold()
                    .to_string(),

                Level::DEBUG => format!("{:<5}", level.as_str())
                    .if_supports_color(self.stream, |txt| txt.fg_rgb::<148, 224, 232>())
                    .bold()
                    .to_string(),

                Level::ERROR => format!("{:<5}", level.as_str())
                    .if_supports_color(self.stream, |txt| txt.fg_rgb::<153, 75, 104>())
                    .bold()
                    .to_string(),

                Level::WARN => format!("{:<5}", level.as_str())
                    .if_supports_color(self.stream, |txt| txt.fg_rgb::<243, 243, 134>())
                    .bold()
                    .to_string(),

                Level::INFO => format!("{:<5}", level.as_str())
                    .if_supports_color(self.stream, |txt| txt.fg_rgb::<178, 157, 243>())
                    .bold()
                    .to_string(),
            };

            let _ = write!(buf, "{level}");
            return;
        }

        let _ = write!(buf, "{}", metadata.level());
    }

    pub(crate) fn print_metadata<W: Write>(&self, metadata: &Metadata<'_>, buf: &mut W) {
        let module = metadata.module_path().unwrap_or("unknown");
        let thread = std::thread::current();
        let name = thread.name().unwrap_or("main");

        if self.colors {
            if self.print_module {
                let _ = write!(
                    buf,
                    "{}",
                    module.if_supports_color(self.stream, |txt| txt.fg_rgb::<72, 61, 139>())
                );
            }

            let (b1, b2) = (
                "«".if_supports_color(self.stream, gray_fg),
                "»".if_supports_color(self.stream, gray_fg),
            );

            if self.print_module {
                let _ = write!(buf, " ");
            }

            if self.print_thread {
                let _ = write!(
                    buf,
                    "{b1}{}{b2}",
                    name.if_supports_color(self.stream, |txt| txt.fg_rgb::<244, 181, 213>())
                );
            }

            return;
        }

        if self.print_module {
            let _ = write!(buf, "{module}");
            let _ = write!(buf, " ");
        }

        if self.print_thread {
            let _ = write!(buf, "«{name}»");
        }
    }

    pub(crate) fn print_spans<L: for<'a> LookupSpan<'a>, W: Write>(&self, spans: &[SpanRef<'_, L>], buf: &mut W) {
        for span in spans {
            if self.colors {
                let _ = write!(buf, "{}", "{".if_supports_color(self.stream, |txt| txt.bold()));
            } else {
                let _ = write!(buf, "{{");
            }

            let extensions = span.extensions();
            let json = extensions.get::<JsonExtension>().unwrap();

            let _ = write!(buf, "{}", span.name());
            let mut first = false;
            for (key, value) in &json.0 {
                if first {
                    let _ = write!(buf, ": {key}={value}");
                    first = false;

                    continue;
                }

                let _ = write!(buf, "; {key}={value}");
            }

            if self.colors {
                let _ = write!(buf, "{}", "}".if_supports_color(self.stream, |txt| txt.bold()));
            } else {
                let _ = write!(buf, "}}");
            }
        }

        let _ = write!(buf, ": ");
    }
}

fn gray_fg<'a>(x: &'a &'a str) -> FgColorDisplay<'a, CustomColor<134, 134, 134>, &str> {
    x.fg_rgb::<134, 134, 134>()
}

#[cfg(test)]
mod tests {
    use super::Writer;
    use crate::WriteLayer;
    use std::io;
    use tracing::{debug, error, error_span, info, info_span, trace, warn};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    #[test]
    fn test_out_default_formatter() {
        let _guard = tracing_subscriber::registry()
            .with(WriteLayer::new_with(std::io::stdout(), Writer::default()))
            .set_default();

        trace!("Hello, world!");
        debug!(hello = "world", "No.");

        let span = info_span!("heck", hello.world = true, abc = "d", num = 42);
        span.in_scope(|| {
            info!("hi... (in hello scope)");
            warn!("woah...");

            let err = <io::ErrorKind as Into<io::Error>>::into(io::ErrorKind::InvalidData);
            error!(%err, "we are so done for");
        });

        let span2 = error_span!("hello");
        span2.in_scope(|| {
            error!("in `hello` span scope");
        });

        info!("no longer in `hello` span scope");
    }
}
