//! # img-search
//!
//! A library for indexing images with Tesseract OCR and searching the extracted
//! text contents.
//!
//! The crate provides two main functions:
//!
//! - [`update_tesseract_cache`] recursively scans a directory for supported image
//!   files, extracts their text using Tesseract, and stores the OCR results in a
//!   cache directory.
//! - [`exact_search`] searches the cached OCR text and returns the original paths
//!   of images containing the requested text.
//!
//! Supported image formats are:
//!
//! - PNG
//! - JPEG
//! - WebP
//! - TIFF
//!
//! Cached OCR results are associated with the image's modification timestamp.
//! An image is processed again only when its cached data is missing or the image
//! has been modified.
//!
//! # Requirements
//!
//! Tesseract must be installed and available through the `tesseract` command.
//! The required Tesseract language data must also be installed.
//!
//! # Example
//!
//! ```no_run
//! use std::path::Path;
//! use img_search::{exact_search, update_tesseract_cache};
//!
//! fn main() -> anyhow::Result<()> {
//!     let image_dir = Path::new("./images");
//!     let cache_dir = Path::new("./cache");
//!     let languages = vec!["eng".to_string(), "deu".to_string()];
//!
//!     update_tesseract_cache(
//!         image_dir,
//!         cache_dir,
//!         &languages,
//!         4,
//!     )?;
//!
//!     let results = exact_search(
//!         cache_dir,
//!         "invoice".to_string(),
//!     )?;
//!
//!     for path in results {
//!         println!("{path}");
//!     }
//!
//!     Ok(())
//! }
//! ```

mod cache;
mod exact_search;

pub use cache::update_tesseract_cache;
pub use exact_search::exact_search;
