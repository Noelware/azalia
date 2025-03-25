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

//! # 🐻‍❄️🪚 `azalia-serde`
//! The **azalia-serde** crate provides blanket `serde` implementations for crates that don't expose any. This
//! uses Cargo's crate features to explicitly enable which implementations you need, rather than adding them all
//! at once.
//!
//! We only provide implementations to Rust types that are most used by us, so we will probably reject most
//! requests to add more types other than the ones listed.
//!
//! ## Usage
//! ### `tracing::Level` (requires `tracing` feature)
//! ```
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "azalia_serde::tracing")]
//!     level: tracing::Level,
//! }
//! ```
//!
//! ### `aws_types::types::Region` (requires `aws` feature)
//! ```
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize)]
//! struct MyStruct {
//!     #[serde(with = "azalia_serde::aws::region")]
//!     region: aws_sdk_s3::types::Region
//! }
// ```
#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![doc(html_favicon_url = "https://cdn.floofy.dev/images/trans.png")]
#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]

#[cfg(feature = "tracing")]
#[cfg_attr(any(docsrs, noeldoc), doc(cfg(feature = "tracing")))]
pub mod tracing;

#[cfg(feature = "aws")]
#[cfg_attr(any(docsrs, noeldoc), doc(cfg(feature = "aws")))]
pub mod aws;

#[cfg(feature = "s3")]
#[cfg_attr(any(docsrs, noeldoc), doc(cfg(feature = "s3")))]
pub mod s3;
