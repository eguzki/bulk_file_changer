use clap::Parser;
use walkdir::{DirEntry, WalkDir};
mod date_parser;
mod datetime_tag_parser;
mod datetime_tag_writer;
mod stats;

/// Add date to image Exif metadata when missing.
#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    /// The path to the base directory
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
