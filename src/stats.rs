use walkdir::DirEntry;

// Parsing stats
#[derive(Debug)]
pub struct Stats {
    num_files: u32,
    num_parsed_files: u32,
    num_skipped_files: u32,
    skipped_files: Vec<String>,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            num_files: 0,
            num_parsed_files: 0,
            num_skipped_files: 0,
            skipped_files: Vec::new(),
        }
    }
}

impl FromIterator<DirEntry> for Stats {
    fn from_iter<I: IntoIterator<Item = DirEntry>>(iter: I) -> Self {
        Stats::new()
    }
}
