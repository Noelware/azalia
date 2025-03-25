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

//! Module that contains the `Merge` trait, a way to merge two values
//! into one.

pub mod strategy;

use crate::libstd::{
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32, NonZeroU64,
        NonZeroU8, NonZeroUsize,
    },
    String,
};

#[cfg(feature = "macros")]
pub use azalia_config_macros::Merge;

/// Trait that allows deep merging between the same **type** but possibly different values.
///
/// ***This is also a derive macro when the `macros` feature is avaliable:
/// <code>#[derive([`Merge`][derive-redirect])]</code>***
///
/// ## Notes
/// When using the derive macro, the crate path will always be **azalia::config**. You can
/// set `crate = <path>` when <code>#[derive([`Merge`][derive-redirect])]</code>:
///
/// ```ignore
/// #[merge(crate = some::other::crate)]
/// ```
///
/// If you're using the standalone crate (`azalia-config`), this is unfortunately required
/// as the proc-macro doesn't understand the dependency tree of the project and since is mainly
/// for Noelware's use case, we use the centeralised crate approach.
///
/// ## Example
/// ```
/// # const _: &str = stringify! {
/// use azalia_config::merge::Merge;
///
/// #[derive(Merge)]
/// # #[merge(crate = azalia_config)]
/// pub struct Wrapper(u64);
/// # };
/// ```
///
/// [derive-redirect]: ../merge/derive.Merge.html
pub trait Merge: Sized {
    fn merge(&mut self, other: Self);
}

impl Merge for () {
    fn merge(&mut self, _: Self) {}
}

impl<T> Merge for Option<T> {
    fn merge(&mut self, mut other: Self) {
        if !self.is_some() {
            *self = other.take();
        }
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
/// The implementation for this type is very loose and will only extend.
///
/// Check out the [`strategy::vec`] module for other strategies.
impl<T> Merge for crate::libstd::Vec<T> {
    fn merge(&mut self, other: Self) {
        strategy::vec::extend(self, other);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
/// The implementation for this type is very loose and will only extend.
///
/// Check out the [`strategy::maps::btreemap`] module for other strategies.
impl<K: Ord, V> Merge for crate::libstd::BTreeMap<K, V> {
    fn merge(&mut self, other: Self) {
        strategy::maps::btreemap::extend(self, other);
    }
}

#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
/// The implementation for this type is very loose and will only extend.
///
/// Check out the [`strategy::sets::btreeset`] module for other strategies.
impl<T: Ord> Merge for crate::libstd::BTreeSet<T> {
    fn merge(&mut self, other: Self) {
        strategy::sets::btreeset::extend(self, other);
    }
}

#[cfg(feature = "std")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "std")))]
/// The implementation for this type is very loose and will only extend.
///
/// Check out the [`strategy::maps::hashmap`] module for other strategies.
impl<K: crate::libstd::Hash + Eq, V> Merge for crate::libstd::HashMap<K, V> {
    fn merge(&mut self, other: Self) {
        strategy::maps::hashmap::extend(self, other);
    }
}

#[cfg(feature = "std")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "std")))]
/// The implementation for this type is very loose and will only extend.
///
/// Check out the [`strategy::sets::btreeset`] module for other strategies.
impl<T: crate::libstd::Hash + Eq> Merge for crate::libstd::HashSet<T> {
    fn merge(&mut self, other: Self) {
        strategy::sets::hashset::extend(self, other);
    }
}

/// The implementation for this type is very loose and will only compare
/// between non-negative floating points.
///
/// Check out the [`strategy::f32`] module for other strategies.
impl Merge for f32 {
    fn merge(&mut self, other: Self) {
        strategy::f32::without_negative(self, other);
    }
}

/// The implementation for this type is very loose and will only compare
/// between non-negative floating points.
///
/// Check out the [`strategy::f64`] module for other strategies.
impl Merge for f64 {
    fn merge(&mut self, other: Self) {
        strategy::f64::without_negative(self, other);
    }
}

macro_rules! impl_unsigned_int {
    ($($Ty:ty)+) => {
        $(impl Merge for $Ty {
            fn merge(&mut self, other: Self) {
                // fast path #1: self == 0 && other == 0 = do not merge
                if *self == 0 && other == 0 {
                    return;
                }

                // fast path #2: self != 0 && other == 0 = do not merge
                if *self != 0 && other == 0 {
                    return;
                }

                // fast path #3: self == 0 && other > 0 = merge!
                if *self == 0 && other > 0 {
                    *self = other;
                }

                // slow path: comparsion
                if *self != other {
                    *self = other;
                }
            }
        })*
    };
}

impl_unsigned_int!(u8 u16 u32 u64 u128 usize);

macro_rules! impl_nonzero {
    ($($Ty:ty)+) => {
        $(impl Merge for $Ty {
            fn merge(&mut self, other: Self) {
                if self.get() != other.get() {
                    // Safety: we are guaranteed that we are not zero! If zero is provided
                    // then it is not the library's fault.
                    *self = unsafe { <$Ty>::new_unchecked(other.get()) };
                }
            }
        })*
    };
}

impl_nonzero!(
    NonZeroI8
    NonZeroI16
    NonZeroI32
    NonZeroI64
    NonZeroI128
    NonZeroIsize

    NonZeroU8
    NonZeroU16
    NonZeroU32
    NonZeroU64
    NonZeroUsize
);

macro_rules! impl_partialeq {
    (
        $(
            $(#[$meta:meta])*
            $Ty:ty
        )*
    ) => {
        $(
            $(#[$meta])*
            impl Merge for $Ty {
                fn merge(&mut self, other: Self) {
                    if *self != other {
                        *self = other;
                    }
                }
            }
        )*
    };
}

impl_partialeq!(
    i8
    i16
    i32
    i64
    i128
    isize

    /// The implementation for this type is very loose and will only do comparisons.
    ///
    /// Check out the [`strategy::bool`] module for other strategies.
    bool

    /// The implementation for this type is very loose and will only do comparisons.
    ///
    /// Check out the [`strategy::string`] module for other strategies.
    String

    #[cfg(feature = "std")]
    std::path::PathBuf

    #[cfg(feature = "url")]
    url::Url
);
