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

//! Types and functions that interact with the system environment variables.

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
/// # use azalia_config::{expand, env};
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
pub struct EnvGuard(String);

impl EnvGuard {
    /// Enters a [`EnvGuard`] with the given value as `1`. It is recommended to use
    /// the [`expand`] function instead to set `env` to `1` and be dropped once a closure
    /// is done invoking.
    ///
    /// ## Example
    /// ```
    /// # use azalia_config::EnvGuard;
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
/// # use azalia_config::{expand, env};
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
/// # use azalia_config::{expand_with, env};
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
