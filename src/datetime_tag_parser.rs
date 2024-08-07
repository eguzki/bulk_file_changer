use exif::{DateTime, In, Tag, Value};
use std::fs::File;
use std::path::Path;

pub fn captures(path: &Path) -> Result<Option<chrono::DateTime<chrono::Utc>>, String> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    return match exifreader.read_from_container(&mut bufreader) {
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
            match exif.get_field(Tag::DateTime, In::PRIMARY) {
                Some(field) => match field.value {
                    Value::Ascii(ref vec) if !vec.is_empty() => {
                        match DateTime::from_ascii(&vec[0]) {
                            Ok(datetime) => Ok(Some(datetime)),
                            Err(e) => Err(e.to_string()),
                        }
                    }
                    _ => Ok(None),
                },
                None => Ok(None),
            };

            //if let Some(field) = exif.get_field(Tag::DateTime, In::PRIMARY) {
            //    match field.value {
            //        Value::Ascii(ref vec) if !vec.is_empty() => {
            //            if let Ok(datetime) = DateTime::from_ascii(&vec[0]) {
            //                println!("Year of DateTime is {}.", datetime.year);
            //            }
            //        }
            //        _ => {}
            //    }
            //}
        }
        Err(e) => return Err(e.to_string()),
    };
}
