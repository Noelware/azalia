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

use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn::{
    ext::IdentExt,
    parse::{discouraged::Speculative, Parse, ParseStream},
    spanned::Spanned,
    Attribute, Expr, ExprCall, Ident, ItemFn, Lit, LitStr, Path, PathSegment, Token,
};

#[derive(PartialEq, Eq, Hash)]
enum Key {
    Literal(String),
    Path(Path),
}

impl Parse for Key {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let p = input.span();
        if input.peek(Lit) {
            return Ok(Key::Literal(input.parse::<LitStr>()?.value()));
        } else if input.peek(Ident::peek_any) {
            let ident: Ident = input.parse()?;
            return Ok(Key::Literal(ident.to_string()));
        }

        let fork = input.fork();

        if let Ok(path) = fork.parse::<Path>() {
            input.advance_to(&fork);
            return Ok(Key::Path(path));
        }

        Err(syn::Error::new(
            p,
            "expected literal string, path, identifier, or fn call expression",
        ))
    }
}

impl ToTokens for Key {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            Self::Literal(s) => s.to_tokens(tokens),
            Self::Path(p) => p.to_tokens(tokens),
        }
    }
}

pub struct Attributes {
    krate: Path,
    attrs: HashMap<Key, Expr>,
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            attrs: Default::default(),
            krate: Path {
                leading_colon: Some(Token![::](Span::call_site())),
                segments: [
                    PathSegment::from(Ident::new("azalia", Span::call_site())),
                    PathSegment::from(Ident::new("config", Span::call_site())),
                ]
                .into_iter()
                .collect(),
            },
        }
    }
}

impl Parse for Attributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.is_empty() {
            return Ok(Self::default());
        }

        let mut krate = Path {
            leading_colon: Some(Token![::](Span::call_site())),
            segments: [
                PathSegment::from(Ident::new("azalia", Span::call_site())),
                PathSegment::from(Ident::new("config", Span::call_site())),
            ]
            .into_iter()
            .collect(),
        };

        if input.peek(Token![crate]) && input.peek2(Token![=]) {
            input.parse::<Token![crate]>()?;
            input.parse::<Token![=]>()?;
            krate = input.parse()?;

            if input.is_empty() {
                return Ok(Attributes {
                    krate,
                    attrs: Default::default(),
                });
            }

            input.parse::<Token![,]>()?;
            if input.is_empty() {
                return Ok(Attributes {
                    krate,
                    attrs: Default::default(),
                });
            }
        }

        let mut attrs = HashMap::new();
        loop {
            let left = input.parse::<Key>()?;
            input.parse::<Token![=]>()?;
            let right = input.parse::<Expr>()?;

            attrs.insert(left, right);

            if input.is_empty() {
                break;
            }

            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }

        Ok(Self { krate, attrs })
    }
}

// credit: https://github.com/tokio-rs/tokio/blob/d4178cf34924d14fca4ecf551c97b8953376f25a/tokio-macros/src/entry.rs#L541-L562
//
// Check whether given attribute is a test attribute of forms:
// * `#[test]`
// * `#[core::prelude::*::test]` or `#[::core::prelude::*::test]`
// * `#[std::prelude::*::test]` or `#[::std::prelude::*::test]`
fn is_test_attribute(attr: &Attribute) -> bool {
    let path = match &attr.meta {
        syn::Meta::Path(path) => path,
        _ => return false,
    };

    let candidates = [["core", "prelude", "*", "test"], ["std", "prelude", "*", "test"]];
    if path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments[0].arguments.is_none()
        && path.segments[0].ident == "test"
    {
        return true;
    } else if path.segments.len() != candidates[0].len() {
        return false;
    }

    candidates.into_iter().any(|segments| {
        path.segments
            .iter()
            .zip(segments)
            .all(|(segment, path)| segment.arguments.is_none() && (path == "*" || segment.ident == path))
    })
}

pub fn expand(
    Attributes {
        krate,
        attrs: variables,
    }: &Attributes,
    ItemFn {
        mut attrs,
        block,
        sig,
        vis,
    }: ItemFn,
) -> syn::Result<TokenStream> {
    if attrs.iter().any(is_test_attribute) {
        return Err(syn::Error::new(block.span(), "a second #[test] attribute was found :("));
    }

    if sig.constness.is_some() {
        return Err(syn::Error::new(
            sig.constness.span(),
            "`const` is not allowed in test code",
        ));
    }

    if sig.abi.is_some() {
        return Err(syn::Error::new(
            sig.abi.span(),
            "external ABIs are not allowed in test code",
        ));
    }

    let items = variables
        .iter()
        .map(|(key, value)| match key {
            Key::Literal(s) => {
                let var = s.to_ascii_uppercase();
                quote!((#var, #value))
            }

            Key::Path(path) if path.get_ident().is_some() => {
                let ident = path.require_ident().unwrap();
                let var = ident.to_string().to_ascii_uppercase();

                quote!((#var, #value))
            }

            key => quote!((#key, #value)),
        })
        .collect::<Vec<_>>();

    attrs.push(
        sig.asyncness
            .is_some()
            .then_some(syn::parse_quote! {
                #[::tokio::test]
            })
            .unwrap_or(syn::parse_quote! {
                #[::core::prelude::v1::test]
            }),
    );

    Ok(quote! {
        #(#attrs)*
        #vis #sig {
            let _guard = #krate::env::MultipleEnvGuard::enter([#(#items),*]);
            let __ret = #block;

            // drop the guard after the block runs so that
            // it is released.
            ::core::mem::drop(_guard);
            __ret
        }
    })
}
