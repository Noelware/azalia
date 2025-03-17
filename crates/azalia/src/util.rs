// 🐻‍❄️🪚 azalia: Noelware's Rust commons library.
// Copyright (c) 2024-2025 Noelware, LLC. <team@noelware.org>
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

#[cfg(all(
    feature = "regex",
    any(feature = "lazy", feature = "use-once-cell"),
    any(feature = "std", feature = "alloc")
))]
pub static TRUTHY_REGEX: crate::macros::LazySync<regex::Regex> =
    crate::lazy!(crate::regex!(r#"^(yes|true|si*|e|enable|1)$"#));

#[cfg(feature = "std")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "std")))]
/// Returns a <code>[`Cow`](crate::libstd::Cow)\<'static, [`str`]\></code> of any <code>[`Box`](crate::libstd::Box)\<dyn [`Any`](crate::libstd::any::Any) + [`Send`] + 'static\></code>,
/// mainly from a panic hook or [`std::panic::catch_unwind`].
pub fn message_from_panic(
    error: Box<dyn crate::libstd::any::Any + Send + 'static>,
) -> crate::libstd::Cow<'static, str> {
    use crate::libstd::Cow;

    if let Some(msg) = error.downcast_ref::<String>() {
        Cow::Owned(msg.clone())
    } else if let Some(s) = error.downcast_ref::<&str>() {
        Cow::Borrowed(s)
    } else {
        Cow::Borrowed("<unknown panic message>")
    }
}
