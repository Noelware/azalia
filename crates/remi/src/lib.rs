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

//! <div align="center">
//!     <h3>🐻‍❄️🪚 <code>azalia-remi</code></h3>
//!     <h4>Unified storage services for <a href="https://docs.rs/remi/*/remi/trait.StorageService.html">remi::StorageService</a> of official crates</h4>
//!     <hr />
//! </div>
//!
//! ## 🐻‍❄️🪚 `azalia-remi`
//!
//! **azalia-remi** adds a unified storage service on top of **remi-rs** for allow configuring multiple storage
//! services but only uses one from what the end user wants.
//!
//! This uses Cargo's crate features to implicitilly allow you to pick out which Remi-based crates to implement
//! into your applications. You can use the `features = ["all"]` in your Cargo.toml's definition of `azalia-remi`
//! to include all crates.
//!
//! ## Example
//! ```no_run
//! // Cargo.toml:
//! //
//! // [dependencies]
//! // tokio = { version = "*", features = ["full"] }
//! // azalia-remi = { version = "^0", features = ["fs"] }
//!
//! use azalia_remi::{
//!     StorageService,
//!     Config,
//!
//!     core::StorageService as _,
//!     fs
//! };
//!
//! # #[tokio::main]
//! # async fn main() {
//! let config = fs::StorageConfig {
//!     directory: "/data".into(),
//! };
//!
//! let service = StorageService::Filesystem(fs::StorageService::with_config(config));
//! service.init().await.unwrap(); // initialize the fs version of remi
//!
//! // do whatever you want
//! # }
//! ```

#![doc(html_logo_url = "https://cdn.floofy.dev/images/trans.png")]
#![cfg_attr(any(noeldoc, docsrs), feature(doc_cfg))]
#![allow(non_camel_case_types)]

use remi::{ListBlobsRequest, UploadRequest};
use std::path::Path;

pub use remi as core;

#[cfg(feature = "gridfs")]
#[cfg_attr(any(docsrs, noeldoc), doc(cfg(feature = "gridfs")))]
pub use remi_gridfs as gridfs;

#[cfg(feature = "azure")]
#[cfg_attr(any(docsrs, noeldoc), doc(cfg(feature = "azure")))]
pub use remi_azure as azure;

#[cfg(feature = "s3")]
#[cfg_attr(any(docsrs, noeldoc), doc(cfg(feature = "s3")))]
pub use remi_s3 as s3;

#[cfg(feature = "fs")]
#[cfg_attr(any(docsrs, noeldoc), doc(cfg(feature = "fs")))]
pub use remi_fs as fs;

macro_rules! mk_storage_service_impl {
    (
        $(#[$meta:meta])*
        $($feat:literal => $field:ident as $ty:ty {
            $(#[$error_meta:meta])*
            Error: $error:ty;

            $(#[$config_meta:meta])*
            Config: $config:ty;

            Display: |$f:ident, $error_name:ident| $display:expr;
        })*
    ) => {
        $(#[$meta])*
        pub enum StorageService {
            $(
                #[cfg(feature = $feat)]
                #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = $feat)))]
                $field($ty),
            )*

            __non_exhaustive
        }

        /// Error variant when using methods from [`StorageService`].
        #[derive(Debug)]
        #[allow(non_camel_case_types)]
        pub enum Error {
            $(
                #[cfg(feature = $feat)]
                #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = $feat)))]
                $(#[$error_meta])*
                $field($error),
            )*

            __non_exhaustive,
        }

        impl ::std::fmt::Display for Error {
            #[allow(unused)]
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match self {
                    $(
                        #[cfg(feature = $feat)]
                        Error::$field(err) => {
                            let $error_name = err;
                            let $f = f;

                            $display
                        },
                    )*

                    _ => unreachable!()
                }
            }
        }

        impl ::std::error::Error for Error {
            fn source(&self) -> Option<&(dyn ::std::error::Error + 'static)> {
                match self {
                    $(
                        #[cfg(feature = $feat)]
                        Error::$field(err) => Some(err),
                    )*

                    _ => None
                }
            }
        }

        $(
            #[cfg(feature = $feat)]
            #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = $feat)))]
            impl ::core::convert::From<$error> for Error {
                fn from(value: $error) -> Self {
                    Error::$field(value)
                }
            }
        )*

        #[derive(Debug, Clone, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
        #[non_exhaustive]
        pub enum Config {
            $(
                #[cfg(feature = $feat)]
                #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = $feat)))]
                #[doc = concat!("Configuration variant that can be used to configure the [`StorageService::", stringify!($field), "`] variant.")]
                $(#[$config_meta])*
                $field($config),
            )*

            #[default]
            Unknown,
        }
    };
}

mk_storage_service_impl! {
    /// Represents a unified [`StorageService`][remi::StorageService] that can be
    /// either the following:
    ///
    #[cfg_attr(feature = "gridfs", doc = "* [`remi_gridfs::StorageService`]")]
    #[cfg_attr(feature = "azure", doc = "* [`remi_azure::StorageService`]")]
    #[cfg_attr(feature = "fs", doc = "* [`remi_fs::StorageService`]")]
    #[cfg_attr(feature = "s3", doc = "* [`remi_s3::StorageService`]")]
    #[allow(non_camel_case_types)]
    #[derive(Clone)]

    "gridfs" => Gridfs as ::remi_gridfs::StorageService {
        /// Error variant that can happen when using [`remi_gridfs::StorageService`].
        Error: ::remi_gridfs::mongodb::error::Error;
        Config: ::remi_gridfs::StorageConfig;
        Display: |f, err| match &*err.kind {
            ::remi_gridfs::mongodb::error::ErrorKind::Custom(msg) => {
                if let Some(msg) = msg.downcast_ref::<&str>() {
                    f.write_str(msg)
                } else if let Some(msg) = msg.downcast_ref::<String>() {
                    f.write_str(msg)
                } else {
                    ::std::fmt::Display::fmt(err, f)
                }
            },

            _ => ::std::fmt::Display::fmt(err, f),
        };
    }

    "azure" => Azure as ::remi_azure::StorageService {
        /// Error variant that can happen when using [`remi_azure::StorageService`].
        Error: ::remi_azure::core::error::Error;
        Config: ::remi_azure::StorageConfig;
        Display: |f, err| ::std::fmt::Display::fmt(err, f);
    }

    "fs" => Filesystem as ::remi_fs::StorageService {
        /// Error variant that can happen when using [`remi_fs::StorageService`].
        Error: ::std::io::Error;
        Config: ::remi_fs::StorageConfig;
        Display: |f, err| ::std::fmt::Display::fmt(err, f);
    }

    "s3" => S3 as ::remi_s3::StorageService {
        /// Error variant that can happen when using [`remi_s3::StorageService`].
        Error: ::remi_s3::Error;
        Config: ::remi_s3::StorageConfig;
        Display: |f, err| ::std::fmt::Display::fmt(err, f);
    }
}

#[remi::async_trait]
#[allow(unused)]
impl remi::StorageService for StorageService {
    type Error = Error;

    fn name(&self) -> ::std::borrow::Cow<'static, str> {
        ::std::borrow::Cow::Borrowed("azalia:remi")
    }

    async fn init(&self) -> Result<(), Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            StorageService::Filesystem(service) => service.init().await.map_err(From::from),

            #[cfg(feature = "s3")]
            StorageService::S3(service) => service.init().await.map_err(From::from),

            #[cfg(feature = "azure")]
            StorageService::Azure(service) => service.init().await.map_err(From::from),

            #[cfg(feature = "gridfs")]
            StorageService::Gridfs(service) => service.init().await.map_err(From::from),

            _ => unreachable!(),
        }
    }

    async fn open<P: AsRef<Path> + Send>(&self, path: P) -> Result<Option<remi::Bytes>, Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            StorageService::Filesystem(service) => service.open(path).await.map_err(From::from),

            #[cfg(feature = "s3")]
            StorageService::S3(service) => service.open(path).await.map_err(From::from),

            #[cfg(feature = "azure")]
            StorageService::Azure(service) => service.open(path).await.map_err(From::from),

            #[cfg(feature = "gridfs")]
            StorageService::Gridfs(service) => service.open(path).await.map_err(From::from),

            _ => unreachable!(),
        }
    }

    async fn blob<P: AsRef<Path> + Send>(&self, path: P) -> Result<Option<remi::Blob>, Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            StorageService::Filesystem(service) => service.blob(path).await.map_err(From::from),

            #[cfg(feature = "s3")]
            StorageService::S3(service) => service.blob(path).await.map_err(From::from),

            #[cfg(feature = "azure")]
            StorageService::Azure(service) => service.blob(path).await.map_err(From::from),

            #[cfg(feature = "gridfs")]
            StorageService::Gridfs(service) => service.blob(path).await.map_err(From::from),

            _ => unreachable!(),
        }
    }

    async fn blobs<P: AsRef<Path> + Send>(
        &self,
        path: Option<P>,
        options: Option<ListBlobsRequest>,
    ) -> Result<Vec<remi::Blob>, Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            StorageService::Filesystem(service) => service.blobs(path, options).await.map_err(From::from),

            #[cfg(feature = "s3")]
            StorageService::S3(service) => service.blobs(path, options).await.map_err(From::from),

            #[cfg(feature = "azure")]
            StorageService::Azure(service) => service.blobs(path, options).await.map_err(From::from),

            #[cfg(feature = "gridfs")]
            StorageService::Gridfs(service) => service.blobs(path, options).await.map_err(From::from),

            _ => unreachable!(),
        }
    }

    async fn delete<P: AsRef<Path> + Send>(&self, path: P) -> Result<(), Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            StorageService::Filesystem(service) => service.delete(path).await.map_err(From::from),

            #[cfg(feature = "s3")]
            StorageService::S3(service) => service.delete(path).await.map_err(From::from),

            #[cfg(feature = "azure")]
            StorageService::Azure(service) => service.delete(path).await.map_err(From::from),

            #[cfg(feature = "gridfs")]
            StorageService::Gridfs(service) => service.delete(path).await.map_err(From::from),

            _ => unreachable!(),
        }
    }

    async fn exists<P: AsRef<Path> + Send>(&self, path: P) -> Result<bool, Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            StorageService::Filesystem(service) => service.exists(path).await.map_err(From::from),

            #[cfg(feature = "s3")]
            StorageService::S3(service) => service.exists(path).await.map_err(From::from),

            #[cfg(feature = "azure")]
            StorageService::Azure(service) => service.exists(path).await.map_err(From::from),

            #[cfg(feature = "gridfs")]
            StorageService::Gridfs(service) => service.exists(path).await.map_err(From::from),

            _ => unreachable!(),
        }
    }

    async fn upload<P: AsRef<Path> + Send>(&self, path: P, request: UploadRequest) -> Result<(), Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            StorageService::Filesystem(service) => service.upload(path, request).await.map_err(From::from),

            #[cfg(feature = "s3")]
            StorageService::S3(service) => service.upload(path, request).await.map_err(From::from),

            #[cfg(feature = "azure")]
            StorageService::Azure(service) => service.upload(path, request).await.map_err(From::from),

            #[cfg(feature = "gridfs")]
            StorageService::Gridfs(service) => service.upload(path, request).await.map_err(From::from),

            _ => unreachable!(),
        }
    }

    #[cfg(feature = "unstable")]
    #[cfg_attr(any(noeldoc, docsrs), doc(cfg(feature = "unstable")))]
    async fn healthcheck(&self) -> Result<(), Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            StorageService::Filesystem(service) => service.healthcheck().await.map_err(From::from),

            #[cfg(feature = "s3")]
            StorageService::S3(service) => service.healthcheck().await.map_err(From::from),

            #[cfg(feature = "azure")]
            StorageService::Azure(service) => service.healthcheck().await.map_err(From::from),

            #[cfg(feature = "gridfs")]
            StorageService::Gridfs(service) => service.healthcheck().await.map_err(From::from),

            _ => unreachable!(),
        }
    }
}
