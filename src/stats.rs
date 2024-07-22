use crate::parser;
//use filetime::{set_file_mtime, FileTime};
use little_exif::metadata::Metadata;
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
        let p = parser::Parser::new();

        for i in iter {
            //full path: i.path().display()
            //filename: i.file_name().to_str()
            s.num_files += 1;

            println!("{} {}", i.path().display(), i.file_name().to_str().unwrap());

            match Metadata::new_from_path(i.path()) {
                Ok(metadata) => {
                    println!("=== HELLO!");
                    for tag in metadata.data() {
                        println!("{:?}", tag);
                    }
                }
                Err(e) => println!("error parsing metadata: {e:?}"),
            }

            println!("=== HELLO! 222");
            //match p.captures(i.file_name().to_str().unwrap()) {
            //    None => {
            //        s.num_skipped_files += 1;
            //        s.skipped_files
            //            .push(String::from(i.path().to_str().unwrap()));
            //    }
            //    Some(date_time) => {
            //        s.num_parsed_files += 1;
            //        let mtime = FileTime::from_unix_time(date_time.timestamp(), 0);
            //        set_file_mtime(i.path().to_str().unwrap(), mtime).unwrap();
            //    }
            //}
        }

        s
    }
}
