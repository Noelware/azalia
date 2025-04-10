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
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    Attribute, DeriveInput, ExprPath, Fields, Ident, LitStr, Member, Meta, PathSegment, Token,
};

pub struct Path(pub(crate) ExprPath);
impl Parse for Path {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let s = input.span();
        if let Ok(path) = input.parse::<syn::ExprPath>() {
            Ok(Path(path))
        } else if let Ok(s) = input.parse::<LitStr>() {
            s.parse::<ExprPath>().map(Self)
        } else {
            Err(syn::Error::new(s, "expected either a qualified path (i.e, `std::mem::replace`) or a literal string that can be a qualified path"))
        }
    }
}

impl ToTokens for Path {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

pub struct Container {
    pub krate: Path,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            krate: Path(ExprPath {
                attrs: Vec::new(),
                qself: None,

                // ::azalia::config
                path: syn::Path {
                    leading_colon: Some(Token![::](Span::call_site())),
                    segments: [
                        PathSegment::from(Ident::new("azalia", Span::call_site())),
                        PathSegment::from(Ident::new("config", Span::call_site())),
                    ]
                    .into_iter()
                    .collect(),
                },
            }),
        }
    }
}

impl Parse for Container {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut me = Self::default();

        if input.peek(Token![crate]) {
            input.parse::<Token![crate]>()?;
            input.parse::<Token![=]>()?;

            me.krate = input.parse()?;
        }

        Ok(me)
    }
}

mod kw {
    syn::custom_keyword!(strategy);
    syn::custom_keyword!(skip);
}

#[derive(Default)]
pub struct Field {
    pub skipped: bool,
    pub strategy: Option<Path>,
}

struct StructField {
    attrs: Vec<Attribute>,
    member: Member,
    span: Span,
}

impl From<(usize, syn::Field)> for StructField {
    fn from((idx, field): (usize, syn::Field)) -> Self {
        let span = field.span();
        Self {
            attrs: field.attrs,
            span,
            member: field
                .ident
                .clone()
                .map(Member::Named)
                .unwrap_or(Member::Unnamed(idx.into())),
        }
    }
}

pub fn expand_struct(
    DeriveInput {
        ident, generics, attrs, ..
    }: &DeriveInput,
    fields: &Fields,
) -> TokenStream {
    let mut container = Container::default();
    for attr in attrs {
        if !attr.path().is_ident("merge") {
            continue;
        }

        if let Meta::List(list) = &attr.meta {
            if list.tokens.is_empty() {
                continue;
            }
        }

        if let Err(e) = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("crate") {
                container.krate = meta.value()?.parse()?;
                return Ok(());
            }

            Err(meta.error("only `crate = <path>` is supported"))
        }) {
            return e.into_compile_error();
        }
    }

    let krate = &container.krate;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    if fields.is_empty() {
        return quote! {
            #[automatically_derived]
            impl #impl_generics #krate::merge::Merge for #ident #ty_generics #where_clause {
                fn merge(&mut self, other: Self) {
                    let _ = other;
                }
            }
        };
    }

    let mut assignments = Vec::with_capacity(fields.len());
    for s_field in fields
        .iter()
        .enumerate()
        .map(|(idx, field)| StructField::from((idx, field.clone())))
    {
        let mut field = Field::default();
        for attr in s_field.attrs {
            if !attr.path().is_ident("merge") {
                continue;
            }

            if let Meta::List(list) = &attr.meta {
                if list.tokens.is_empty() {
                    continue;
                }
            }

            if let Err(e) = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip") {
                    if field.skipped {
                        return Err(syn::Error::new(meta.path.span(), "field already has `#[merge(skip)]`"));
                    }

                    field.skipped = true;
                    return Ok(());
                }

                if meta.path.is_ident("strategy") {
                    let input = meta.value()?;
                    if field.strategy.is_some() {
                        return Err(syn::Error::new(
                            meta.path.span(),
                            "field already has `#[merge(strategy)]`",
                        ));
                    }

                    field.strategy = Some(input.parse()?);
                    return Ok(());
                }

                Err(meta.error("unknown field, expected either `skip`, `strategy`"))
            }) {
                return e.into_compile_error();
            }
        }

        if field.skipped {
            continue;
        }

        let name = &s_field.member;
        assignments.push(match field.strategy {
            Some(path) => quote_spanned!(path.span()=> #path(&mut self.#name, other.#name)),
            None => quote_spanned!(s_field.span=> #krate::merge::Merge::merge(&mut self.#name, other.#name)),
        });
    }

    quote! {
        #[automatically_derived]
        impl #impl_generics #krate::merge::Merge for #ident #ty_generics #where_clause {
            fn merge(&mut self, other: Self) {
                #(#assignments;)*
            }
        }
    }
}

#[allow(unused)]
pub fn expand_enumeration() -> TokenStream {
    // TODO(@auguwu): experiment on how to expand `Merge` for `enum`s.
    todo!()
}
