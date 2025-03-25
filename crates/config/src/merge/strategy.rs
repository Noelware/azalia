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

//! A module for defining common ways to merge types.

#[deprecated(
    since = "0.1.0",
    note = "used in old versions of azalia before crates.io release. scheduled for removal in v0.2"
)]
pub use string as strings;

/// Other strategies for merging strings. The default strategy will compare if `lhs != rhs`.
///
/// String slices are not supported in this module as string slices are considered immutable
/// and shouldn't be tampered with.
pub mod string {
    use crate::libstd::String;

    /// Grows and appends `rhs` into `lhs`.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::merge::strategy::strings::append;
    ///
    /// let mut a = String::from("hello");
    /// let b = String::from(", world!");
    ///
    /// append(&mut a, b);
    /// assert_eq!(a.as_str(), "hello, world!");
    /// ```
    pub fn append(left: &mut String, right: String) {
        left.push_str(&right);
    }

    /// Overwrites `lhs` into `rhs`.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::merge::strategy::strings::overwrite;
    ///
    /// let mut a = String::from("hi!");
    /// let b = String::from("overwritten...");
    ///
    /// overwrite(&mut a, b);
    /// assert_eq!(a.as_str(), "overwritten...");
    /// ```
    pub fn overwrite(left: &mut String, right: String) {
        *left = right;
    }

    /// Overwrites `rhs` into `lhs` if `lhs` is empty.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::merge::strategy::strings::overwrite_empty;
    ///
    /// let mut a = String::new();
    /// let b = String::from("overwritten!");
    ///
    /// overwrite_empty(&mut a, b);
    /// assert_eq!(a.as_str(), "overwritten!");
    /// ```
    pub fn overwrite_empty(left: &mut String, right: String) {
        if left.is_empty() {
            *left = right;
        }
    }
}

/// Other strategies for merging boolean values. The default strategy will compare if `lhs != rhs`.
pub mod bool {
    /// Merge `lhs <- rhs` if `lhs` == `false`.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::merge::strategy::bool::only_if_falsy;
    ///
    /// let mut x = false;
    ///
    /// only_if_falsy(&mut x, true);
    /// assert!(x);
    ///
    /// only_if_falsy(&mut x, false);
    /// assert!(x);
    /// ```
    pub fn only_if_falsy(lhs: &mut bool, rhs: bool) {
        if !*lhs {
            *lhs = rhs;
        }
    }
}

macro_rules! mk_floating_strategies {
    ($Ty:ty) => {
        #[doc = concat!("Merges any [`", stringify!($Ty), "`][prim@", stringify!($Ty), "] without allowing negatives")]
        /// to be allowed when merging.
        ///
        /// ## Example
        /// ```
        #[doc = concat!("use azalia_config::merge::strategy::", stringify!($Ty), "::without_negative;")]
        ///
        #[doc = concat!("let mut x: ", stringify!($Ty), " = 32.0;")]
        ///
        /// without_negative(&mut x, -32.3);
        /// assert_eq!(x, 32.0);
        ///
        /// without_negative(&mut x, 3.56);
        /// assert_eq!(x, 3.56);
        /// ```
        pub fn without_negative(lhs: &mut $Ty, rhs: $Ty) {
            if *lhs < 0.0 || rhs < 0.0 {
                return;
            }

            with_negatives(lhs, rhs);
        }

        #[doc = concat!("Merges any [`", stringify!($Ty), "`][prim@", stringify!($Ty), "] with allowing negatives")]
        /// to be allowed when merging.
        ///
        /// ## Example
        /// ```
        #[doc = concat!("use azalia_config::merge::strategy::", stringify!($Ty), "::with_negatives;")]
        ///
        #[doc = concat!("let mut x: ", stringify!($Ty), " = 32.0;")]
        ///
        /// with_negatives(&mut x, -32.3);
        /// assert_eq!(x, -32.3);
        ///
        /// with_negatives(&mut x, 3.56);
        /// assert_eq!(x, 3.56);
        /// ```
        pub fn with_negatives(lhs: &mut $Ty, rhs: $Ty) {
            // fast path #1: left == 0.0 && right == 0.0 = do not merge
            if *lhs == 0.0 && rhs == 0.0 {
                return;
            }

            // fast path #2: left != 0.0 && right == 0.0 = do not merge
            if *lhs != 0.0 && rhs == 0.0 {
                return;
            }

            if *lhs != rhs {
                *lhs = rhs;
            }
        }
    };
}

/// Other strategies for merging [`f32`][prim@f32] values. The default strategy will compare non-negative
/// floating point types.
pub mod f32 {
    mk_floating_strategies!(f32);
}

/// Other strategies for merging [`f64`][prim@f64] values. The default strategy will compare non-negative
/// floating point types.
pub mod f64 {
    mk_floating_strategies!(f64);
}

macro_rules! mk_collection_strategies {
    (
        $Ty:ty => ($($f:tt)*)
    ) => {
        /// Extends all of `rhs` into `lhs`.
        ///
        /// This doesn't move ***all*** elements into `lhs` and
        /// doesn't leaves `rhs` empty.
        pub fn extend<$($f)*>(lhs: &mut $Ty, rhs: $Ty) {
            lhs.extend(rhs);
        }

        /// Overwrite `lhs` -> `rhs`.
        pub fn overwrite<$($f)*>(lhs: &mut $Ty, rhs: $Ty) {
            *lhs = rhs;
        }
    };
}

/// Collection of strategies related to [`HashMap`](crate::libstd::HashMap) and [`BTreeMap`](crate::libstd::BTreeMap).
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), cfg(any(feature = "std", feature = "alloc")))]
pub mod maps {
    /// Other strategies for merging [`BTreeMap`](crate::libstd::BTreeMap)s. The default strategy will extend the
    /// collection.
    #[cfg(any(feature = "std", feature = "alloc"))]
    #[cfg_attr(any(noeldoc, docsrs), cfg(any(feature = "std", feature = "alloc")))]
    pub mod btreemap {
        use crate::libstd::BTreeMap;

        mk_collection_strategies!(BTreeMap<K, V> => (K: Ord, V));
    }

    /// Other strategies for merging [`HashMap`](crate::libstd::HashMap)s. The default strategy will extend the
    /// collection.
    #[cfg(feature = "std")]
    #[cfg_attr(any(noeldoc, docsrs), cfg(feature = "std"))]
    pub mod hashmap {
        use crate::libstd::{Hash, HashMap};

        mk_collection_strategies!(HashMap<K, V> => (K: Hash + Eq, V));
    }
}

/// Collection of strategies related to [`HashSet`](crate::libstd::HashSet) and [`BTreeSet`](crate::libstd::BTreeSet).
pub mod sets {
    /// Other strategies for merging [`BTreeSet`](crate::libstd::BTreeSet)s. The default strategy will extend the
    /// collection.
    pub mod btreeset {
        use crate::libstd::BTreeSet;

        mk_collection_strategies!(BTreeSet<T> => (T: Ord));

        /// Moves all the elements from `right` into `left`, this doesn't
        /// sort the elements or checks for uniqueness.
        pub fn append<T: Ord>(left: &mut BTreeSet<T>, mut right: BTreeSet<T>) {
            left.append(&mut right);
        }

        /// Append all elements from `rhs` if `lhs` is empty.
        pub fn overwrite_empty<T: Ord>(lhs: &mut BTreeSet<T>, mut right: BTreeSet<T>) {
            if lhs.is_empty() {
                lhs.append(&mut right);
            }
        }
    }

    /// Other strategies for merging [`HashSet`](crate::libstd::HashSet)s. The default strategy will extend the
    /// collection.
    pub mod hashset {
        use crate::libstd::{Hash, HashSet};

        mk_collection_strategies!(HashSet<T> => (T: Hash + Eq));
    }
}

/// Other strategies for merging [`Vec`]s. The default strategy will extend the
/// collection.
pub mod vec {
    use crate::libstd::Vec;

    mk_collection_strategies!(Vec<T> => (T));

    /// Moves all the elements from `right` into `left`, this doesn't
    /// sort the elements or checks for uniqueness.
    pub fn append<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        left.append(&mut right);
    }

    /// Append all elements from `rhs` if `lhs` is empty.
    pub fn overwrite_empty<T>(lhs: &mut Vec<T>, mut right: Vec<T>) {
        if lhs.is_empty() {
            lhs.append(&mut right);
        }
    }
}
