use std::fs::{File, create_dir_all, metadata, read_to_string, write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::SystemTime;

use anyhow::{Ok, Result, bail};
use indicatif::{ProgressBar, ProgressStyle};
use walkdir::WalkDir;

fn get_cached_mtime(cache_dir: &Path, cache_id: &str) -> Option<u64> {
    let mtime_path = cache_dir.join(format!("{cache_id}.mtime"));

    read_to_string(mtime_path)
        .ok()
        .and_then(|contents| contents.trim().parse::<u64>().ok())
}

fn needs_cache_update(image_path: &Path, cache_dir: &Path) -> bool {
    let Some(image_path_str) = image_path.to_str() else {
        return false;
    };

    let cache_id = sha256::digest(image_path_str.as_bytes());

    let text_file = cache_dir.join(format!("{cache_id}.txt"));
    let path_file = cache_dir.join(format!("{cache_id}.path"));
    let mtime_file = cache_dir.join(format!("{cache_id}.mtime"));

    if !text_file.exists() || !path_file.exists() || !mtime_file.exists() {
        return true;
    }

    let current_mtime = metadata(image_path)
        .and_then(|metadata| metadata.modified())
        .unwrap()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let cached_mtime = get_cached_mtime(cache_dir, &cache_id);

    cached_mtime != Some(current_mtime)
}

fn update_cached_file(image_path: &Path, cache_dir: &Path, languages: &[String]) -> Result<()> {
    let Some(image_path_str) = image_path.to_str() else {
        return Ok(());
    };

    let modified_time = metadata(image_path)?
        .modified()?
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();

    let cache_id = sha256::digest(image_path_str.as_bytes());

    let text_cache_path = cache_dir.join(format!("{cache_id}.txt"));
    let path_cache_path = cache_dir.join(format!("{cache_id}.path"));
    let mtime_cache_path = cache_dir.join(format!("{cache_id}.mtime"));

    let output_file = File::create(&text_cache_path)?;

    let status = Command::new("tesseract")
        .arg(image_path)
        .arg("stdout")
        .arg("-l")
        .arg(languages.join("+"))
        .stdout(Stdio::from(output_file))
        .stderr(Stdio::null())
        .status()?;

    if !status.success() {
        bail!("tesseract failed for {}", image_path.display());
    }

    write(path_cache_path, image_path_str)?;
    write(mtime_cache_path, modified_time.to_string())?;

    Ok(())
}

pub fn update_tesseract_cache(
    search_dir: &Path,
    cache_dir: &Path,
    languages: &[String],
) -> Result<()> {
    create_dir_all(cache_dir)?;

    let image_files: Vec<_> = WalkDir::new(search_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            let path = entry.path();

            entry.file_type().is_file()
                && path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| {
                        matches!(
                            extension.to_ascii_lowercase().as_str(),
                            "png" | "jpg" | "jpeg" | "webp" | "tif" | "tiff"
                        )
                    })
        })
        .collect();

    let files_to_update: Vec<_> = image_files
        .into_iter()
        .filter(|entry| needs_cache_update(entry.path(), cache_dir))
        .collect();

    let progress = ProgressBar::new(files_to_update.len() as u64);

    progress.set_style(
        ProgressStyle::with_template("[{bar:40}] {pos}/{len} {percent}% {msg}")?
            .progress_chars("=> "),
    );

    for entry in &files_to_update {
        progress.set_message(entry.file_name().to_string_lossy().into_owned());

        update_cached_file(entry.path(), cache_dir, languages)?;

        progress.inc(1);
    }

    progress.finish_and_clear();

    Ok(())
}
