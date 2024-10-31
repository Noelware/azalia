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

#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc = include_str!("../README.md")]
#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]

use proc_macro::TokenStream;
use proc_macro2::Span;
use std::fmt::Display;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput};

mod merge;

/// The `#[env_test]` procedural macro will generate a test with any set environment variable.
///
/// ## Example
/// ```ignore
/// use azalia::config::env_test;
/// use std::env::var;
///
/// #[env_test(hello = "world")]
/// fn test_env() {
///     assert!(var("HELLO").is_ok());
/// }
/// ```
#[cfg(feature = "unstable")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "unstable")))]
#[proc_macro_attribute]
pub fn env_test(attrs: TokenStream, body: TokenStream) -> TokenStream {
    use heck::ToShoutySnakeCase;
    use quote::quote;
    use syn::{
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        spanned::Spanned,
        Expr, ExprAssign, ExprLit, ExprPath, ItemFn, Lit, Signature, Token,
    };

    struct TestEnv(Vec<(String, String)>);
    impl Parse for TestEnv {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let punct: Punctuated<ExprAssign, Token![,]> = Punctuated::parse_terminated(input)?;
            let mut items: Vec<(String, String)> = Vec::new();

            for assignment in punct.into_iter() {
                let Expr::Path(ExprPath { path, .. }) = *assignment.left else {
                    return Err(syn::Error::new(
                        assignment.left.span(),
                        "expected identifier as left-hand side",
                    ));
                };

                let ident = path.require_ident()?;
                match *assignment.right {
                    Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => {
                        items.push((ident.to_string().to_shouty_snake_case(), s.value()));
                    }

                    _ => {
                        return Err(syn::Error::new(
                            assignment.right.span(),
                            "expected literal string as right-hand side",
                        ))
                    }
                }
            }

            Ok(Self(items))
        }
    }

    let ItemFn {
        vis,
        block,
        sig: Signature {
            ident, inputs, output, ..
        },
        ..
    } = parse_macro_input!(body as ItemFn);

    if !inputs.is_empty() {
        return syn::Error::new(inputs.span(), "expected no inputs")
            .into_compile_error()
            .into();
    }

    let TestEnv(env) = parse_macro_input!(attrs as TestEnv);

    let env_as_slice = env
        .iter()
        .map(|(key, value)| quote!((#key, #value),))
        .collect::<Vec<_>>();

    quote! {
        #[::core::prelude::v1::test]
        #vis fn #ident() #output {
            ::azalia::config::expand_multi_with(
                [#(#env_as_slice),*],
                || #output #block
            )
        }
    }
    .into()
}

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
#[proc_macro_derive(Merge, attributes(merge))]
pub fn merge(body: TokenStream) -> TokenStream {
    let input = parse_macro_input!(body as DeriveInput);
    match &input.data {
        Data::Struct(DataStruct { fields, .. }) => merge::struct_fields(&input, fields).into(),
        Data::Enum(_) => error(Span::call_site(), "enums are not supported with #[derive(Merge)]"),
        Data::Union(_) => error(Span::call_site(), "unions are not supported with #[derive(Merge)]"),
    }
}

fn error<T: Display>(span: Span, msg: T) -> TokenStream {
    syn::Error::new(span, msg).into_compile_error().into()
}
