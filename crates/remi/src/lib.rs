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
#![allow(deprecated)] // all #[deprecated] are all the non exhaustive types

pub use remi;

#[cfg(feature = "gridfs")]
#[cfg_attr(docsrs, doc(cfg(feature = "gridfs")))]
pub use remi_gridfs as gridfs;

#[cfg(feature = "azure")]
#[cfg_attr(docsrs, doc(cfg(feature = "azure")))]
pub use remi_azure as azure;

#[cfg(feature = "s3")]
#[cfg_attr(docsrs, doc(cfg(feature = "s3")))]
pub use remi_s3 as s3;

#[cfg(feature = "fs")]
#[cfg_attr(docsrs, doc(cfg(feature = "fs")))]
pub use remi_fs as fs;

#[allow(unused)]
use remi::{async_trait, Blob, Bytes, ListBlobsRequest, StorageService as _, UploadRequest};
use std::{error, fmt::Display, path::Path};

/// Union-like enum for [`StorageService`]. As more official crates are supported, this will always
/// be non-exhausive.
#[derive(Clone)]
#[allow(deprecated)]
#[non_exhaustive]
pub enum StorageService {
    #[cfg(feature = "fs")]
    Filesystem(remi_fs::StorageService),

    #[cfg(feature = "gridfs")]
    GridFS(remi_gridfs::StorageService),

    #[cfg(feature = "azure")]
    Azure(remi_azure::StorageService),

    #[cfg(feature = "s3")]
    S3(remi_s3::S3StorageService),

    #[allow(deprecated, non_camel_case_types)]
    __non_exhaustive,
}

/// Represents an error that occurred. As more official crates are supported, this will always
/// be non-exhausive.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Represents the error type for the local filesystem's [`StorageService`][remi_fs::StorageService] implementation.
    #[cfg(feature = "fs")]
    Filesystem(std::io::Error),

    /// Represents the error type for MongoDB Gridfs' [`StorageService`][remi_fs::StorageService] implementation.
    #[cfg(feature = "gridfs")]
    GridFS(mongodb::error::Error),

    /// Represents the error type for Microsoft's Azure Blob Storage [`StorageService`][remi_fs::StorageService] implementation.
    #[cfg(feature = "gridfs")]
    Azure(azure_core::Error),

    /// Represents the error type for Amazon S3's [`StorageService`][remi_fs::StorageService] implementation.
    #[cfg(feature = "s3")]
    S3(std::io::Error),

    #[allow(deprecated, non_camel_case_types)]
    __non_exhaustive,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "filesystem")]
            Error::Filesystem(err) => Display::fmt(err, f),

            #[cfg(feature = "gridfs")]
            Error::GridFS(err) => Display::fmt(err, f),

            #[cfg(feature = "azure")]
            Error::Azure(err) => Display::fmt(err, f),

            #[cfg(feature = "s3")]
            Error::S3(err) => Display::fmt(err, f),

            _ => f.write_str("<unknown error>"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            #[cfg(feature = "filesystem")]
            Error::Filesystem(err) => Some(err),

            #[cfg(feature = "gridfs")]
            Error::GridFS(err) => Some(err),

            #[cfg(feature = "azure")]
            Error::Azure(err) => Some(err),

            #[cfg(feature = "s3")]
            Error::S3(err) => Some(err),

            _ => None,
        }
    }
}

#[cfg(feature = "gridfs")]
impl From<mongodb::error::Error> for Error {
    fn from(err: mongodb::error::Error) -> Error {
        Error::GridFS(err)
    }
}

#[cfg(feature = "azure")]
impl From<azure_core::Error> for Error {
    fn from(err: azure_core::Error) -> Error {
        Error::Azure(err)
    }
}

#[async_trait]
#[allow(unused)]
impl remi::StorageService for StorageService {
    type Error = Error;
    const NAME: &'static str = "noelware:remi";

    async fn init(&self) -> Result<(), Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.init().await.map_err(Error::Filesystem),

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.init().await.map_err(From::from),

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.init().await.map_err(From::from),

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.init().await.map_err(Error::S3),

            _ => Ok(()),
        }
    }

    async fn open<P: AsRef<Path> + Send>(&self, path: P) -> Result<Option<Bytes>, Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.open(path).await.map_err(Error::Filesystem),

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.open(path).await.map_err(From::from),

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.open(path).await.map_err(From::from),

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.open(path).await.map_err(Error::S3),

            _ => Ok(None),
        }
    }

    async fn blob<P: AsRef<Path> + Send>(&self, path: P) -> Result<Option<Blob>, Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.blob(path).await.map_err(Error::Filesystem),

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.blob(path).await.map_err(From::from),

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.blob(path).await.map_err(From::from),

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.blob(path).await.map_err(Error::S3),

            _ => Ok(None),
        }
    }

    async fn blobs<P: AsRef<Path> + Send>(
        &self,
        path: Option<P>,
        options: Option<ListBlobsRequest>,
    ) -> Result<Vec<Blob>, Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.blobs(path, options).await.map_err(Error::Filesystem),

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.blobs(path, options).await.map_err(From::from),

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.blobs(path, options).await.map_err(From::from),

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.blobs(path, options).await.map_err(Error::S3),

            _ => Ok(vec![]),
        }
    }

    async fn delete<P: AsRef<Path> + Send>(&self, path: P) -> Result<(), Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.delete(path).await.map_err(Error::Filesystem),

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.delete(path).await.map_err(From::from),

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.delete(path).await.map_err(From::from),

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.delete(path).await.map_err(Error::S3),

            _ => Ok(()),
        }
    }

    async fn exists<P: AsRef<Path> + Send>(&self, path: P) -> Result<bool, Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.exists(path).await.map_err(Error::Filesystem),

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.exists(path).await.map_err(From::from),

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.exists(path).await.map_err(From::from),

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.exists(path).await.map_err(Error::S3),

            _ => Ok(false),
        }
    }

    async fn upload<P: AsRef<Path> + Send>(&self, path: P, options: UploadRequest) -> Result<(), Self::Error> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.upload(path, options).await.map_err(Error::Filesystem),

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.upload(path, options).await.map_err(From::from),

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.upload(path, options).await.map_err(From::from),

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.upload(path, options).await.map_err(Error::S3),

            _ => Ok(()),
        }
    }
}

/// Union-like enum for all the possible configuration structures for each
/// Remi-based crate.
#[derive(Debug, Clone)]
#[allow(deprecated)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum Config {
    #[cfg(feature = "fs")]
    Filesystem(remi_fs::Config),

    #[cfg(feature = "gridfs")]
    GridFS(remi_gridfs::StorageConfig),

    #[cfg(feature = "azure")]
    Azure(remi_azure::StorageConfig),

    #[cfg(feature = "s3")]
    S3(remi_s3::S3StorageConfig),

    #[allow(deprecated, non_camel_case_types)]
    __non_exhaustive,
}
