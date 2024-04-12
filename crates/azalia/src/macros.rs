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

/// Easily create a [`HashMap`][std::collections::HashMap].
///
/// ## Example
/// ```rust
/// # use azalia::hashmap;
/// #
/// let mut map = hashmap!(&str, &str);
/// map.insert("hello", "world");
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! hashmap {
    ($K:ty, $V:ty, { $($key:expr => $value:expr),* }) => {{
        let mut map = $crate::hashmap!($K, $V);
        $(
            map.insert(From::from($key), From::from($value));
        )*

        map
    }};

    ($K:ty, $V:ty) => {{
        ::std::collections::HashMap::<$K, $V>::new()
    }};

    ($($key:expr => $value:expr),*) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert(From::from($key), From::from($value));
        )*

        map
    }};

    () => {{
        ::std::collections::HashMap::new()
    }};
}

/// Easily create a [`HashSet`][std::collections::HashSet].
///
/// ## Example
/// ```rust
/// # use azalia::hashset;
/// #
/// let mut map = hashset!(&str);
/// map.insert("hello");
/// map.insert("world");
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! hashset {
    () => {{
        ::std::collections::HashSet::new()
    }};

    ($V:ty) => {{
        ::std::collections::HashSet::<$V>::new()
    }};

    ($($value:expr),*) => {{
        let mut set = ::std::collections::HashSet::new();
        $(set.insert(From::from($value));)*

        set
    }};
}

/// Easily create a [`BTreeMap`][std::collections::BTreeMap].
///
/// ## Example
/// ```rust
/// # use azalia::btreemap;
/// #
/// let mut map = btreemap!(&str, &str);
/// map.insert("hello", "world");
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! btreemap {
    ($K:ty, $V:ty, { $($key:expr => $value:expr),* }) => {{
        let mut map = $crate::btreemap!($K, $V);
        $(
            map.insert(From::from($key), From::from($value));
        )*

        map
    }};

    ($K:ty, $V:ty) => {{
        ::std::collections::BTreeMap::<$K, $V>::new()
    }};

    ($($key:expr => $value:expr),*) => {{
        let mut map = ::std::collections::BTreeMap::new();
        $(
            map.insert(From::from($key), From::from($value));
        )*

        map
    }};

    () => {{
        ::std::collections::BTreeMap::new()
    }};
}

/// Easily create a [`BTreeMap`][alloc::collections::BTreeMap].
///
/// ## Example
/// ```rust
/// # use azalia::btreemap;
/// #
/// let mut map = btreemap!(&str, &str);
/// map.insert("hello", "world");
/// ```
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! btreemap {
    ($K:ty, $V:ty, { $($key:expr => $value:expr),* }) => {{
        let mut map = $crate::btreemap!($K, $V);
        $(
            map.insert(From::from($key), From::from($value));
        )*

        map
    }};

    ($K:ty, $V:ty) => {{
        ::alloc::collections::BTreeMap::<$K, $V>::new()
    }};

    ($($key:expr => $value:expr),*) => {{
        let mut map = ::alloc::collections::BTreeMap::new();
        $(
            map.insert(From::from($key), From::from($value));
        )*

        map
    }};

    () => {{
        ::alloc::collections::BTreeMap::new()
    }};
}

/// Easily create a [`HashSet`][std::collections::HashSet].
///
/// ## Example
/// ```rust
/// # use azalia::btreeset;
/// #
/// let mut map = btreeset!(&str);
/// map.insert("hello");
/// map.insert("world");
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! btreeset {
    () => {{
        ::std::collections::BTreeSet::new()
    }};

    ($V:ty) => {{
        ::std::collections::BTreeSet::<$V>::new()
    }};

    ($($value:expr),*) => {{
        let mut set = ::std::collections::BTreeSet::new();
        $(set.insert(From::from($value));)*

        set
    }};
}

/// Easily create a [`HashSet`][alloc::collections::HashSet].
///
/// ## Example
/// ```rust
/// # use azalia::btreeset;
/// #
/// let mut map = btreeset!(&str);
/// map.insert("hello");
/// map.insert("world");
/// ```
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! btreeset {
    ($(value:expr),*) => {{
        let mut set = $crate::btreeset!();
        $(set.insert(From::from($value));)*

        set
    }};

    ($V:ty, [$($value:expr),*]) => {{
        let mut set = $crate::btreeset!($V);
        $(set.insert($value);)*

        set
    }};

    ($V:ty) => {{
        ::alloc::collections::BTreeSet::<$V>::new()
    }};

    () => {{
        ::alloc::collections::BTreeSet::new()
    }};
}
