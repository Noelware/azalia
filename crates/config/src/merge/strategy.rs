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

//! Common merge strategies for primitives. This is made since the default implementations
//! might not what you want, so this is some common ones that can be overwritten with the
//! `Merge` proc-macro, or written by hand without it.

/// Common merge strategies for strings. The default strategy will compare the strings
/// and checks if `lhs` != `rhs`. This comes with the `append` and `overwrite` strategies:
///
/// * `overwrite_empty` will overwrite `right` into `left` if `left` was empty.
/// * `overwrite` will overwrite `right` into `left` regardless
/// * `append` will append `right` into `left`.
///
/// For string slices (`&str`), it is impossible to do since string slices are immutable
/// while [`String`] is mutable, so we don't plan to add `&str` support without doing
/// unsafe code.
pub mod strings {
    /// Grows and appends the `right` into the `left`.
    ///
    /// ## Example
    /// ```no_run
    /// # use noelware_config::merge::strategy::strings::append;
    /// #
    /// let mut a = String::from("hello");
    /// let b = String::from(", world!");
    ///
    /// append(&mut a, b);
    /// assert_eq!(a.as_str(), "hello, world!");
    /// ```
    pub fn append(left: &mut String, right: String) {
        left.push_str(&right);
    }

    /// Overwrites the left hand-side into the right-hand side regardless of anything.
    ///
    /// ## Example
    /// ```no_run
    /// # use noelware_config::merge::strategy::strings::overwrite;
    /// #
    /// let mut a = String::from("hi!");
    /// let b = String::from("overwritten...");
    ///
    /// overwrite(&mut a, b);
    /// assert_eq!(a.as_str(), "overwritten...");
    /// ```
    pub fn overwrite(left: &mut String, right: String) {
        *left = right;
    }

    /// Overwrites the left hand-side into the right-hand side if lhs was empty.
    ///
    /// ## Example
    /// ```no_run
    /// # use noelware_config::merge::strategy::strings::overwrite_empty;
    /// #
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

/// Common merging strategies for the `Vec` type.
pub mod vec {
    /// Moves all the elements from `right` into `left`, this doesn't
    /// sort the elements or checks for uniqueness.
    pub fn append<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        left.append(&mut right);
    }

    /// Extends all the elements from `right` into `left`. This doesn't move
    /// all elements from `right` into `left`, leaving `right` empty like
    /// [`append`][crate::strategy::vec::append] does.
    pub fn extend<T>(left: &mut Vec<T>, right: Vec<T>) {
        left.extend(right);
    }

    /// Overwrites the elements in `left` from `right` if `left` is empty.
    pub fn overwrite_empty<T>(left: &mut Vec<T>, mut right: Vec<T>) {
        if left.is_empty() {
            left.append(&mut right);
        }
    }
}

/// Common merging strategies for the [`bool`] type.
pub mod bool {
    /// Only merge `left` into `right` if `left` is `false`.
    pub fn only_if_falsy(left: &mut bool, right: bool) {
        if !*left {
            *left = right;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::strings;

    // ~ strings ~
    #[test]
    fn strings_append() {
        let mut a = String::from("foo");
        strings::append(&mut a, String::from("bar"));

        assert_eq!("foobar", a);
    }

    #[test]
    fn strings_overwrite() {
        let mut a = String::from("woof");
        strings::overwrite(&mut a, String::from("wag"));

        assert_eq!("wag", a);
    }

    #[test]
    fn strings_overwrite_empty() {
        let mut a = String::new();
        strings::overwrite_empty(&mut a, String::from("weow"));

        assert_eq!("weow", a);
        strings::overwrite_empty(&mut a, String::from("heck"));

        assert_eq!("weow", a);
    }
}