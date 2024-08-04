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

use crate::merge::{Args, Container};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{spanned::Spanned, Attribute, DeriveInput, Fields, Member};

struct Field {
    member: Member,
    span: Span,
    attrs: Vec<Attribute>,
}

impl From<(usize, &syn::Field)> for Field {
    fn from((idx, field): (usize, &syn::Field)) -> Self {
        Field {
            attrs: field.attrs.clone(),
            span: field.span(),
            member: if let Some(ident) = field.ident.clone() {
                Member::Named(ident)
            } else {
                Member::Unnamed(idx.into())
            },
        }
    }
}

pub fn struct_fields(input: &DeriveInput, fields: &Fields) -> TokenStream {
    let name = &input.ident;
    let generics = &input.generics;
    let container_args = match input
        .attrs
        .clone()
        .into_iter()
        .filter(|x| match x.meta.path().require_ident() {
            Ok(ident) => ident == "merge",
            Err(_) => false,
        })
        .map(|attr| attr.parse_args::<Container>())
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(elems) => elems.first().cloned().unwrap_or_default(),
        Err(x) => return x.into_compile_error(),
    };

    let krate = &container_args.krate;
    if fields.is_empty() {
        return quote! {
            #[automatically_derived]
            impl #generics #krate::merge::Merge for #name #generics {
                fn merge(&mut self, _other: Self) {}
            }
        };
    }

    let mut assignments = Vec::with_capacity(fields.len());
    let fields = fields.iter().enumerate().map(Field::from);
    for field in fields {
        if let Some(tt) = gen_field_assignment(&field, &container_args) {
            assignments.push(tt);
        }
    }

    quote! {
        #[automatically_derived]
        impl #generics #krate::merge::Merge for #name #generics {
            fn merge(&mut self, other: Self) {
                #(#assignments)*
            }
        }
    }
}

fn gen_field_assignment(field: &Field, Container { krate }: &Container) -> Option<TokenStream> {
    let attr = field
        .attrs
        .iter()
        .filter(|s| match s.meta.path().get_ident() {
            Some(ident) => ident == "merge",
            None => false,
        })
        .filter_map(|s| s.parse_args::<Args>().ok())
        .collect::<Vec<_>>();

    let first = attr
        .first()
        // only needed since &Args doesn't implement Default
        // TODO(@auguwu): fix
        .map(|s| Args {
            is_skipped: s.is_skipped,
            strategy: s.strategy.clone()
        })
        .unwrap_or_default();

    // don't even attempt to merge if it is skipped
    if first.is_skipped {
        return None;
    }

    let name = &field.member;
    Some(match first.strategy {
        Some(path) => quote_spanned!(path.span()=> #path(&mut self.#name, other.#name);),
        None => quote_spanned!(field.span=> #krate::merge::Merge::merge(&mut self.#name, other.#name);),
    })
}
