// 🐻‍❄️🪚 Azalia: Noelware's Rust commons library.
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

use std::{
    borrow::Cow,
    collections::HashSet,
    env::{remove_var, set_var},
    marker::PhantomData,
    rc::Rc,
};

/// Guard that sets a environment variable and removes it when being [dropped][Drop]. Useful
/// for testing.
///
/// ## Safety
/// This is safe to call since this struct doesn't allow this guard to be [`Send`]
/// and [`Sync`], making it impossible to be used within multi-threaded contexts
/// but it is easy to use in threads anyway.
///
/// <div class="warning">
///
/// As of Rust edition **2024**, `{set,remove}_var` is considered unsafe and
/// will call either way but this is a fair warning when using in a non-testing
/// environment.
///
/// </div>
pub struct EnvGuard<'a> {
    name: Cow<'a, str>,

    // ensures that `EnvGuard` is `!Send` and `!Sync`
    _p: PhantomData<Rc<i32>>,
}

impl<'a> EnvGuard<'a> {
    /// Enters the guard by setting an environment variable from `name` and
    /// setting its value to `1`.
    ///
    /// ## Safety
    /// Read the **Safety** header in the [`EnvGuard`] type documentation.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::env::EnvGuard;
    /// use std::env;
    ///
    /// // The guard lives on this scope
    /// {
    ///     let guard = EnvGuard::enter("HELLO");
    ///     assert!(env::var("HELLO").is_ok());
    /// }
    ///
    /// // and it'll be removed when dropped from scope
    /// assert!(env::var("HELLO").is_err());
    /// ```
    pub fn enter<K: Into<Cow<'a, str>>>(name: K) -> EnvGuard<'a> {
        let name = name.into();

        // SAFETY: the guard is !Send & !Sync so it wont be possible
        //         to be across thread boundaries but this guard
        //         is mainly for single-threaded programs, so it breaks
        //         then it isn't really our fault, is it?
        unsafe { set_var(&*name, "1") };

        EnvGuard { name, _p: PhantomData }
    }

    /// Enters the guard by setting an environment variable from `name` and
    /// setting its value to the `value` parameter.
    ///
    /// ## Safety
    /// Read the **Safety** header in the [`EnvGuard`] type documentation.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::env::EnvGuard;
    /// use std::env;
    ///
    /// // The guard lives on this scope
    /// {
    ///     let guard = EnvGuard::enter_with("HELLO", "world");
    ///     assert!(matches!(env::var("HELLO"), Ok(String::from("world"))));
    /// }
    ///
    /// // and it'll be removed when dropped from scope
    /// assert!(env::var("HELLO").is_err());
    /// ```
    pub fn enter_with(name: impl Into<Cow<'a, str>>, value: impl Into<String>) -> EnvGuard<'a> {
        let name = name.into();

        // SAFETY: see rationale on EnvGuard#enter
        unsafe { set_var(&*name, value.into()) };

        EnvGuard { name, _p: PhantomData }
    }
}

impl Drop for EnvGuard<'_> {
    fn drop(&mut self) {
        // SAFETY: see rationale on EnvGuard#enter
        unsafe { remove_var(&*self.name) };
    }
}

/// Set a environment variable with the `name` set to the value of `1` and run the closure (`f`).
///
/// This is the same as <code>[`EnvGuard::enter`]\(name\)</code> but runs the closure. When the
/// closure is finished, then the environment variable with the `name` is removed and cannot
/// be accessed anymore.
///
/// This method is useful for testing code that interacts with environment variables. You
/// can use the <code>#\[expand_env\]</code> attribute-based procedural macro if the `derive` feature
/// is enabled and it'll use this function in the expanded code.
///
/// ## Safety
/// Read the **Safety** header in the [`EnvGuard`] type documentation.
///
/// ## Example
/// ```
/// use azalia_config::env::expand;
/// use std::env;
///
/// # fn main() {
/// expand("ENV", || {
///     assert!(env::var("ENV").is_ok());
/// });
///
/// // It is dropped after the closure is called
/// assert!(env::var("ENV").is_err());
/// # }
/// ```
pub fn expand<'a>(name: impl Into<Cow<'a, str>>, f: impl FnOnce() + 'a) {
    let _guard = EnvGuard::enter(name);
    f()
}

/// Set a environment variable with the `name` set to the value to the `value` parameter
/// and runs the closure (`f`).
///
/// This is the same as <code>[`EnvGuard::enter_with`]\(name, value\)</code> but runs the closure. When the
/// closure is finished, then the environment variable with the `name` is removed and cannot be accessed
/// anymore.
///
/// This method is useful for testing code that interacts with environment variables. You
/// can use the <code>#\[expand_env\]</code> attribute-based procedural macro if the `derive` feature
/// is enabled and it'll use this function in the expanded code.
///
/// ## Safety
/// Read the **Safety** header in the [`EnvGuard`] type documentation.
///
/// ## Example
/// ```
/// use azalia_config::env::expand_with;
/// use std::env;
///
/// # fn main() {
/// expand_with("ENV", "var", || {
///     assert!(matches!(env::var("ENV"), Ok(String::from("var"))));
/// });
///
/// // It is dropped after the closure is called
/// assert!(env::var("ENV").is_err());
/// # }
/// ```
pub fn expand_with<'a>(name: impl Into<Cow<'a, str>>, value: impl Into<String>, f: impl FnOnce() + 'a) {
    let _guard = EnvGuard::enter_with(name, value);
    f()
}

/// A guard type that can be used to set multiple environment variables
/// at once and remove them when being [dropped][Drop].
///
/// ## Safety
/// This is safe to call since this struct doesn't allow this guard to be [`Send`]
/// and [`Sync`], making it impossible to be used within multi-threaded contexts
/// but it is easy to use in threads anyway.
///
/// <div class="warning">
///
/// As of Rust edition **2024**, `{set,remove}_var` is considered unsafe and
/// will call either way but this is a fair warning when using in a non-testing
/// environment.
///
/// </div>
pub struct MultipleEnvGuard<'a> {
    names: HashSet<Cow<'a, str>>,
    _p: PhantomData<Rc<i32>>,
}

impl<'a> MultipleEnvGuard<'a> {
    /// Create this guard that sets all the environment variables from the iterator
    /// to a set value and when [dropped][Drop], each variable in the iterator
    /// is removed.
    ///
    /// ## Safety
    /// Read the **Safety** header in the [`MultipleEnvGuard`] type documentation.
    ///
    /// ## Example
    /// ```
    /// use azalia_config::env::MultipleEnvGuard;
    /// use std::env;
    ///
    /// {
    ///     let _guard = MultipleEnvGuard::enter([
    ///         ("HELLO", "world"),
    ///         ("WORLD", "domination")
    ///     ]);
    ///
    ///     assert!(matches!(env::var("HELLO"), Ok(String::from("world"))));
    ///     assert!(matches!(env::var("WORLD"), Ok(String::from("domination"))));
    /// }
    ///
    /// // They will be dropped once the scope runs
    /// assert!(env::var("HELLO").is_err());
    /// assert!(env::var("WORLD").is_err());
    /// ```
    pub fn enter<K: Into<Cow<'a, str>>, V: Into<String>, I: IntoIterator<Item = (K, V)>>(
        variables: I,
    ) -> MultipleEnvGuard<'a> {
        let set = variables
            .into_iter()
            .map(|(name, value)| {
                let name = name.into();

                // SAFETY: see rationale on EnvGuard#enter
                unsafe { set_var(&*name, value.into()) };
                name
            })
            .fold(HashSet::new(), |mut set, name| {
                set.insert(name);
                set
            });

        MultipleEnvGuard {
            names: set,
            _p: PhantomData,
        }
    }
}

impl Drop for MultipleEnvGuard<'_> {
    fn drop(&mut self) {
        for name in &self.names {
            // SAFETY: see rationale on EnvGuard#enter
            unsafe { remove_var(&**name) };
        }
    }
}

/// Sets multiple environment variables with an iterator and runs the closure (`f`).
///
/// This is the same as <code>[`MultipleEnvGuard::enter`]\(...\)</code> but runs a closure
/// instead of keeping track of a drop guard.
///
/// This method is useful for testing code that interacts with environment variables. You
/// can use the <code>#\[expand_env\]</code> attribute-based procedural macro if the `derive` feature
/// is enabled and it'll use this function in the expanded code.
///
/// ## Safety
/// Read the **Safety** header in the [`MultipleEnvGuard`] type documentation.
///
/// ## Example
/// ```
/// use azalia_config::env::expand_multiple;
/// use std::env;
///
/// expand_multiple([
///     ("HELLO", "world"),
///     ("WORLD", "domination")
/// ], || {
///     assert!(matches!(env::var("HELLO"), Ok(String::from("world"))));
///     assert!(matches!(env::var("WORLD"), Ok(String::from("domination"))));
/// });
///
/// // They will be dropped once the closure runs
/// assert!(env::var("HELLO").is_err());
/// assert!(env::var("WORLD").is_err());
/// ```
pub fn expand_multiple<'a>(
    variables: impl IntoIterator<Item = (impl Into<Cow<'a, str>>, impl Into<String>)>,
    f: impl FnOnce() + 'a,
) {
    let _guard = MultipleEnvGuard::enter(variables);
    f()
}
