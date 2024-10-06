use crate::datetime_tag_parser;
use crate::datetime_tag_writer;
use serde::Serialize;
use std::ffi::OsStr;
use walkdir::DirEntry;

// Parsing stats
#[derive(Serialize, PartialEq, Debug)]
pub struct Stats {
    num_files: u32,
    num_files_not_images: u32,
    num_files_with_datetime_tag: u32,
    num_files_failed_tag_parsing: u32,
    num_files_missing_datetime_tag: u32,
    num_files_failed_filename_parsing: u32,
    num_files_successfully_tagged: u32,
    num_files_failed_tagging: u32,
    filenames_not_images: Vec<String>,
    filenames_tag_unparseable: Vec<String>,
    filenames_name_unparseable: Vec<String>,
    filenames_name_untaggeable: Vec<String>,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            num_files: 0,
            num_files_not_images: 0,
            num_files_with_datetime_tag: 0,
            num_files_failed_tag_parsing: 0,
            num_files_missing_datetime_tag: 0,
            num_files_failed_filename_parsing: 0,
            num_files_successfully_tagged: 0,
            num_files_failed_tagging: 0,
            filenames_not_images: Vec::new(),
            filenames_tag_unparseable: Vec::new(),
            filenames_name_unparseable: Vec::new(),
            filenames_name_untaggeable: Vec::new(),
        }
    }
}

impl FromIterator<DirEntry> for Stats {
    fn from_iter<I: IntoIterator<Item = DirEntry>>(iter: I) -> Self {
        let mut s = Stats::new();

        for i in iter {
            //full path: i.path().display()
            //filename: i.file_name().to_str()
            s.num_files += 1;

            if let Some(ext) = i.path().extension().and_then(OsStr::to_str) {
                if !["jpeg", "jpg", "png"].contains(&ext) {
                    s.num_files_not_images += 1;
                    s.filenames_not_images.push(i.path().display().to_string());
                    continue;
                }
            }

            log::debug!("{} {}", i.path().display(), i.file_name().to_str().unwrap());

            let datetime_tag = match datetime_tag_parser::captures(i.path()) {
                Ok(datetime_tag) => datetime_tag,
                Err(e) => {
                    log::error!("{} parsing metadata: {e:?}", i.path().display());
                    s.num_files_failed_tag_parsing += 1;
                    s.filenames_tag_unparseable
                        .push(i.path().display().to_string());
                    continue;
                }
            };

            match datetime_tag {
                Some(_) => s.num_files_with_datetime_tag += 1,
                None => {
                    s.num_files_missing_datetime_tag += 1;
                    match crate::date_parser::parse(i.file_name().to_str().unwrap()) {
                        Some(tag) => match datetime_tag_writer::write_tag(i.path(), tag) {
                            Ok(_) => s.num_files_successfully_tagged += 1,
                            Err(e) => {
                                log::error!("{} tagging metadata: {e:?}", i.path().display());
                                s.num_files_failed_tagging += 1;
                                s.filenames_name_untaggeable
                                    .push(i.path().display().to_string());
                                continue;
                            }
                        },
                        None => {
                            s.num_files_failed_filename_parsing += 1;
                            s.filenames_name_unparseable
                                .push(i.path().display().to_string());
                        }
                    };
                }
            };
        }
        s
    }
}
