use clap::Parser;
use walkdir::{DirEntry, WalkDir};
mod stats;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
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
    println!("Reading path: {:?}", args.path);

    let s: stats::Stats = WalkDir::new(args.path)
        .into_iter()
        .filter_entry(|e| !is_hidden(e))
        .filter_map(|e| e.ok())
        .filter(|e| !is_dir(e))
        .collect();
    println!("{:?}", s);
    //    .for_each(|e| println!("{}", e.file_name().to_str().unwrap_or("")));
    //    .for_each(|e| println!("{}", e.path().display()));
}
