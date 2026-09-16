use anyhow::{Ok, Result};
use std::{
    fs::{DirEntry, read_dir, read_to_string},
    path::Path,
};

/// Searches cached OCR text for an exact substring.
///
/// The search is case-insensitive and returns the original image paths whose
/// cached OCR text contains `query`.
///
/// # Errors
///
/// Returns an error if the cache directory or one of its cache files cannot
/// be read.
pub fn exact_search(cache_dir: &Path, query: String) -> Result<Vec<String>> {
    let directory_entries = read_dir(cache_dir)?;

    let text_cache_files: Vec<DirEntry> = directory_entries
        .flatten()
        .filter(|entry| {
            let path = entry.path();

            entry.metadata().is_ok_and(|metadata| metadata.is_file())
                && path.extension().and_then(|extension| extension.to_str()) == Some("txt")
        })
        .collect();

    let results = text_cache_files
        .into_iter()
        .map(|file| -> Result<Option<String>> {
            let file_path = file.path();
            let content = read_to_string(&file_path)?;

            if !content.to_lowercase().contains(query.as_str()) {
                return Ok(None);
            }

            let original_path = read_to_string(file_path.with_extension("path"))?;

            Ok(Some(original_path))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    Ok(results)
}
