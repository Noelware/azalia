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

#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]
#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]

mod merge;

#[cfg(feature = "unstable")]
mod tryfromenv;

use proc_macro::TokenStream;
use syn::{parse_macro_input, spanned::Spanned, Data, DeriveInput};

/// Procedural macro to implement the [**`Merge`**] trait from `azalia::config`'s `merge` module.
///
/// [**`Merge`**]: trait.Merge.html
///
/// ## Example
/// > **NOTE**: This will require the `macros` feature for `azalia_config` or `config+macros` for
/// > the `azalia` crate.
///
/// ```ignore
/// # mod azalia {
/// #     mod config {
/// #         mod merge {
/// #             pub use azalia_config_macros::Merge;
/// #         }
/// #     }
/// # }
/// #
/// use azalia::config::merge::Merge;
///
/// #[derive(Debug, Merge, Default, PartialEq)]
/// pub struct Config {
///     pub a: String,
///
///     #[merge(skip)]
///     pub b: bool,
///
///     #[merge(strategy = "i32::merge")]
///     pub c: i32,
/// }
///
/// let mut config = Config::default();
/// assert_eq!(config, Config { a: String::default(), b: false, c: 0i32 });
///
/// config.merge(Config { a: "hello world".into(), b: true, c: 9 });
/// assert_eq!(config, Config { a: "hello world".into(), b: false, c: 42 });
///
/// // strategies can be regular functions that will expand
/// // to a fn that implements `(&mut self, Self)`
/// mod i32 {
///     pub fn merge(lhs: &mut i32, rhs: i32) {
///         *lhs = 42;
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[proc_macro_derive(Merge, attributes(merge))]
pub fn Merge(input: TokenStream) -> TokenStream {
    let derive = parse_macro_input!(input as DeriveInput);
    match &derive.data {
        Data::Struct(s) => merge::expand_struct(&derive, &s.fields).into(),
        Data::Enum(e) => syn::Error::new(e.enum_token.span(), "merge trait for enumerations are not supported")
            .into_compile_error()
            .into(),

        Data::Union(u) => syn::Error::new(u.union_token.span(), "merge trait for unions will never be supported")
            .into_compile_error()
            .into(),
    }
}

/// Procedural macro to implement [`TryFromEnv`] for `struct`s.
///
/// [`TryFromEnv`]: trait.TryFromEnv.html
///
/// ## Example
/// ```ignore
/// # mod azalia {
/// #     mod config {
/// #         mod env {
/// #             pub use azalia_config_macros::TryFromEnv;
/// #         }
/// #     }
/// # }
/// #
/// use azalia::config::env::TryFromEnv;
/// use std::error::Error;
///
/// #[derive(TryFromEnv)]
/// #[env(error = Box<dyn Error>, prefix = "APP_")]
/// pub struct Config {
///     #[env(var = "A")]
///     pub a: String,
/// }
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// #     let _ = unsafe { std::env::set_var("APP_A", "apple") };
/// #     assert!(std::env::var("APP_A").is_ok());
/// #
/// // assume that APP_A=apple
/// let config = Config::try_from_env()?;
/// assert!(config.a, "apple");
/// #
/// #      let _ = unsafe { std::env::remove_var("APP_A") };
/// #      assert!(std::env::var("APP_A").is_err());
/// #
/// #      Ok(())
/// # }
/// ```
#[allow(non_snake_case)]
#[cfg(feature = "unstable")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "unstable")))]
#[proc_macro_derive(TryFromEnv, attributes(env))]
pub fn TryFromEnv(input: TokenStream) -> TokenStream {
    // let derive = parse_macro_input!(input as DeriveInput);
    // match &derive.data {
    //     Data::Struct(s) => tryfromenv::expand_struct(&derive, &s.fields)
    //         .unwrap_or_else(syn::Error::into_compile_error)
    //         .into(),

    //     Data::Enum(_) => syn::Error::new(derive.span(), "TryFromEnv trait for enumerations are not supported")
    //         .into_compile_error()
    //         .into(),

    //     Data::Union(_) => syn::Error::new(derive.span(), "TryFromEnv trait for unions will never be supported")
    //         .into_compile_error()
    //         .into(),
    // }

    syn::Error::new(
        proc_macro2::Span::call_site(),
        "`#[derive(TryFromEnv)]` is not implemented at this time",
    )
    .into_compile_error()
    .into()
}
