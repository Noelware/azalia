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

//! # 🐻‍❄️🪚 `azalia-config`
//! The **azalia_config** Rust crate defines types and utilities dealing with application configuration.
#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]
#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc;

#[cfg(feature = "std")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "std")))]
pub mod env;
pub mod merge;

#[cfg(feature = "std")]
pub(crate) mod libstd {
    pub use std::{
        collections::{BTreeMap, BTreeSet, HashMap, HashSet},
        hash::Hash,
        num,
        string::String,
        vec::Vec,
    };
}

#[cfg(not(feature = "std"))]
pub(crate) mod libstd {
    pub use core::num;

    #[cfg(feature = "alloc")]
    pub use alloc::{
        collections::{BTreeMap, BTreeSet},
        string::String,
        vec::Vec,
    };
}
