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

use crate::{JsonExtension, WriteFn};
use chrono::Local;
use owo_colors::{colors::CustomColor, FgColorDisplay, OwoColorize, Stream};
use std::fmt::{self, Write};
use tracing::{field::Visit, Level};
use tracing_subscriber::registry::LookupSpan;

struct Visitor<'s, W: fmt::Write + Send> {
    result: fmt::Result,
    writer: &'s mut W,
    stream: Stream,
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
                self.result = write!(
                    self.writer,
                    " {}",
                    format!("{name}={value:?}").if_supports_color(self.stream, |txt| txt.fg_rgb::<134, 134, 134>())
                );
            }
        }
    }
}

pub struct Writer {
    opts: Options,
}

// time level  module   (thread)({span=value}{span2=value2}?): message fields
impl<S: for<'l> LookupSpan<'l>> WriteFn<S> for Writer {
    fn buffer(
        &self,
        event: &tracing::Event,
        metadata: &tracing::Metadata,
        spans: Vec<tracing_subscriber::registry::SpanRef<'_, S>>,
    ) -> String {
        let mut buf = String::new();
        let now = Local::now().format("%B %d, %G - %H:%M:%S %p");
        let (b1, b2) = (
            "«".if_supports_color(self.opts.stream, gray_fg),
            "»".if_supports_color(self.opts.stream, gray_fg),
        );

        let level = match *metadata.level() {
            Level::TRACE => "TRACE"
                .if_supports_color(Stream::Stdout, |x| x.fg_rgb::<163, 182, 138>())
                .bold()
                .to_string(),

            Level::DEBUG => "DEBUG"
                .if_supports_color(Stream::Stdout, |x| x.fg_rgb::<148, 224, 232>())
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
        };

        let _ = write!(
            buf,
            "{} {level}  ",
            now.if_supports_color(self.opts.stream, |x| x.fg_rgb::<134, 134, 134>())
        );

        let module = metadata.module_path().unwrap_or("unknown");
        let thread = std::thread::current();
        let thread = thread.name().unwrap_or("main");

        let _ = write!(
            buf,
            "{} {b1}{:}{b2}",
            module.if_supports_color(self.opts.stream, |x| x.fg_rgb::<72, 61, 139>()),
            thread.if_supports_color(self.opts.stream, |x| x.fg_rgb::<244, 181, 213>())
        );

        if self.opts.emit_span_info && !spans.is_empty() {
            let _ = write!(buf, " ");
            for span in spans.iter() {
                let _ = write!(buf, "{}", "{".if_supports_color(Stream::Stderr, |t| t.white()));

                let extensions = span.extensions();
                let json = extensions.get::<JsonExtension>().unwrap();

                let _ = write!(buf, "{}", span.name());
                let mut first = true;
                for (key, value) in &json.0 {
                    if first {
                        first = false;
                        let _ = write!(buf, ": {key}={value}");
                        continue;
                    }

                    let _ = write!(buf, "; {key}={value}");
                }

                let _ = write!(buf, "{}", "}".if_supports_color(Stream::Stderr, |t| t.white()));
            }

            let _ = write!(buf, ": ");
        } else {
            let _ = write!(buf, " ");
        }

        let mut visitor = Visitor {
            result: Ok(()),
            writer: &mut buf,
            stream: self.opts.stream,
        };

        event.record(&mut visitor);

        buf
    }
}

/// Represents the options for configuring a [`Writer`].
pub struct Options {
    pub emit_span_info: bool,
    pub stream: Stream,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            emit_span_info: true,
            stream: Stream::Stdout,
        }
    }
}

/// Creates a new default writer which is in the style of all Noelware's software to keep it
/// easy, concise, and recognizable.
pub fn writer(options: Option<Options>) -> Writer {
    Writer {
        opts: options.unwrap_or_default(),
    }
}

fn gray_fg<'a>(x: &'a &'a str) -> FgColorDisplay<'a, CustomColor<134, 134, 134>, &str> {
    x.fg_rgb::<134, 134, 134>()
}

#[cfg(test)]
mod tests {
    use super::writer;
    use crate::WriteLayer;
    use std::io;
    use tracing::{debug, error, error_span, info, info_span, trace, warn};
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    #[test]
    fn test_out_default_formatter() {
        let _guard = tracing_subscriber::registry()
            .with(WriteLayer::new_with(std::io::stdout(), writer(None)))
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
