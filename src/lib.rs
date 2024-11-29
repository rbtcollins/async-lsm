//! Async-lsm library
//!
//! To open a database, use the OpenOptions struct:
//!
//! ```rust,no_run
//! use async_lsm::OpenOptions;
//! use async_lsm::features::filestorage::FileStorage;
//! # async fn run() -> Result<(), ()> {
//! OpenOptions::new(FileStorage::default()).open("path/to/db").await.unwrap();
//! # Ok(())
//! # }
//! ```
//!
//! Use crate features to enable different storage backends:
//! `file` - file storage backend

pub mod features;
pub mod storage;

use color_eyre::eyre::Result;

use storage::Storage;

/// OpenOptions is a builder for opening or creating databases.
#[derive(Debug)]
pub struct OpenOptions {
    _storage: Box<dyn Storage>,
}

impl OpenOptions {
    /// Create a new OpenOptions
    pub fn new(storage: impl Storage + 'static) -> Self {
        OpenOptions {
            _storage: Box::new(storage),
        }
    }

    /// Open a database at the given path
    #[tracing::instrument]
    pub async fn open(&self, _path: &str) -> Result<()> {
        Ok(())
    }
}
