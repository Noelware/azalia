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

/// Construct a identifier with a given input.
///
/// It is recommended to use the `ident` path as Rust will always make
/// this as a valid identifier.
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
#[macro_export]
macro_rules! err {
    ($message:literal) => {
        ::syn::Error::new(::proc_macro2::Span::call_site(), $message)
    };

    ($message:expr) => {
        ::syn::Error::new(::proc_macro2::Span::call_site(), $message)
    };

    ($span:ident, $message:expr) => {{
        #[allow(unused_imports)]
        use ::syn::spanned::Spanned;

        ::syn::Error::new(($span).span(), $message)
    }};

    ($span:ident, $message:literal) => {{
        #[allow(unused_imports)]
        use ::syn::spanned::Spanned;

        ::syn::Error::new(($span).span(), $message)
    }};
}
