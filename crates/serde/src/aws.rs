// 🐻‍❄️🪚 Azalia: Family of crates that implement common Rust code
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

//! Defines extra [`Serializer`](serde::ser::Serializer) and [`Deserializer`](serde::de::Deserializer) functions for
//! AWS-specific types that are most useful to use `serde` with.

/// `serde` implementation for [`Region`](aws_types::region::Region)
pub mod region {
    use aws_types::region::Region;
    use serde::{Deserializer, Serializer};

    /// Provides a [`Serializer`] implementation to [`Region`]. This is mainly to be used
    /// with the `#[serde(with)]` attribute when using serde's derive macros. It can be used for
    /// your own serializers as well.
    ///
    /// ## Example
    /// ```no_run
    /// # use aws_types::region::Region;
    /// # use serde::Serialize;
    /// #
    /// #[derive(Serialize)]
    /// pub struct MyStruct {
    ///     #[serde(serialize_with = "azalia_serde::aws::region::serialize")]
    ///     region: Region,
    /// }
    /// ```
    pub fn serialize<S: Serializer>(value: &Region, serializer: S) -> Result<S::Ok, S::Error> {
        // TODO(@auguwu): Same in src/s3.rs, do we allow this since the `Region` type is just a
        // container of Cow<'static, str> (copy-on-write). For now, I guess it'll be the bare
        // minimum, but do we collect all the valid AWS regions? (that would require work)
        serializer.serialize_str(value.to_string().as_str())
    }

    /// Provides a [`Deserializer`] implementation to [`Region`]. This is mainly to be used
    /// with the `#[serde(with)]` attribute when using serde's derive macros. It can be used for
    /// your own serializers as well.
    ///
    /// ## Example
    /// ```no_run
    /// # use aws_types::region::Region;
    /// # use serde::Deserialize;
    /// #
    /// #[derive(Deserialize)]
    /// pub struct MyStruct {
    ///     #[serde(deserialize_with = "azalia_serde::aws::region::deserialize")]
    ///     region: Region,
    /// }
    /// ```
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Region, D::Error> {
        use serde::Deserialize;

        let value = String::deserialize(deserializer)?;
        Ok(Region::new(value))
    }
}
