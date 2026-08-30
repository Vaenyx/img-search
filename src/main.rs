use std::path::Path;

use anyhow::Result;
use img_search::{exact_search, update_tesseract_cache};
use indicatif::{ProgressBar, ProgressStyle};

fn main() -> Result<()> {
    //let search_dir = env::current_dir()?;
    let search_dir = Path::new("/home/vaenyx/testmc/screenshots/");
    let cache_dir = Path::new("/home/vaenyx/.cache/img-search");
    let languages = vec!["eng", "deu"];
    let query = "good girl";

    let status = ProgressBar::new_spinner();

    status.set_style(ProgressStyle::with_template("{msg}")?);

    status.set_message(format!("Indexing {:?}", search_dir));

    update_tesseract_cache(search_dir, cache_dir, &languages)?;

    // Removes "Updating OCR cache..." from the terminal
    status.finish_and_clear();

    let results = exact_search(cache_dir, query)?;

    for res in results {
        println!("{res}");
    }

    Ok(())
}
