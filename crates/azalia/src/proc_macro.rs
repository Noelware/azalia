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

/// Unwrap a <code>[`Result`]\<T, [`syn::Error`]\></code> into `T` but
/// propagate a [`syn::Error`] as a compile-time error.
///
/// [`syn::Error`]: https://docs.rs/syn/latest/syn/struct.Error.html
///
/// ## Example
/// ```
/// # extern crate proc_macro;
/// use azalia::into_compile_error;
///
/// # const _: &str = stringify! {
/// #[proc_macro]
/// # };
/// pub fn my_proc_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
///     let tt = into_compile_error!(parse_my_data());
///     tt
/// }
///
/// pub(crate) fn parse_my_data() -> syn::Result<proc_macro::TokenStream> {
///     // ...
///     # Ok(proc_macro::TokenStream::new())
/// }
/// ```
#[cfg(feature = "proc-macro")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "proc-macro")))]
#[macro_export]
macro_rules! into_compile_error {
    ($e:expr) => {
        match $e {
            ::core::result::Result::Ok(tt) => tt,
            ::core::result::Result::Err(err) => return err.into_compile_error().into(),
        }
    };
}

/// Construct a identifier with a given input.
///
/// It is recommended to use the `ident` path as Rust will always make
/// this as a valid identifier.
#[cfg(feature = "proc-macro")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "proc-macro")))]
#[macro_export]
macro_rules! ident {
    ($ident:ident) => {
        ::proc_macro2::Ident::new(stringify!($ident), ::proc_macro2::Span::call_site())
    };

    ($ident:literal) => {
        ::proc_macro2::Ident::new(stringify!($ident), ::proc_macro2::Span::call_site())
    };

    (raw $ident:literal) => {
        ::proc_macro2::Ident::new_raw(stringify!($ident), ::proc_macro2::Span::call_site())
    };

    ($ident:expr) => {
        ::proc_macro2::Ident::new(&$ident, ::proc_macro2::Span::call_site())
    };

    (raw $ident:expr) => {
        ::proc_macro2::Ident::new_raw(&$ident, ::proc_macro2::Span::call_site())
    };
}

/// Construct a [`syn::Error`] with a message and an optional span.
///
/// [`syn::Error`]: https://docs.rs/syn/latest/syn/struct.Error.html
#[cfg(feature = "proc-macro")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "proc-macro")))]
#[macro_export]
macro_rules! error {
    ($message:literal) => {
        ::syn::Error::new(::proc_macro2::Span::call_site(), $message)
    };

    ($message:expr) => {
        ::syn::Error::new(::proc_macro2::Span::call_site(), $message)
    };

    ($message:expr => $span:ident) => {{
        #[allow(unused_imports)]
        use ::syn::spanned::Spanned;

        ::syn::Error::new(($span).span(), $message)
    }};

    ($message:literal => $span:ident) => {{
        #[allow(unused_imports)]
        use ::syn::spanned::Spanned;

        ::syn::Error::new(($span).span(), $message)
    }};
}
