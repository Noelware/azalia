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

//! Traits, types, and utilities when dealing with system environment variables.

#[cfg(all(feature = "macros", feature = "unstable"))]
pub use azalia_config_macros::TryFromEnv;

#[cfg(all(feature = "macros", feature = "unstable"))]
pub use azalia_config_macros::env_test as test;

use std::{
    char::ParseCharError,
    collections::{BTreeMap, BTreeSet, HashSet},
    convert::Infallible,
    env::{remove_var, VarError},
    ffi::OsStr,
    fmt::{Debug, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
    num::{
        NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU128, NonZeroU16, NonZeroU32,
        NonZeroU64, NonZeroU8, NonZeroUsize, ParseFloatError, ParseIntError,
    },
    rc::Rc,
    str::ParseBoolError,
};

/// When reading from the system environment variables, types might want to convert
/// the value from `getenv` to something useful and this is where this trait comes in.
pub trait FromEnvValue: Sized {
    /// Implicit conversion between a environment variable's value to `Self::Output`.
    fn from_env_value(value: String) -> Self;
}

impl FromEnvValue for String {
    fn from_env_value(value: String) -> Self {
        value
    }
}

/// Analognous to [`FromEnvValue`] but it can fail if the given value is not right.
pub trait TryFromEnvValue: Sized {
    /// Error type.
    type Error;

    /// Implicit conversion between a environment variable's value to `Ok(Self::Output)`
    /// if successful.
    fn try_from_env_value(value: String) -> Result<Self, Self::Error>;
}

impl<K: TryFromEnvValue + Eq + Hash, V: TryFromEnvValue> TryFromEnvValue for std::collections::HashMap<K, V> {
    type Error = MapTryFromEnvError<K, V>;

    fn try_from_env_value(value: String) -> Result<Self, Self::Error> {
        let elements = value.split(',');
        let size_hint = elements.size_hint().0;
        let mut map = std::collections::HashMap::with_capacity(size_hint);

        for line in elements {
            if let Some((key, value)) = line.split_once('=') {
                if value.contains('=') {
                    continue;
                }

                let key = K::try_from_env_value(key.to_owned()).map_err(MapTryFromEnvError::Key)?;
                let value = V::try_from_env_value(value.to_owned()).map_err(MapTryFromEnvError::Value)?;

                map.insert(key, value);
            }
        }

        Ok(map)
    }
}

impl<K: TryFromEnvValue + Ord, V: TryFromEnvValue> TryFromEnvValue for BTreeMap<K, V> {
    type Error = MapTryFromEnvError<K, V>;

    fn try_from_env_value(value: String) -> Result<Self, Self::Error> {
        let elements = value.split(',');
        let mut map = BTreeMap::new();

        for line in elements {
            if let Some((key, value)) = line.split_once('=') {
                if value.contains('=') {
                    continue;
                }

                let key = K::try_from_env_value(key.to_owned()).map_err(MapTryFromEnvError::Key)?;
                let value = V::try_from_env_value(value.to_owned()).map_err(MapTryFromEnvError::Value)?;

                map.insert(key, value);
            }
        }

        Ok(map)
    }
}

/// Error variant for <code>impl [`TryFromEnvValue`] for [`std::collections::HashMap`]<K, V></code>
/// and <code>impl [`TryFromEnvValue`] for [`std::collections::BTreeMap`]<K, V></code>.
#[derive(Debug)]
pub enum MapTryFromEnvError<K: TryFromEnvValue, V: TryFromEnvValue> {
    Key(K::Error),
    Value(V::Error),
}

impl<K: TryFromEnvValue, V: TryFromEnvValue> Display for MapTryFromEnvError<K, V>
where
    K::Error: Display,
    V::Error: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Key(s) => Display::fmt(s, f),
            Self::Value(v) => Display::fmt(v, f),
        }
    }
}

impl<K: TryFromEnvValue + Debug + Display, V: TryFromEnvValue + Debug + Display> std::error::Error
    for MapTryFromEnvError<K, V>
where
    K::Error: std::error::Error + 'static,
    V::Error: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Key(k) => Some(k),
            Self::Value(v) => Some(v),
        }
    }
}

impl<T: TryFromEnvValue + Eq + Hash> TryFromEnvValue for HashSet<T> {
    type Error = T::Error;

    fn try_from_env_value(value: String) -> Result<Self, Self::Error> {
        value
            .split(',')
            .map(|v| T::try_from_env_value(v.to_owned()))
            .collect::<Result<_, T::Error>>()
    }
}

impl<T: TryFromEnvValue + Ord> TryFromEnvValue for BTreeSet<T> {
    type Error = T::Error;

    fn try_from_env_value(value: String) -> Result<Self, Self::Error> {
        value
            .split(',')
            .map(|v| T::try_from_env_value(v.to_owned()))
            .collect::<Result<_, T::Error>>()
    }
}

macro_rules! impl_try_from_env {
    ($($(#[$meta:meta])* $Ty:ty: $Error:ty;)*) => {
        $(
            $(#[$meta])*
            impl $crate::env::TryFromEnvValue for $Ty {
                type Error = $Error;

                fn try_from_env_value(value: String) -> Result<Self, Self::Error> {
                    value.parse()
                }
            }
        )*
    };
}

impl_try_from_env!(
    bool: ParseBoolError;
    char: ParseCharError;

    f32: ParseFloatError;
    f64: ParseFloatError;

    NonZeroI8: ParseIntError;
    NonZeroI16: ParseIntError;
    NonZeroI32: ParseIntError;
    NonZeroI64: ParseIntError;
    NonZeroI128: ParseIntError;
    NonZeroIsize: ParseIntError;

    i8: ParseIntError;
    i16: ParseIntError;
    i32: ParseIntError;
    i64: ParseIntError;
    i128: ParseIntError;
    isize: ParseIntError;

    NonZeroU8: ParseIntError;
    NonZeroU16: ParseIntError;
    NonZeroU32: ParseIntError;
    NonZeroU64: ParseIntError;
    NonZeroU128: ParseIntError;
    NonZeroUsize: ParseIntError;

    u8: ParseIntError;
    u16: ParseIntError;
    u32: ParseIntError;
    u64: ParseIntError;
    u128: ParseIntError;
    usize: ParseIntError;

    std::path::PathBuf: Infallible;

    #[cfg(feature = "url")]
    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "url")))]
    url::Url: url::ParseError;
);

impl<T: FromEnvValue> TryFromEnvValue for T {
    type Error = Infallible;

    fn try_from_env_value(value: String) -> Result<Self, Self::Error> {
        Ok(T::from_env_value(value))
    }
}

/// Parses an environment variable from a [`FromEnvValue`] implementation.
pub fn parse<K: Into<String>, V: FromEnvValue>(key: K) -> Result<V, VarError> {
    std::env::var(key.into()).map(V::from_env_value)
}

/// Parses an environment variable from a [`TryFromEnvValue`] implementation.
pub fn try_parse<K: Into<String>, V: TryFromEnvValue>(key: K) -> Result<V, TryParseError<V>> {
    match std::env::var(key.into()) {
        Ok(value) => V::try_from_env_value(value).map_err(TryParseError::Parse),
        Err(e) => Err(TryParseError::System(e)),
    }
}

/// Analogous to [`try_parse`] but uses a closure to compute the default value.
pub fn try_parse_or<K: Into<String>, V: TryFromEnvValue>(
    key: K,
    default: impl FnOnce() -> V,
) -> Result<V, TryParseError<V>> {
    match try_parse(key) {
        Ok(value) => Ok(value),
        Err(TryParseError::System(std::env::VarError::NotPresent)) => Ok(default()),
        Err(e) => Err(e),
    }
}

/// Analogous to [`try_parse`] but uses a default value if the environment variable was not found.
pub fn try_parse_or_else<K: Into<String>, V: TryFromEnvValue>(key: K, default: V) -> Result<V, TryParseError<V>> {
    match std::env::var(key.into()) {
        Ok(value) => V::try_from_env_value(value).map_err(TryParseError::Parse),
        Err(VarError::NotPresent) => Ok(default),
        Err(e) => Err(TryParseError::System(e)),
    }
}

/// Anlogous to [`try_parse`] but returns a <code>[`Option`]\<V\></code> instead.
///
/// When the environment variable by the name of `key` doesn't exist, it'll return `None`.
pub fn try_parse_optional<K: Into<String>, V: TryFromEnvValue>(key: K) -> Result<Option<V>, TryParseError<V>> {
    match std::env::var(key.into()) {
        Ok(value) => V::try_from_env_value(value).map(Some).map_err(TryParseError::Parse),
        Err(VarError::NotPresent) => Ok(None),
        Err(e) => Err(TryParseError::System(e)),
    }
}

/// Error variant for [`try_parse`].
#[derive(Debug)]
pub enum TryParseError<V: TryFromEnvValue> {
    System(VarError),
    Parse(V::Error),
}

impl<V: TryFromEnvValue> Display for TryParseError<V>
where
    V::Error: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TryParseError::System(s) => Display::fmt(s, f),
            TryParseError::Parse(s) => Display::fmt(s, f),
        }
    }
}

impl<V: TryFromEnvValue + Display + Debug> std::error::Error for TryParseError<V>
where
    V::Error: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::System(v) => Some(v),
            Self::Parse(v) => Some(v),
        }
    }
}

/// Represents a trait that allows conversion of a collection of system environment
/// variables for structs or enumerations.
///
/// ## Example
/// ```ignore
/// use azalia_config::env::FromEnv;
///
/// pub struct Config {
///     pub a: String,
/// }
///
/// impl FromEnv for Config {
///     fn from_env() -> Self {
///         Config { a: Default::default() }
///     }
/// }
///
/// let config = Config::from_env();
/// // => Config { a: "" }
/// ```
#[deprecated(
    since = "0.1.0",
    note = "trait is no longer needed as of azalia v0.1.0 (preparation of crates.io ver)"
)]
pub trait FromEnv: Sized {
    /// Implicit conversion to return `Self`.
    fn from_env() -> Self;
}

/// Analogous to [`FromEnv`] but falliable.
///
/// ***This is also a derive macro when the `macros` feature is enabled:
/// <code>#[derive([`TryFromEnv`][derive-redirect])</code>***
///
/// ## Notes
/// The **#[derive([`TryFromEnv`][derive-redirect])]** macro is unstable! Add
/// the `unstable` crate feature to use it.
///
/// For the derive macro, specifying the error type is required:
///
/// ```ignore
/// #[derive(TryFromEnv)]
/// #[env(Box<dyn std::error::Error>)]
/// ```
///
/// Since the procedural macro would have no idea on how to propagate errors
/// based off the context, it is required.
///
/// ## Example
/// ```ignore
/// use azalia_config::env::TryFromEnv;
///
/// #[derive(TryFromEnv)]
/// #[env(Box<dyn std::error::Error>, prefix = "APP")]
/// pub struct Config {
///     #[env("A", default)]
///     pub a: String,
/// }
///
/// let config = Config::try_from_env();
/// assert!(config.is_ok());
/// ```
///
/// [derive-redirect]: derive.TryFromEnv.html
pub trait TryFromEnv: Sized {
    /// Error type
    type Error;

    /// Implicit conversion to return a result of `Self`.
    fn try_from_env() -> Result<Self, Self::Error>;
}

#[allow(deprecated)]
impl<T: FromEnv> TryFromEnv for T {
    type Error = Infallible;

    fn try_from_env() -> Result<Self, Self::Error> {
        Ok(T::from_env())
    }
}

/// A guard type that drops the environment variable once the scope
/// is being dropped.
///
/// This type is [`!Send`](std::marker::Send) and [`!Sync`](std::marker::Sync) as it is unsafe
/// to drop environment variables in different threads.
///
/// ## Safety
///
/// <div class="warning">
///
/// As of Rust edition **2024**, `{set,remove}_var` is considered unsafe and
/// will call either way but this is a fair warning when using in a non-testing
/// environment.
///
/// </div>
///
/// This is only meant in testing environments so it is not our issue to deal
/// with if anything outside of testing goes unsound.
pub struct EnvGuard {
    name: String,
    _non_send_and_sync: PhantomData<Rc<()>>,
}

impl EnvGuard {
    /// Enters the guard and sets the name of the environment variable
    /// to the value of **1**.
    ///
    /// ## Safety
    /// Environment variables are inheritely unsafe to test! See the [`EnvGuard`]'s
    /// Safety documentation about it.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::env::EnvGuard;
    /// use std::env;
    ///
    /// // The guard lives on this scope
    /// {
    ///     let _guard = EnvGuard::enter("HELLO");
    ///     assert!(env::var("HELLO").is_ok());
    /// }
    ///
    /// // and it'll be removed when dropped from scope
    /// assert!(env::var("HELLO").is_err());
    /// ```
    pub fn enter(name: impl Into<String>) -> Self {
        EnvGuard::enter_with(name, "1")
    }

    /// Enters the guard and sets the **name** to a correspondant **value** into
    /// the system environment variables.
    ///
    /// ## Safety
    /// Environment variables are inheritely unsafe to test! See the [`EnvGuard`]'s
    /// Safety documentation about it.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::env::EnvGuard;
    /// use std::env;
    ///
    /// // The guard lives on this scope
    /// {
    ///     let guard = EnvGuard::enter_with("HELLO", "world");
    ///     assert_eq!(env::var("HELLO"), Ok(String::from("world")));
    /// }
    ///
    /// // and it'll be removed when dropped from scope
    /// assert!(env::var("HELLO").is_err());
    /// ```
    pub fn enter_with(name: impl Into<String>, value: impl AsRef<OsStr>) -> Self {
        let name = name.into();

        // Safety: rationale in Safety section of the struct
        // TODO(@auguwu): add `unsafe` block once in 2024 edition
        std::env::set_var(&name, value);

        EnvGuard {
            name,
            _non_send_and_sync: PhantomData,
        }
    }
}

impl PartialEq for EnvGuard {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for EnvGuard {}

impl Hash for EnvGuard {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        // TODO(@auguwu): add `unsafe` block once in 2024 edition
        remove_var(&self.name);
    }
}

/// A guard analogous to [`EnvGuard`] but holds a set of guards to be dropped
/// once the scope is finished.
///
/// This type is [`!Send`](std::marker::Send) and [`!Sync`](std::marker::Sync) as it is unsafe
/// to drop environment variables in different threads.
///
/// ## Safety
///
/// <div class="warning">
///
/// As of Rust edition **2024**, `{set,remove}_var` is considered unsafe and
/// will call either way but this is a fair warning when using in a non-testing
/// environment.
///
/// </div>
///
/// This is only meant in testing environments so it is not our issue to deal
/// with if anything outside of testing goes unsound.
pub struct MultipleEnvGuard {
    _variables: HashSet<EnvGuard>,
    _non_send_sync: PhantomData<Rc<()>>,
}

impl MultipleEnvGuard {
    /// Enters the guard and sets a iterator of `(key, value)` as [`EnvGuard`]s. On [`Drop`], it'll
    /// call [`remove_var`] of the specified environment variables.
    ///
    /// ## Safety
    /// Environment variables are inheritely unsafe to test! See the [`MultipleEnvGuard`]'s
    /// Safety documentation about it.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::env::MultipleEnvGuard;
    /// use std::env::var;
    ///
    /// {
    ///     let _guard = MultipleEnvGuard::enter([
    ///         ("HELLO", "world"),
    ///         ("NOEL_IS_CUTE", "true")
    ///     ]);
    ///
    ///     assert_eq!(var("HELLO"), Ok(String::from("world")));
    ///     assert_ne!(var("NOEL_IS_CUTE"), Ok(String::from("false")));
    /// }
    ///
    /// assert!(var("HELLO").is_err());
    /// assert!(var("NOEL_IS_CUTE").is_err());
    /// ```
    pub fn enter(values: impl IntoIterator<Item = (impl Into<String>, impl AsRef<OsStr>)>) -> Self {
        MultipleEnvGuard {
            _non_send_sync: PhantomData,
            _variables: values
                .into_iter()
                .map(|(key, value)| EnvGuard::enter_with(key, value))
                .collect(),
        }
    }
}

/// Enters the [`EnvGuard`] by setting **key** to **1** and calls `f`.
///
/// ## Safety
/// Environment variables are inheritely unsafe to test! See the [`EnvGuard`]'s
/// Safety documentation about it.
pub fn enter(key: impl Into<String>, f: impl FnOnce()) {
    let _guard = EnvGuard::enter(key);
    f()
}

/// Enters the [`EnvGuard`] by setting **key** to the **value** and calls `f`.
///
/// ## Safety
/// Environment variables are inheritely unsafe to test! See the [`EnvGuard`]'s
/// Safety documentation about it.
pub fn enter_with(key: impl Into<String>, value: impl AsRef<OsStr>, f: impl FnOnce()) {
    let _guard = EnvGuard::enter_with(key, value);
    f()
}

/// Enters the [`EnvGuard`] by setting multiple environment variables via an iterator
/// implementation and calls **f**.
///
/// ## Safety
/// Environment variables are inheritely unsafe to test! See the [`MultipleEnvGuard`]'s
/// Safety documentation about it.
pub fn enter_multiple(iter: impl IntoIterator<Item = (impl Into<String>, impl AsRef<OsStr>)>, f: impl FnOnce()) {
    let _guard = MultipleEnvGuard::enter(iter);
    f()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

    // this is a hack since we export `test` if `--features unstable` is enabled
    #[cfg_attr(not(feature = "unstable"), test)]
    #[cfg_attr(feature = "unstable", core::prelude::v1::test)]
    fn drop_multiple_env_guard() {
        {
            let mut guards = HashSet::new();
            guards.insert(EnvGuard::enter("HELLO"));

            assert!(std::env::var("HELLO").is_ok());
        }

        assert!(std::env::var("HELLO").is_err());

        {
            let _guard = MultipleEnvGuard::enter([("HELLO", "world")]);
            assert!(std::env::var("HELLO").is_ok());
        }

        assert!(std::env::var("HELLO").is_err());
    }

    #[cfg_attr(not(feature = "unstable"), test)]
    #[cfg_attr(feature = "unstable", core::prelude::v1::test)]
    fn map_try_from_env_value() {
        assert!(<HashMap<String, String> as TryFromEnvValue>::try_from_env_value("hello=world".into()).is_ok());
        assert!(<HashMap<String, String> as TryFromEnvValue>::try_from_env_value("helloworld".into()).is_ok());
        assert!(<HashMap<String, String> as TryFromEnvValue>::try_from_env_value("".into()).is_ok());
        assert!(<HashMap<String, String> as TryFromEnvValue>::try_from_env_value(
            "hello=world,weow=fluff;wwww,s=true".into()
        )
        .is_ok());

        assert!(<BTreeMap<String, String> as TryFromEnvValue>::try_from_env_value("hello=world".into()).is_ok());
        assert!(<BTreeMap<String, String> as TryFromEnvValue>::try_from_env_value("helloworld".into()).is_ok());
        assert!(<BTreeMap<String, String> as TryFromEnvValue>::try_from_env_value("".into()).is_ok());
        assert!(<BTreeMap<String, String> as TryFromEnvValue>::try_from_env_value(
            "hello=world,weow=fluff;wwww,s=true".into()
        )
        .is_ok());
    }

    #[cfg_attr(not(feature = "unstable"), test)]
    #[cfg_attr(feature = "unstable", core::prelude::v1::test)]
    fn set_try_from_env_value() {
        assert!(<HashSet<String> as TryFromEnvValue>::try_from_env_value("hello,world".into()).is_ok());
        assert!(<HashSet<String> as TryFromEnvValue>::try_from_env_value("helloworld".into()).is_ok());
        assert!(<HashSet<String> as TryFromEnvValue>::try_from_env_value("".into()).is_ok());
        assert!(<HashSet<String> as TryFromEnvValue>::try_from_env_value("hello,world,weow,fluff".into()).is_ok());

        assert!(<BTreeSet<String> as TryFromEnvValue>::try_from_env_value("hello,world".into()).is_ok());
        assert!(<BTreeSet<String> as TryFromEnvValue>::try_from_env_value("helloworld".into()).is_ok());
        assert!(<BTreeSet<String> as TryFromEnvValue>::try_from_env_value("".into()).is_ok());
        assert!(<BTreeSet<String> as TryFromEnvValue>::try_from_env_value("hello,world,weow,fluff".into()).is_ok());
    }
}
