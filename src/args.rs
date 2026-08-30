use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Find occurrences of text in images")]
pub struct Args {
    #[arg(help = "Input (case insensitive)")]
    pub input: String,

    #[arg(help = "Search directory to match input against")]
    pub search_directory: Option<PathBuf>,

    #[arg(short, long, help = "Cache directory to store ocr representations")]
    pub cache_directory: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Indexing languages for ocr (ISO 639-2)",
        default_values_t = ["eng".to_string(), "deu".to_string()]
    )]
    pub languages: Vec<String>,
}
