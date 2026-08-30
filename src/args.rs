use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "Find occurrences of text in images")]
pub struct Args {
    pub input: String,

    pub search_directory: Option<PathBuf>,

    #[arg(short, long, help = "Cache directory")]
    pub cache_directory: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Indexing languages (ISO 639-2)",
        default_values_t = ["eng".to_string(), "deu".to_string()]
    )]
    pub languages: Vec<String>,
}
