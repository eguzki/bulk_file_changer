use crate::datetime_tag_parser;
use crate::parser;
//use filetime::{set_file_mtime, FileTime};
//use little_exif::metadata::Metadata;
use exif::{DateTime, In, Tag, Value};
use serde::Serialize;
use std::fs::File;
use walkdir::DirEntry;

// Parsing stats
#[derive(Serialize, PartialEq, Debug)]
pub struct Stats {
    num_files: u32,
    num_files_with_datetime_tag: u32,
    num_files_failed_tag_parsing: u32,
    num_files_missing_datetime_tag: u32,
    num_files_failed_filename_parsing: u32,
    filenames_tag_unparseable: Vec<String>,
    filenames_name_unparseable: Vec<String>,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            num_files: 0,
            num_files_with_datetime_tag: 0,
            num_files_failed_tag_parsing: 0,
            num_files_missing_datetime_tag: 0,
            num_files_failed_filename_parsing: 0,
            filenames_tag_unparseable: Vec::new(),
            filenames_name_unparseable: Vec::new(),
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

            match datetime_tag_parser::captures(i.path()) {
                Some(_) => s.num_files_with_datetime_tag += 1,
                None => {
                    s.num_files_missing_datetime_tag += 1;
                }
            }

            let file = File::open(i.path()).unwrap();
            let mut bufreader = std::io::BufReader::new(&file);
            let exifreader = exif::Reader::new();
            match exifreader.read_from_container(&mut bufreader) {
                Ok(exif) => {
                    //for f in exif.fields() {
                    //    println!(
                    //        "  {}/{}: {}",
                    //        f.ifd_num.index(),
                    //        f.tag,
                    //        f.display_value().with_unit(&exif)
                    //    );
                    //    println!("      {:?}", f.value);
                    //}
                    // To parse a DateTime-like field, `DateTime::from_ascii` can be used.
                    if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
                        match field.value {
                            Value::Ascii(ref vec) if !vec.is_empty() => {
                                if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
                                    println!("Year of DateTime is {}.", datetime.year);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Err(e) => match &e {
                    exif::Error::NotFound(msg) => println!("No Exif data found: {msg:?}"),
                    other_error => println!("error parsing metadata: {other_error:?}"),
                },
            }
            //let exif = exifreader.read_from_container(&mut bufreader).unwrap();

            //match Metadata::new_from_path(i.path()) {
            //    Ok(metadata) => {
            //        println!("=== HELLO!");
            //        for tag in metadata.data() {
            //            println!("{:?}", tag);
            //        }
            //    }
            //    Err(e) => println!("error parsing metadata: {e:?}"),
            //}

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
