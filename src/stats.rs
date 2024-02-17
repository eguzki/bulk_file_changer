use regex::RegexSet;
use serde::Serialize;
use walkdir::DirEntry;

// Parsing stats
#[derive(Serialize, PartialEq, Debug)]
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
        let mut s = Stats::new();

        let set = RegexSet::new(&[
            r"\w+", r"\d+", r"\pL+", r"foo", r"bar", r"barfoo", r"foobar",
        ])
        .unwrap();

        for i in iter {
            //full path:
            //println!("{}", i.path().display());
            //filename
            //println!("{}", i.file_name().to_str().unwrap_or(""));
            s.num_files += 1;
            // Iterate over and collect all of the matches. Each match corresponds to the
            // ID of the matching pattern.
            // https://docs.rs/regex/latest/regex/#example-finding-dates-in-a-haystack
            // Check https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html#replace-all-occurrences-of-one-text-pattern-with-another-pattern
            let matches: Vec<_> = set
                .matches(i.file_name().to_str().unwrap_or(""))
                .into_iter()
                .collect();
        }

        s
    }
}
