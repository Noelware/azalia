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

/// Creates a [`regex::Regex`] object from a given regular expression, if it fails, then
/// the whole program will crash.
#[cfg(feature = "regex")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "regex")))]
#[macro_export]
macro_rules! regex {
    ($regex:literal) => {
        match ::regex::Regex::new($regex) {
            Ok(regex) => regex,
            Err(_e) => unreachable!("expected regex::Regex call to succeed"),
        }
    };

    ($regex:expr) => {
        match ::regex::Regex::new($regex) {
            Ok(regex) => regex,
            Err(_e) => unreachable!("expected regex::Regex call to succeed"),
        }
    };
}

#[cfg(any(feature = "lazy", feature = "use-once-cell"))]
pub(crate) use once_cell::{sync::Lazy as LazySync, unsync::Lazy as UnsyncLazy};

#[cfg(all(
    not(any(feature = "lazy", feature = "use-once-cell")),
    any(feature = "std", feature = "alloc")
))]
#[allow(unused)]
pub(crate) use std::{cell::LazyCell as UnsyncLazy, sync::LazyLock as LazySync};

#[cfg(all(
    any(feature = "lazy", feature = "use-once-cell"),
    any(feature = "std", feature = "alloc")
))]
#[cfg_attr(
    any(noeldoc, docsrs),
    doc(cfg(all(
        any(feature = "lazy", feature = "use-once-cell"),
        any(feature = "std", feature = "alloc")
    )))
)]
#[macro_export]
macro_rules! lazy {
    ($code:expr) => {
        $crate::lazy!(@internal sync || $code)
    };

    (unsync { $code:expr }) => {
        $crate::lazy!(@internal unsync || $code)
    };

    // INTERNAL \\
    (@internal sync || $code:expr) => {
        $crate::macros::LazySync::new(|| $code)
    };

    (@internal unsync || $code:expr) => {
        $crate::macros::UnsyncLazy::new(|| $code)
    };
}

#[allow(unused)]
pub(crate) trait IsInteger: Sized {}
macro_rules! impl_is_integer {
    ($($ty:ty)*) => {
        $(impl IsInteger for $ty {})*
    };
}

impl_is_integer!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize);

/// Creates a [`HashMap`](std::collections::HashMap) easily.
#[cfg(feature = "std")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "std")))]
#[macro_export]
macro_rules! hashmap {
    // hashmap!() :: create empty hashmap
    () => {
        ::std::collections::HashMap::new()
    };

    ($len:literal) => {
        $crate::hashmap!(@internal $len)
    };

    ($($key:expr => $value:expr),*) => {{
        let mut map = $crate::hashmap!();
        $(map.insert(($key).into(), ($value).into());)*

        map
    }};

    ($len:literal; { $($key:expr => $value:expr),* }) => {{
        let mut map = $crate::hashmap!($len);
        $(map.insert(($key).into(), ($value).into());)*

        map
    }};

    // hashmap!(&str, &str) :: create empty hash map from pre-defined types
    ($K:ty, $V:ty) => {
        ::std::collections::HashMap::<$K, $V>::new()
    };

    // hashmap!(&str, &str: len) :: calls `HashMap::with_capacity(<len>)`
    ($K:ty, $V:ty: $len:literal) => {
        $crate::hashmap!(@internal $K, $V: $len)
    };

    // hashmap!(&str, &str: len; {}) :: calls `HashMap::with_capacity(<len>)` and prepends each
    // element in the block into the hashmap
    ($K:ty, $V:ty: $len:literal; { $($key:expr => $value:expr),* }) => {{
        let mut map = $crate::hashmap!($K, $V: $len);
        $(map.insert(($key).into(), ($value).into());)*

        map
    }};

    // INTERNAL \\
    (@internal $capacity:literal) => {{
        const fn __is_integer<X: $crate::macros::IsInteger>(_: &X) {}
        __is_integer(&$capacity);

        ::std::collections::HashMap::with_capacity($capacity)
    }};

    (@internal $K:ty, $V:ty: $capacity:literal) => {{
        const fn __is_integer<X: $crate::macros::IsInteger>(_: &X) {}
        __is_integer(&$capacity);

        ::std::collections::HashMap::<$K, $V>::with_capacity($capacity)
    }};
}

/// Creates a [`HashSet`](std::collections::HashSet) easily.
#[cfg(feature = "std")]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "std")))]
#[macro_export]
macro_rules! hashset {
    () => {
        ::std::collections::HashSet::new()
    };

    ($len:literal) => {
        $crate::hashset!(@internal $len)
    };

    ($T:ty) => { ::std::collections::HashSet::<$T>::new() };
    ($T:ty: $len:literal) => {
        $crate::hashset!(@internal $T: $len)
    };

    ($T:ty: [$($value:expr),*]) => {{
        let mut set = $crate::hashset!($T);
        $(set.insert(($value).into());)*

        set
    }};

    ($T:ty: $len:literal; []) => { $crate::hashset!($T: $len) };

    ($T:ty: $len:literal; [$($value:expr),+]) => {{
        let mut set = $crate::hashset!($T: $len);
        $(set.insert(($value).into());)*

        set
    }};

    ($($value:expr),*) => {{
        let mut set = $crate::hashset!();
        $(set.insert(($value).into());)*

        set
    }};

    ($len:literal; []) => { $crate::hashset!($len) };

    ($len:literal; [$($value:expr),+]) => {{
        let mut set = $crate::hashset!($len; []);
        $(set.insert(($value).into());)*

        set
    }};

    // INTERNAL \\
    (@internal $capacity:literal) => {{
        const fn __is_integer<X: $crate::macros::IsInteger>(_: &X) {}
        __is_integer(&$capacity);

        ::std::collections::HashSet::with_capacity($capacity)
    }};

    (@internal $V:ty: $capacity:literal) => {{
        const fn __is_integer<X: $crate::macros::IsInteger>(_: &X) {}
        __is_integer(&$capacity);

        ::std::collections::HashSet::<$V>::with_capacity($capacity)
    }};
}

/// Create a [`BTreeMap`](crate::libstd::BTreeMap) easily.
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
#[macro_export]
macro_rules! btreemap {
    () => {
        $crate::libstd::BTreeMap::new()
    };

    ($($key:expr => $value:expr),*) => {{
        let mut map = $crate::btreemap!();
        $(map.insert(($key).into(), ($value).into());)*

        map
    }};

    ($K:ty, $V:ty) => {
        $crate::libstd::BTreeMap::<$K, $V>::new()
    };

    ($K:ty, $V:ty: { $($key:expr => $value:expr),* }) => {{
        let mut map = $crate::btreemap!($K, $V);
        $(map.insert(($key).into(), ($value).into());)*

        map
    }};
}

/// Create a [`BTreeSet`](crate::libstd::BTreeSet) easily.
#[cfg(any(feature = "std", feature = "alloc"))]
#[cfg_attr(any(noeldoc, docsrs), doc(cfg(any(feature = "std", feature = "alloc"))))]
#[macro_export]
macro_rules! btreeset {
    () => {
        $crate::libstd::BTreeSet::new()
    };

    ($T:ty) => {
        $crate::libstd::BTreeSet::<$T>::new()
    };

    ($($value:expr),*) => {{
        let mut set = $crate::btreeset!();
        $(set.insert(($value).into());)*

        set
    }};

    ($T:ty: [$($value:expr),*]) => {{
        let mut set = $crate::btreeset!($T);
        $(set.insert(($value).into());)*

        set
    }};
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "std")]
    #[test]
    fn test_hashmap_impls() {
        use std::collections::HashMap;

        // without type declaring
        let _: HashMap<String, String> = crate::hashmap!();
        let _: HashMap<String, String> = crate::hashmap!(2);
        let _: HashMap<String, String> = crate::hashmap! { "hello" => "world" };
        let _: HashMap<String, String> = crate::hashmap!(2; { "hello" => "world" });

        // with type declaring
        let _ = crate::hashmap!(String, String);
        let _ = crate::hashmap!(String, String: 2);
        let _ = crate::hashmap!(String, String: 2; { "hello" => "world" });
    }

    #[cfg(feature = "std")]
    #[test]
    fn test_hashset_impls() {
        use std::collections::HashSet;

        let _: HashSet<String> = hashset!();
        let _: HashSet<String> = hashset!(2);
        let _: HashSet<String> = hashset!["hello", "world"];
        let _: HashSet<String> = hashset!(2; ["hello", "world"]);

        let _ = crate::hashset!(String);
        let _ = crate::hashset!(String: 32);
        let _ = crate::hashset!(String: ["hello"]);
        let _ = crate::hashset!(String: 32; ["hello", "world"]);
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn test_btreemap_impls() {
        use crate::libstd::BTreeMap;

        let _: BTreeMap<String, String> = btreemap!();
        let _: BTreeMap<String, String> = btreemap! { "hello" => "world" };

        let _ = btreemap!(String, String);
        let _ = btreemap!(String, String: { "hello" => "world" });
    }

    #[cfg(any(feature = "std", feature = "alloc"))]
    #[test]
    fn test_btreeset_impls() {
        use crate::libstd::BTreeSet;

        let _: BTreeSet<String> = btreeset!();
        let _: BTreeSet<String> = btreeset!["hello", "world"];

        let _ = btreeset!(String);
        let _ = btreeset!(String: ["hello", "world"]);
    }
}
