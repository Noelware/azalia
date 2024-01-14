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

//! Allows the use to merge two types into one.

pub mod strategy;

#[cfg(feature = "derive")]
pub use noelware_config_derive::*;

/// Trait that allows you to merge together two objects into one easily. This can be used to your
/// advantage to allow deep merging.
///
/// ## Example
/// ```ignore
/// # use noelware_config::merge::Merge;
/// #
/// pub struct MyWrapper(u64);
/// #
/// impl Merge for MyWrapper {
///     fn merge(&mut self, other: Self) {
///         *self.0 = other.0;
///     }
/// }
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

impl<T> Merge for Vec<T> {
    fn merge(&mut self, other: Self) {
        strategy::vec::extend(self, other);
    }
}

#[cfg(not(feature = "no_std"))]
impl<K: std::hash::Hash + Eq, V> Merge for std::collections::HashMap<K, V> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(not(feature = "no_std"))]
impl<T: std::hash::Hash + Eq> Merge for std::collections::HashSet<T> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(feature = "no_std")]
impl<K: core::cmp::Ord, V> Merge for alloc::collections::BTreeMap<K, V> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(not(feature = "no_std"))]
impl<K: std::cmp::Ord, V> Merge for std::collections::BTreeMap<K, V> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(feature = "no_std")]
impl<T: core::cmp::Ord> Merge for alloc::collections::BTreeSet<T> {
    fn merge(&mut self, other: Self) {
        self.extend(other);
    }
}

#[cfg(not(feature = "no_std"))]
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

macro_rules! impl_generic_partial_eq_merge {
    ($($ty:ty),*) => {
        $(
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

impl_unum_merge!(u8, u16, u32, u64, u128, usize);
impl_generic_partial_eq_merge!(
    i8, i16, i32, i64, i128, isize, // numbers
    String, &str // strings
);
