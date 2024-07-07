// 🐻‍❄️🪚 core-rs: Collection of Rust crates that are used by and built for Noelware's projects
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
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(rustdoc::broken_intra_doc_links)] // we use GitHub's alerts and rustdoc doesn't like them

#[cfg(not(feature = "std"))]
extern crate core as std;

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "config")]
#[cfg_attr(docsrs, doc(cfg(feature = "config")))]
pub use azalia_config as config;

#[cfg(feature = "log")]
#[cfg_attr(docsrs, doc(cfg(feature = "log")))]
pub use azalia_log as log;

#[cfg(feature = "remi")]
#[cfg_attr(docsrs, doc(cfg(feature = "remi")))]
pub use azalia_remi as remi;

#[cfg(feature = "serde")]
#[cfg_attr(docsrs, doc(cfg(feature = "serde")))]
pub use azalia_serde as serde;

#[cfg(feature = "std")]
use std::any::Any;

#[cfg(not(feature = "std"))]
use core::any::Any;

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(not(feature = "std"))]
use alloc::borrow::Cow;

mod macros;
pub mod rust;

#[cfg(all(feature = "lazy", feature = "regex"))]
pub static TRUTHY_REGEX: once_cell::sync::Lazy<regex::Regex> =
    crate::lazy!(regex::Regex::new(r#"^(yes|true|si*|e|enable|1)$"#).unwrap());

/// Returns a [`Cow`]<'static, [`str`]> of a panic message, probably from [`std::panic::catch_unwind`].
pub fn message_from_panic(error: Box<dyn Any + Send + 'static>) -> Cow<'static, str> {
    if let Some(msg) = error.downcast_ref::<String>() {
        Cow::Owned(msg.clone())
    } else if let Some(s) = error.downcast_ref::<&str>() {
        Cow::Borrowed(s)
    } else {
        Cow::Borrowed("unknown panic message received")
    }
}

#[cfg(test)]
mod tests {
    use super::{hashmap, hashset};

    #[cfg(feature = "lazy")]
    #[test]
    fn test_lazy_exprs() {
        let weow = crate::lazy!("weow");
        assert_eq!(*weow, "weow");
    }

    #[cfg(all(feature = "lazy", feature = "regex"))]
    #[test]
    fn test_truthy_values() {
        // force-init lazy value
        let regex: &regex::Regex = &*crate::TRUTHY_REGEX;

        assert!(regex.is_match("true"));
        assert!(regex.is_match("yes"));
        assert!(regex.is_match("si"));
        assert!(regex.is_match("siiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii"));
        assert!(regex.is_match("enable"));
        assert!(regex.is_match("1"));
    }

    #[test]
    fn test_hashmap_variants() {
        #[cfg(feature = "std")]
        use std::collections::HashMap;

        #[cfg(not(feature = "std"))]
        use alloc::collections::HashMap;

        let _map = hashmap!(String, String);
        let _map: HashMap<String, String> = hashmap! {
            "key" => "value"
        };

        let _map = hashmap!(String, String, {
            "hello" => "world",
            "weow" => "true"
        });
    }

    #[test]
    fn test_hashset_variants() {
        #[cfg(feature = "std")]
        use std::collections::HashSet;

        #[cfg(not(feature = "std"))]
        use alloc::collections::HashSet;

        let _set: HashSet<u32> = hashset!();
        let _set: HashSet<i32> = hashset!(1, 2, 3);
        let _set = hashset!(i32);
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_message_from_panic() {
        use super::message_from_panic;
        use std::panic::catch_unwind;

        fn __should_panic() {
            todo!()
        }

        assert_eq!(
            message_from_panic(catch_unwind(__should_panic).unwrap_err()),
            "not yet implemented"
        );
    }
}
