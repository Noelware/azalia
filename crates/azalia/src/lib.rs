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

//! # 🐻‍❄️🪚 `azalia`
//! Azalia is a family of crated maintained and used by [Noelware, LLC.] that implement common functionality
//! between all of Noelware's Rust codebases.
//!
//! This crate is a centeralised and easily consumable crate that can cherry-pick Azalia crates that you
//! need under a common module (i.e, `azalia-remi` -> `azalia::remi`) via Cargo's crate features feature.
//!
//! [Noelware, LLC.]: https://noelware.org

#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]
#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

pub mod rust;

mod macros;
mod util;

pub use util::*;

#[cfg(feature = "config")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "config")))]
pub use azalia_config as config;

#[cfg(feature = "std")]
<<<<<<< HEAD
#[doc(hidden)]
pub mod libstd {
    pub use std::{
        any,
        borrow::Cow,
        boxed::Box,
        collections::{BTreeMap, BTreeSet},
        rc::Rc,
        sync::Arc,
    };
=======
use std::any::Any;

#[cfg(not(feature = "std"))]
use core::any::Any;

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;

mod macros;
pub mod rust;

#[cfg(all(feature = "regex", no_lazy_lock))]
pub static TRUTHY_REGEX: ::once_cell::sync::Lazy<::regex::Regex> =
    crate::lazy!(::regex::Regex::new(r#"^(yes|true|si*|e|enable|1)$"#).unwrap());

#[cfg(all(feature = "regex", feature = "lazy", not(no_lazy_lock)))]
pub static TRUTHY_REGEX: ::once_cell::sync::Lazy<::regex::Regex> =
    crate::lazy!(::regex::Regex::new(r#"^(yes|true|si*|e|enable|1)$"#).unwrap());

#[cfg(all(feature = "regex", feature = "lazy", no_lazy_lock))]
pub static TRUTHY_REGEX: ::once_cell::sync::Lazy<::regex::Regex> =
    crate::lazy!(::regex::Regex::new(r#"^(yes|true|si*|e|enable|1)$"#).unwrap());

#[cfg(all(feature = "regex", not(no_lazy_lock)))]
pub static TRUTHY_REGEX: ::std::sync::LazyLock<::regex::Regex, _> =
    ::std::sync::LazyLock::new(|| ::regex::Regex::new(r#"^(yes|true|si*|e|enable|1)$"#).unwrap());

/// Returns a <code>[`Cow`]<'static, [`str`]></code> of a panic message, probably from [`std::panic::catch_unwind`].
pub fn message_from_panic(error: Box<dyn Any + Send + 'static>) -> Cow<'static, str> {
    if let Some(msg) = error.downcast_ref::<String>() {
        Cow::Owned(msg.clone())
    } else if let Some(s) = error.downcast_ref::<&str>() {
        Cow::Borrowed(s)
    } else {
        Cow::Borrowed("unknown panic message received")
    }
>>>>>>> 3bc5952 (Disable trybuild tests, attempt to use `LazyLock` in Rust 1.80 or higher)
}

#[cfg(not(feature = "std"))]
#[doc(hidden)]
pub mod libstd {
    pub use core::any;

    #[cfg(feature = "alloc")]
    pub use alloc::{
        borrow::Cow,
        boxed::Box,
        collections::{BTreeMap, BTreeSet},
        rc::Rc,
        sync::Arc,
    };
}
