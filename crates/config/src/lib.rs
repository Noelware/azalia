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
#![cfg_attr(feature = "no-std", no_std)]
#![allow(rustdoc::broken_intra_doc_links)] // we use GitHub's alerts and rustdoc doesn't like them

pub mod merge;

#[cfg(feature = "no-std")]
extern crate core as std;

#[cfg(feature = "no-std")]
extern crate alloc;

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

#[cfg(not(feature = "no-std"))]
#[doc(hidden)]
mod env {
    /// Represents a guard that sets an environment variable and removes it as its [`Drop`] impl. This is
    /// mainly useful for testing and shouldn't be used in production code.
    ///
    /// ## Safety
    /// This is safe to call in a single-threaded environment but might be unstable and unsafe
    /// to call when in a multi-threaded situation. As advertised, this is mainly for unit testing
    /// and shouldn't be used in other production code, so it is safe to call at any `#[test]` fn.
    ///
    /// ## Example
    /// ```
    /// # use noelware_config::{expand, env};
    /// #
    /// # fn main() {
    /// // `expand` will create a `EnvGuard` that will be dropped
    /// // once this goes out of scope
    /// expand("ENV", || {
    ///     // this will print `Ok("1")`
    ///     println!("{:?}", env!("ENV"));
    /// });
    /// # }
    /// ```
    #[cfg(not(feature = "no-std"))]
    #[cfg_attr(feature = "no-std", doc(hidden))]
    pub struct EnvGuard(String);

    #[cfg(not(feature = "no-std"))]
    impl EnvGuard {
        /// Enters a [`EnvGuard`] with the given value as `1`. It is recommended to use
        /// the [`expand`] function instead to set `env` to `1` and be dropped once a closure
        /// is done invoking.
        ///
        /// ## Example
        /// ```
        /// # use noelware_config::EnvGuard;
        /// #
        /// # fn main() {
        /// let _guard = EnvGuard::enter("ENV");
        /// // `ENV` will be 1 and once it is dropped, it will be no longer available.
        /// # }
        /// ```
        pub fn enter(env: impl Into<String>) -> EnvGuard {
            use ::std::env::set_var;

            let env = env.into();
            set_var(&env, "1");

            EnvGuard(env)
        }

        /// Same as [`EnvGuard::enter`], but will set a different value than `1`.
        ///
        /// [`EnvGuard::enter`]: https://crates.noelware.cloud/~/noelware-config/doc/*/struct.EnvGuard#fn.enter
        pub fn enter_with(env: impl Into<String>, val: impl Into<String>) -> EnvGuard {
            use ::std::env::set_var;

            let env = env.into();
            set_var(&env, val.into());

            EnvGuard(env)
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            use ::std::env::remove_var;

            remove_var(&self.0);
        }
    }

    /// Expand a system environment variable as `env`, set `env` to `1`, run the specified closure, and remove
    /// the environment variable once the closure is done.
    ///
    /// ## Safety
    /// View the [safety docs][EnvGuard#safety] to see why this could be unsafe to be called.
    ///
    /// ## Example
    /// ```
    /// # use noelware_config::{expand, env};
    /// #
    /// # fn main() {
    /// // the `ENV` variable will be dropped once it is printed
    /// expand("ENV", || {
    ///     // `env!` is safe to call here and won't fail
    ///     println!("{:?}", env!("ENV"));
    /// });
    /// # }
    /// ```
    ///
    /// [EnvGuard#safety]: https://crates.noelware.cloud/~/noelware-config/doc/*/struct.EnvGuard#safety
    pub fn expand(env: impl Into<String>, f: impl FnOnce()) {
        let _guard = EnvGuard::enter(env);
        f()
    }

    /// Expand a system environment variable as `env`, set `env` to `val`, run the specified closure, and remove
    /// the environment variable once the closure is done.
    ///
    /// ## Safety
    /// View the [safety docs][EnvGuard#safety] to see why this could be unsafe to be called.
    ///
    /// ## Example
    /// ```
    /// # use noelware_config::{expand_with, env};
    /// #
    /// # fn main() {
    /// // the `ENV` variable will be dropped once it is printed
    /// expand_with("ENV", "2", || {
    ///     // `env!` is safe to call here and won't fail
    ///     println!("{:?}", env!("ENV"));
    /// });
    /// # }
    /// ```
    ///
    /// [EnvGuard#safety]: https://crates.noelware.cloud/~/noelware-config/doc/*/struct.EnvGuard#safety
    pub fn expand_with(env: impl Into<String>, val: impl Into<String>, f: impl FnOnce()) {
        let _guard = EnvGuard::enter_with(env, val);
        f()
    }

    // macro is originally the `env!` macro from charted-server
    // original: https://github.com/charted-dev/charted/blob/94e6c9de95059a9f582c934e32d599031a920c18/crates/config/src/lib.rs#L110-L257
    /// Generic Rust functional macro to help with locating an environment variable in the host machine.
    ///
    /// ## Variants
    /// ### `env!($key: expr)`
    /// This will just expand `$key` into a Result<[`String`][std::string::String], [`VarError`][std::env::VarError]> variant.
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
    /// ### `env!($key: expr, is_optional: true)`
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
    /// ### `env!($key: expr, or_else: $else: expr)`
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
    /// ### `env!($key: expr, or_else_do: $else: expr)`
    /// Same as [`env!($key: expr, or_else: $else: expr)`][crate::var], but uses `.unwrap_or_else` to
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
    /// ### `env!($key: expr, use_default: true)`
    /// Same as [`env!($key: expr, or_else_do: $else: expr)`][crate::var], but will use the
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
    /// ### `env!($key: expr, mapper: $mapper: expr)`
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
    #[macro_export(local_inner_macros)]
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

    #[allow(unused)]
    pub use env;
}

#[cfg(not(feature = "no-std"))]
pub use env::*;
