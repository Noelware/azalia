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

use crate::merge::Path;
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    DeriveInput, Expr, ExprLit, ExprPath, Fields, Ident, Lit, LitStr, PathSegment, Token, TypePath,
};

pub enum VariableKind {
    Literal(String),
    Path(ExprPath),
}

impl VariableKind {
    pub(crate) fn is_empty(&self) -> bool {
        match self {
            VariableKind::Literal(s) => s.is_empty(),
            VariableKind::Path(s) => s.path.segments.len() == 0,
        }
    }
}

impl Default for VariableKind {
    fn default() -> Self {
        VariableKind::Literal(String::new())
    }
}

impl Parse for VariableKind {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        match input.parse::<Expr>()? {
            Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) => Ok(VariableKind::Literal(s.value())),
            Expr::Path(p) => Ok(VariableKind::Path(p)),
            expr => Err(syn::Error::new(expr.span(), "unexpected value")),
        }
    }
}

/// **#\[env\]** for an individual field.
#[derive(Default)]
pub struct Field {
    pub variable: VariableKind,

    /// **#\[env(parse = "my_parser")\]** | **#\[env(parse = my_parser)\]**
    pub parser: Option<Path>,

    /// **#\[env(default)\]** | **#\[env(default = "default")\]** | **#\[env(default = default)\]**
    pub default: Option<Path>,
}

pub fn expand_struct(
    DeriveInput {
        ident, generics, attrs, ..
    }: &DeriveInput,
    fields: &Fields,
) -> syn::Result<TokenStream> {
    if !generics.params.is_empty() {
        return Err(syn::Error::new(
            generics.span(),
            "generics are not allowed with `TryFromEnv`",
        ));
    }

    let mut error_ty = None;
    let mut prefix = None;
    let mut krate = syn::Path {
        leading_colon: Some(Token![::](Span::call_site())),
        segments: [
            PathSegment::from(Ident::new("azalia", Span::call_site())),
            PathSegment::from(Ident::new("config", Span::call_site())),
        ]
        .into_iter()
        .collect(),
    };

    for attr in attrs {
        if !attr.path().is_ident("env") {
            continue;
        }

        let list = attr.meta.require_list()?;
        if list.tokens.is_empty() {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("error") {
                if error_ty.is_some() {
                    return Err(meta.error("error type was already spciefied"));
                }

                error_ty = meta.value()?.parse::<TypePath>().map(Some)?;
                return Ok(());
            }

            if meta.path.is_ident("prefix") {
                if prefix.is_some() {
                    return Err(meta.error("`prefix` setting was already set"));
                }

                prefix = meta.value()?.parse::<LitStr>().map(|s| Some(s.value()))?;
                return Ok(());
            }

            if meta.path.is_ident("crate") {
                krate = meta.value()?.parse()?;
                return Ok(());
            }

            Err(meta.error("unsupported syntax"))
        })?;

        if error_ty.is_none() {
            return Err(syn::Error::new(attr.span(), "expected a error type"));
        }
    }

    let error_ty = error_ty.take();
    debug_assert!(error_ty.is_some());

    // Safety: we checked if the call of `Option::take` is `Some(..)` and I
    // don't really want to cause a panic here if it fails as the debug_assert!
    // call will do so anyway.
    let error_ty = unsafe { error_ty.as_ref().unwrap_unchecked() };
    if fields.is_empty() {
        return Ok(quote! {
            #[automatically_derived]
            impl #krate::env::TryFromEnv for #ident {
                type Error = #error_ty;

                fn try_from_env() -> ::core::result::Result<Self, Self::Error> {
                    ::core::result::Result::Ok(Self)
                }
            }
        });
    }

    let mut assignments = Vec::with_capacity(fields.len());
    for field in fields {
        let Some(ref ident) = field.ident else {
            return Err(syn::Error::new(field.span(), "tuple-based fields are not supported"));
        };

        let mut settings = Field::default();
        for attr in &field.attrs {
            if !attr.path().is_ident("env") {
                continue;
            }

            let list = attr.meta.require_list()?;
            if list.tokens.is_empty() {
                continue;
            }

            list.parse_nested_meta(|meta| {
                if meta.path.is_ident("var") {
                    if !settings.variable.is_empty() {
                        return Err(meta.error("variable is already passed in"));
                    }

                    settings.variable = meta.value()?.parse()?;
                    return Ok(());
                }

                if meta.path.is_ident("parse") {
                    if settings.parser.is_some() {
                        return Err(meta.error("`parse` setting is already set"));
                    }

                    settings.parser = Some(meta.value()?.parse()?);
                    return Ok(());
                }

                if meta.path.is_ident("default") {
                    if settings.default.is_some() {
                        return Err(meta.error("`default` setting is already set"));
                    }

                    if meta.input.is_empty() {
                        // core::default::Default::default
                        settings.default = Some(Path(ExprPath {
                            attrs: Vec::new(),
                            qself: None,
                            path: syn::Path {
                                leading_colon: Some(Token![::](Span::call_site())),
                                segments: [
                                    PathSegment::from(Ident::new("core", Span::call_site())),
                                    PathSegment::from(Ident::new("default", Span::call_site())),
                                    PathSegment::from(Ident::new("Default", Span::call_site())),
                                    PathSegment::from(Ident::new("default", Span::call_site())),
                                ]
                                .into_iter()
                                .collect(),
                            },
                        }));

                        return Ok(());
                    }

                    let value = meta.value()?;
                    settings.default = Some(value.parse()?);

                    return Ok(());
                }

                Err(meta.error("unexpected syntax"))
            })?;
        }

        let env_var_call = match settings.variable {
            VariableKind::Path(p) => match prefix.as_ref() {
                Some(inner) => {
                    quote_spanned! {p.span()=> ::std::env::var(format!("{}{sep}{}", #p, #inner, if (#inner).ends_with('_') { "_" } else { "" }))}
                }

                None => quote_spanned! {p.span()=> ::std::env::var(#p)},
            },

            VariableKind::Literal(s) => quote!(::std::env::var(#s)),
        };

        let error_handler = match &settings.default {
            Some(path) => quote_spanned! {path.span()=>
                ::core::result::Result::Err(::std::env::VarError::NotPresent) => #path,
                ::core::result::Result::Err(e) => return ::core::convert::From::from(e)
            },

            None => quote!(::core::result::Result::Err(e) => return ::core::convert::From::from(e)),
        };

        let ty = &field.ty;
        let parser = match (settings.parser, settings.default) {
            (Some(path), Some(default)) => quote_spanned! {path.span()=>
                {
                    match #path(input) {
                        ::core::result::Result::Ok(value) => value,
                        ::core::result::Result::Err(_) => #default(),
                    }
                }
            },

            (Some(path), None) => quote_spanned! {path.span()=> #path(input)?},
            (None, Some(default)) => quote! {
                match <#ty as #krate::env::TryFromEnvValue>::try_from_env_value(input) {
                    ::core::result::Result::Ok(value) => value,
                    ::core::result::Result::Err(_) => #default(),
                }
            },

            (None, None) => quote!(<#ty as #krate::env::TryFromEnvValue>::try_from_env_value(input)?),
        };

        assignments.push(quote! {
            #ident: match #env_var_call {
                ::core::result::Result::Ok(input) => #parser,
                #error_handler
            }
        });
    }

    Ok(quote! {
        #[automatically_derived]
        impl #krate::env::TryFromEnv for #ident {
            type Error = #error_ty;

            fn try_from_env() -> ::core::result::Result<Self, Self::Error> {
                Ok(#ident {
                    #(#assignments,)*
                })
            }
        }
    })
}
