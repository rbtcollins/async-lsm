//! Async-lsm library
//!
//! To open a database, use the OpenOptions struct:
//!
//! ```rust,no_run
//! use async_lsm::OpenOptions;
//! # async fn run() -> Result<(), ()> {
//! OpenOptions::new().open("path/to/db").await.unwrap();
//! # Ok(())
//! # }
//! ```
//!
//! Use crate features to enable different storage backends:
//! `file` - file storage backend

use color_eyre::eyre::Result;

/// OpenOptions is a builder for opening or creating databases.
#[derive(Debug, Default)]
pub struct OpenOptions;

impl OpenOptions {
    /// Create a new OpenOptions
    pub fn new() -> Self {
        OpenOptions
    }

    /// Open a database at the given path
    pub async fn open(&self, _path: &str) -> Result<()> {
        Ok(())
    }
}
