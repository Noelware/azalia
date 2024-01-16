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
use std::{io::Result, path::Path};

/// Union-like enum for [`StorageService`].
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

    #[deprecated(since = "0.1.0", note = "This should be handled when using pattern matching")]
    #[allow(deprecated, non_camel_case_types)]
    __non_exhaustive,
}

#[async_trait]
#[allow(unused)]
impl remi::StorageService for StorageService {
    const NAME: &'static str = "noelware:remi";

    async fn init(&self) -> Result<()> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.init().await,

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.init().await,

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.init().await,

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.init().await,

            _ => Ok(()),
        }
    }

    async fn open<P: AsRef<Path> + Send>(&self, path: P) -> Result<Option<Bytes>> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.open(path).await,

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.open(path).await,

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.open(path).await,

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.open(path).await,

            _ => Ok(None),
        }
    }

    async fn blob<P: AsRef<Path> + Send>(&self, path: P) -> Result<Option<Blob>> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.blob(path).await,

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.blob(path).await,

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.blob(path).await,

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.blob(path).await,

            _ => Ok(None),
        }
    }

    async fn blobs<P: AsRef<Path> + Send>(
        &self,
        path: Option<P>,
        options: Option<ListBlobsRequest>,
    ) -> Result<Vec<Blob>> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.blobs(path, options).await,

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.blobs(path, options).await,

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.blobs(path, options).await,

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.blobs(path, options).await,

            _ => Ok(vec![]),
        }
    }

    async fn delete<P: AsRef<Path> + Send>(&self, path: P) -> Result<()> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.delete(path).await,

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.delete(path).await,

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.delete(path).await,

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.delete(path).await,

            _ => Ok(()),
        }
    }

    async fn exists<P: AsRef<Path> + Send>(&self, path: P) -> Result<bool> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.exists(path).await,

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.exists(path).await,

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.exists(path).await,

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.exists(path).await,

            _ => Ok(false),
        }
    }

    async fn upload<P: AsRef<Path> + Send>(&self, path: P, options: UploadRequest) -> Result<()> {
        match self {
            #[cfg(feature = "fs")]
            Self::Filesystem(fs) => fs.upload(path, options).await,

            #[cfg(feature = "gridfs")]
            Self::GridFS(gridfs) => gridfs.upload(path, options).await,

            #[cfg(feature = "azure")]
            Self::Azure(azure) => azure.upload(path, options).await,

            #[cfg(feature = "s3")]
            Self::S3(s3) => s3.upload(path, options).await,

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
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum Config {
    #[cfg(feature = "fs")]
    Filesystem(remi_fs::Config),

    #[cfg(feature = "gridfs")]
    GridFS(remi_gridfs::StorageConfig),

    #[cfg(feature = "azure")]
    Azure(remi_azure::StorageConfig),

    #[cfg(feature = "s3")]
    S3(remi_s3::S3StorageConfig),

    #[deprecated(since = "0.1.0", note = "This should be handled when using pattern matching")]
    #[allow(deprecated, non_camel_case_types)]
    __non_exhaustive,
}
