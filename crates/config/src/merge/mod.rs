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

//! Allows the use to merge two types into one.

pub mod strategy;

#[cfg(feature = "no-std")]
use core::num;

#[cfg(not(feature = "no-std"))]
use std::num;

#[cfg(feature = "derive")]
pub use azalia_config_derive::*;

/// Trait that allows you to merge together two objects into one easily. This can be used to your
/// advantage to allow deep merging.
///
/// ## Example
/// ```rust
/// # use azalia_config::merge::Merge;
/// #
#[cfg_attr(feature = "derive", doc = "#[derive(Merge)]")]
/// # #[merge(crate = ::azalia_config)]
/// pub struct MyWrapper(u64);
///
#[cfg_attr(not(feature = "derive"), doc = include_str!("../../merge_without_derive"))]
/// ```
pub trait Merge {
    /// Does the merging all-together by modifying `self` from `other`.
    fn merge(&mut self, other: Self);
}

impl Merge for () {
    fn merge(&mut self, _other: Self) {
        // do nothing here :>
    }
}

impl<T> Merge for Option<T> {
    fn merge(&mut self, mut other: Self) {
        if !self.is_some() {
            *self = other.take();
        }
    }
}

#[cfg(feature = "no-std")]
impl<T> Merge for alloc::vec::Vec<T> {
    fn merge(&mut self, other: Self) {
        strategy::vec::extend::<T>(self, other);
    }
}

#[cfg(not(feature = "no-std"))]
impl<T> Merge for Vec<T> {
    fn merge(&mut self, other: Self) {
        strategy::vec::extend(self, other);
    }
}

impl Merge for f32 {
    fn merge(&mut self, other: Self) {
        strategy::f32::non_negative(self, other);
    }
}

impl Merge for f64 {
    fn merge(&mut self, other: Self) {
        strategy::f64::non_negative(self, other);
    }
}

#[cfg(not(feature = "no-std"))]
impl<K: std::hash::Hash + Eq, V> Merge for std::collections::HashMap<K, V> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(not(feature = "no-std"))]
impl<T: std::hash::Hash + Eq> Merge for std::collections::HashSet<T> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(feature = "no-std")]
impl<K: core::cmp::Ord, V> Merge for alloc::collections::BTreeMap<K, V> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(not(feature = "no-std"))]
impl<K: std::cmp::Ord, V> Merge for std::collections::BTreeMap<K, V> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(feature = "no-std")]
impl<T: core::cmp::Ord> Merge for alloc::collections::BTreeSet<T> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(not(feature = "no-std"))]
impl<T: std::cmp::Ord> Merge for std::collections::BTreeSet<T> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

macro_rules! impl_unum_merge {
    ($($ty:ty),*) => {
        $(
            impl Merge for $ty {
                fn merge(&mut self, other: Self) {
                    // fast-path: if both are 0, then don't do anything
                    if *self == 0 && other == 0 {
                        return;
                    }

                    // fast path: self != 0 & other = 0
                    if *self != 0 && other == 0 {
                        return;
                    }

                    // fast path: if self is 0 and other is not, then override
                    if *self == 0 && other > 0 {
                        *self = other;
                        return;
                    }

                    // slow path: compare
                    if *self != other {
                        *self = other;
                    }
                }
            }
        )*
    };
}

macro_rules! impl_nonzero_merge {
    ($($ty:ty),*) => {
        $(
            impl Merge for $ty {
                fn merge(&mut self, other: Self) {
                    if self.get() != other.get() {
                        // SAFETY: we are guranteed that `other` is always NOT zero, if it is
                        //         zero then it is the caller's fault rather than ours :3
                        *self = unsafe { <$ty>::new_unchecked(other.get()) };
                    }
                }
            }
        )*
    };
}

macro_rules! impl_generic_partial_eq_merge {
    (
        $(
            $(#[$meta:meta])? $ty:ty
        ),*
    ) => {
        $(
            $(#[$meta])?
            impl Merge for $ty {
                fn merge(&mut self, other: Self) {
                    // do comparsions
                    if *self != other {
                        *self = other;
                    }
                }
            }
        )*
    };
}

impl_nonzero_merge!(
    num::NonZeroU8,
    num::NonZeroU16,
    num::NonZeroU32,
    num::NonZeroU64,
    num::NonZeroUsize,
    num::NonZeroU128
);

impl_nonzero_merge!(
    num::NonZeroI8,
    num::NonZeroI16,
    num::NonZeroI32,
    num::NonZeroI64,
    num::NonZeroI128,
    num::NonZeroIsize
);

impl_unum_merge!(u8, u16, u32, u64, u128, usize);

#[rustfmt::skip]
impl_generic_partial_eq_merge!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize, // numbers
    &str,  // strings
    bool,  // booleans

    #[cfg(feature = "url")]
    ::url::Url,

    #[cfg(feature = "no-std")]
    ::alloc::string::String,

    #[cfg(not(feature = "no-std"))]
    String,

    #[cfg(not(feature = "no-std"))]
    ::std::path::PathBuf,

    #[cfg(not(feature = "no-std"))]
    &::std::path::Path
);
