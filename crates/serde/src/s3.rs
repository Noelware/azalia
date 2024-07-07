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

//! Defines extra [`Serializer`](serde::ser::Serializer) and [`Deserializer`](serde::de::Deserializer) functions for
//! AWS S3 types that are most useful to use `serde` with.

macro_rules! impl_methods {
    ($mod:ident: $name:ident) => {
        #[doc = concat!(" `serde` implementation for [`", stringify!($name), "`](aws_sdk_s3::types::", stringify!($name), ").")]
        pub mod $mod {
            #[doc = concat!(" Provides a [`Serializer`](serde::ser::Serializer) implementation to [`", stringify!($name), "`](aws_sdk_s3::types::", stringify!($name), "). This is mainly")]
            /// to be used with the `with` serde attribute when using serde's derive macros. But, it can
            /// be used with your own serializer.
            ///
            /// ## Example
            /// ```no_run
            #[doc = concat!(" # use aws_sdk_s3::types::", stringify!($name), ";")]
            /// # use serde::Serialize;
            /// #
            /// #[derive(Serialize)]
            /// pub struct MyStruct {
            #[doc = concat!("   #[serde(serialize_with = \"azalia_serde::s3::", stringify!($mod), "::serialize\")]")]
            #[doc = concat!("   acl: ", stringify!($name), ",")]
            /// }
            /// ```
            pub fn serialize<S: ::serde::ser::Serializer>(value: &::aws_sdk_s3::types::$name, serializer: S) -> Result<S::Ok, S::Error> {
                // TODO(@auguwu): I'm not sure if this is the right choice, but it does discuss that new variants
                // shouldn't be errors, rather than be encouraged to be serialized for new SDK revisions, but
                // should we allow unknown variants? For the moment, yes, but I'm not sure in the long-term.
                serializer.serialize_str(value.as_str())
            }

            #[doc = concat!(" Provides a [`Deserializer`](serde::de::Deserializer) implementation to [`", stringify!($name), "`](aws_sdk_s3::types::", stringify!($name), "). This is mainly")]
            /// to be used with the `with` serde attribute when using serde's derive macros. But, it can
            /// be used with your own serializer.
            ///
            /// ## Example
            /// ```no_run
            #[doc = concat!(" # use aws_sdk_s3::types::", stringify!($name), ";")]
            /// # use serde::Deserialize;
            /// #
            /// #[derive(Deserialize)]
            /// pub struct MyStruct {
            #[doc = concat!("   #[serde(deserialize_with = \"azalia_serde::s3::", stringify!($mod), "::deserialize\")]")]
            #[doc = concat!("   acl: ", stringify!($name), ",")]
            /// }
            /// ```
            pub fn deserialize<'de, D: ::serde::de::Deserializer<'de>>(deserializer: D) -> Result<::aws_sdk_s3::types::$name, D::Error> {
                use ::serde::Deserialize;

                let value = String::deserialize(deserializer)?;
                Ok(::aws_sdk_s3::types::$name::from(value.as_str()))
            }
        }
    };
}

impl_methods!(objectcannedacl: ObjectCannedAcl);
impl_methods!(bucketcannedacl: BucketCannedAcl);
