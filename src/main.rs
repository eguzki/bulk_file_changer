use clap::Parser;
use walkdir::{DirEntry, WalkDir};
mod date_parser;
mod datetime_tag_parser;
mod datetime_tag_writer;
mod stats;

/// Search for a pattern in a file and change modification time from the filename
#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// The path to the file to read
    path: std::path::PathBuf,
}

fn is_dir(entry: &DirEntry) -> bool {
    entry.file_type().is_dir()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn main() {
    let args = Cli::parse();

    env_logger::Builder::new()
        .filter_level(args.verbose.log_level_filter())
        .init();

    log::debug!("Reading path: {:?}", args.path);

    let s: stats::Stats = WalkDir::new(args.path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| !is_dir(e))
        .collect();
    print!("{}", serde_yaml::to_string(&s).unwrap());
}
