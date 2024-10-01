use chrono::NaiveDate;
use exif::{DateTime, In, Tag, Value};
use std::fs::File;
use std::path::Path;

pub fn captures(path: &Path) -> Result<Option<chrono::DateTime<chrono::Utc>>, exif::Error> {
    let file = File::open(path)?;

    let mut bufreader = std::io::BufReader::new(&file);
    let result = exif::Reader::new()
        .continue_on_error(true)
        .read_from_container(&mut bufreader);

    let exif = match result {
        Err(exif::Error::PartialResult(partial)) => {
            let (exif, _) = partial.into_inner();
            exif
        }
        Err(exif::Error::NotFound(_)) => return Ok(None),
        Ok(exif) => exif,
        Err(e) => return Err(e),
    };

    //for f in exif.fields() {
    //    println!(
    //        "  {}/{}: {}",
    //        f.ifd_num.index(),
    //        f.tag,
    //        f.display_value().with_unit(&exif)
    //    );
    //    println!("      {:?}", f.value);
    //}

    match exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
        Some(field) => match field.value {
            Value::Ascii(ref vec) if !vec.is_empty() => {
                let datetime = DateTime::from_ascii(&vec[0])?;
                Ok(Some(
                    NaiveDate::from_ymd_opt(
                        i32::from(datetime.year),
                        u32::from(datetime.month),
                        u32::from(datetime.day),
                    )
                    .unwrap()
                    .and_hms_milli_opt(
                        u32::from(datetime.hour),
                        u32::from(datetime.minute),
                        u32::from(datetime.second),
                        0,
                    )
                    .unwrap()
                    .and_utc(),
                ))
            }
            _ => Ok(None),
        },
        None => Ok(None),
    }

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
