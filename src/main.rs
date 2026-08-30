use anyhow::Result;
use dirs::config_dir;
use img_search::{exact_search, update_tesseract_cache};
use indicatif::{ProgressBar, ProgressStyle};
use std::env;

use clap::Parser;

mod args;

fn main() -> Result<()> {
    let args = args::Args::parse();
    let search_directory = args.search_directory.unwrap_or(env::current_dir()?);
    let cache_directory = args
        .cache_directory
        .unwrap_or(config_dir().unwrap().join("img-search"));

    let status = ProgressBar::new_spinner();
    status.set_style(ProgressStyle::with_template("{msg}")?);

    status.set_message(format!("Indexing {:?}", search_directory));

    update_tesseract_cache(
        &search_directory,
        &cache_directory,
        &args.languages,
        args.thread_count,
    )?;

    status.finish_and_clear();

    let results = exact_search(&cache_directory, args.input)?;

    println!("{}", results.join("\n"));

    Ok(())
}
