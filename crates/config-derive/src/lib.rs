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

use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_error::proc_macro_error;
use std::fmt::Display;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

mod args;
mod expand;

/// Implements the `Merge` trait onto a struct. Unions won't be supported but enums
/// might be if a concrete implementation can be reasoned.
///
/// ## Example
/// ```rust,ignore
/// # use azalia_config::merge::Merge;
/// #
/// #[derive(Merge)]
/// struct MyStruct;
///
/// /*
/// // expands to:
/// #[automatically_derived]
/// impl ::azalia_config::merge::Merge for MyStruct {
///     fn merge(&mut self, _other: Self) {}
/// }
/// */
/// ```
///
/// ## Attributes
/// ### `#[merge(skip)]`
/// This will skip a field from being merged.
///
/// ### `#[merge(strategy = <path>)]`
/// This will replace the strategy for a field into what `<path>` is. It expects this signature:
///
/// ```rust,ignore
/// fn(left: &mut Type, right: Type);
/// ````
#[proc_macro_error]
#[proc_macro_derive(Merge, attributes(merge))]
pub fn merge(body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as DeriveInput);
    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => expand::struct_fields(&input, fields).into(),
        Data::Enum(_) => error(Span::call_site(), "enums are not supported with #[derive(Merge)]"),
        Data::Union(_) => error(Span::call_site(), "unions are not supported with #[derive(Merge)]"),
    }
}

fn error<T: Display>(span: Span, msg: T) -> TokenStream {
    syn::Error::new(span, msg).into_compile_error().into()
}
