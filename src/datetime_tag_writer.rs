use chrono::{DateTime, Utc};
use little_exif::exif_tag::ExifTag;
use little_exif::metadata::Metadata;
use std::path::Path;

pub fn write_tag(path: &Path, date_tag: DateTime<Utc>) -> Result<(), String> {
    let mut metadata = match Metadata::new_from_path(path) {
        Ok(metadata) => metadata,
        Err(e) => return Err(e.to_string()),
    };

    metadata.set_tag(ExifTag::DateTimeOriginal(
        date_tag.format("%Y:%m:%d %H:%M:%S").to_string(),
    ));
    //metadata.set_tag(ExifTag::ModifyDate(
    //    date_tag.format("%Y:%m:%d %H:%M:%S").to_string(),
    //));
    //metadata.set_tag(ExifTag::CreateDate(
    //    date_tag.format("%Y:%m:%d %H:%M:%S").to_string(),
    //));

    match metadata.write_to_file(path) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
