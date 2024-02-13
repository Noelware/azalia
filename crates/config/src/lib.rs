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
#![cfg_attr(feature = "no_std", no_std)]

pub mod merge;

// mainly used to import types from `core`/`alloc` if `no-std` feature is enabled, or just
// uses `std::*` imports if `no-std` is disabled.
#[cfg(feature = "no_std")]
pub(crate) mod std {
    pub mod convert {
        pub use core::convert::Infallible;
    }
}

/// Represents a way to import a type from the system's environment variables easily when
/// doing implicit conversions. This doesn't handle any use-case if it ever fails, if you
/// need that, then implement the [`TryFromEnv`] trait instead.
pub trait FromEnv: Sized {
    /// Concrete output type.
    type Output;

    /// Do an implicit conversion to return `Self::Output` with the system
    /// environment variables.
    fn from_env() -> Self::Output;
}

/// Represents a [`Result`]-based way to import a type from the system's environment variables
/// easy when doing implicit conversions.
pub trait TryFromEnv: Sized {
    /// Concrete output type.
    type Output;

    /// Error type.
    type Error;

    /// Do an implicit conversion that could possibly fail.
    fn try_from_env() -> Result<Self::Output, Self::Error>;
}

impl<O, T: FromEnv<Output = O>> TryFromEnv for T {
    type Output = O;
    type Error = std::convert::Infallible;

    fn try_from_env() -> Result<Self::Output, Self::Error> {
        Ok(T::from_env())
    }
}

// macro is originally the `env!` macro from charted-server
// original: https://github.com/charted-dev/charted/blob/94e6c9de95059a9f582c934e32d599031a920c18/crates/config/src/lib.rs#L110-L257
/// Generic Rust functional macro to help with locating an environment variable in the host machine.
///
/// ## Variants
/// ### `env!($key: literal)`
/// This will just expand `$key` into a Result<[`String`][alloc::string::String], [`VarError`][std::env::VarError]> variant.
///
/// ```
/// # use noelware_config::env;
/// #
/// let result = env!("SOME_ENV_VARIABLE");
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE");
/// #
/// # assert!(result.is_err());
/// ```
///
/// ### `env!($key: literal, is_optional: true)`
/// Expands the `$key` into a Option type if a [`VarError`][std::env::VarError] occurs.
///
/// ```
/// # use noelware_config::env;
/// #
/// let result = env!("SOME_ENV_VARIABLE", is_optional: true);
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE").ok();
/// #
/// # assert!(result.is_none());
/// ```
///
/// ### `env!($key: literal, or_else: $else: expr)`
/// Expands `$key` into a String, but if a [`VarError`][std::env::VarError] occurs, then a provided `$else`
/// is used as the default.
///
/// ```
/// # use noelware_config::env;
/// #
/// let result = env!("SOME_ENV_VARIABLE", or_else: "".into());
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE").unwrap_or("".into());
/// #
/// # assert!(result.is_empty());
/// ```
///
/// ### `env!($key: literal, or_else_do: $else: expr)`
/// Same as [`env!($key: literal, or_else: $else: expr)`][crate::var], but uses `.unwrap_or_else` to
/// accept a [`Fn`][std::ops::Fn].
///
/// ```
/// # use noelware_config::env;
/// #
/// let result = env!("SOME_ENV_VARIABLE", or_else_do: |_| Default::default());
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE").unwrap_or_else(|_| Default::default());
/// #
/// # assert!(result.is_empty());
/// ```
///
/// ### `env!($key: literal, use_default: true)`
/// Same as [`env!($key: literal, or_else_do: $else: expr)`][crate::var], but will use the
/// [Default][core::default::Default] implementation, if it can be resolved.
///
/// ```
/// # use noelware_config::env;
/// #
/// let result = env!("SOME_ENV_VARIABLE", use_default: true);
/// // expanded: ::std::env::var("SOME_ENV_VARIABLE").unwrap_or_else(|_| Default::default());
/// #
/// # assert!(result.is_empty());
/// ```
///
/// ### `env!($key: literal, mapper: $mapper: expr)`
/// Uses the [`.map`][result-map] method with an accepted `mapper` to map to a different type.
///
/// ```
/// # use noelware_config::env;
/// #
/// let result = env!("SOME_ENV_VARIABLE", mapper: |val| &val == "true");
///
/// /*
/// expanded:
/// ::std::env::var("SOME_ENV_VARIABLE").map(|val| &val == "true");
/// */
/// #
/// # assert!(result.is_err());
/// ```
///
/// [result-map]: https://doc.rust-lang.org/nightly/core/result/enum.Result.html#method.map
#[cfg(not(feature = "no-std"))]
#[macro_export]
macro_rules! env {
    ($key:expr, to: $ty:ty, or_else: $else_:expr) => {
        $crate::env!($key, mapper: |p| {
            p.parse::<$ty>().expect(concat!(
                "Unable to resolve env var [",
                $key,
                "] to a [",
                stringify!($ty),
                "] value"
            ))
        })
        .unwrap_or($else_)
    };

    ($key:expr, to: $ty:ty, is_optional: true) => {
        $crate::env!($key, mapper: |p| p.parse::<$ty>().ok()).unwrap_or(None)
    };

    ($key:expr, to: $ty:ty) => {
        $crate::env!($key, mapper: |p| {
            p.parse::<$ty>().expect(concat!(
                "Unable to resolve env var [",
                $key,
                "] to a [",
                stringify!($ty),
                "] value"
            ))
        })
        .unwrap()
    };

    ($key:expr, {
        or_else: $else_:expr;
        mapper: $mapper:expr;
    }) => {
        $crate::env!($key, mapper: $mapper).unwrap_or($else_)
    };

    ($key:expr, mapper: $expr:expr) => {
        $crate::env!($key).map($expr)
    };

    ($key:expr, use_default: true) => {
        $crate::env!($key, or_else_do: |_| Default::default())
    };

    ($key:expr, or_else_do: $expr:expr) => {
        $crate::env!($key).unwrap_or_else($expr)
    };

    ($key:expr, or_else: $else_:expr) => {
        $crate::env!($key).unwrap_or($else_)
    };

    ($key:expr, is_optional: true) => {
        $crate::env!($key).ok()
    };

    ($key:expr) => {
        ::std::env::var($key)
    };
}
