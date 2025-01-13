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

use syn::{
    parse::{Parse, ParseStream},
    Path, Result, Token,
};

#[derive(Clone)]
pub struct Container {
    pub krate: Path,
}

impl Default for Container {
    fn default() -> Self {
        Container {
            krate: syn::parse_quote!(::azalia::config),
        }
    }
}

impl Parse for Container {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut me = Container::default();
        if input.peek(Token![crate]) {
            input.parse::<Token![crate]>()?;
            input.parse::<Token![=]>()?;

            me.krate = input.parse()?;
        }

        Ok(me)
    }
}

#[derive(Default)]
pub struct Args {
    pub is_skipped: bool,
    pub strategy: Option<Path>,
}

mod kw {
    syn::custom_keyword!(strategy);
    syn::custom_keyword!(skip);
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut args = Args::default();

        if input.peek(kw::skip) {
            input.parse::<kw::skip>()?;
            args.is_skipped = true;

            return Ok(args);
        }

        if input.peek(kw::strategy) {
            input.parse::<kw::strategy>()?;
            input.parse::<Token![=]>()?;

            args.strategy = Some(input.parse()?);
        }

        Ok(args)
    }
}
