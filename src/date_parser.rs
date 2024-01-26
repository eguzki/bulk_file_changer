use chrono::{DateTime, NaiveDate, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

// https://github.com/waltzofpearls/dateparser/blob/main/dateparser/src/lib.rs
// https://github.com/waltzofpearls/dateparser/blob/main/dateparser/src/datetime.rs#L26

pub fn parse(input: &str) -> Option<DateTime<Utc>> {
    img_yyyymmdd(input)
}

fn img_yyyymmdd(input: &str) -> Option<DateTime<Utc>> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"^(IMG[-_])*(?P<yyyy>[0-9]{4})(?P<mm>[0-9]{2})(?P<dd>[0-9]{2}).*$",)
                .unwrap();
    }

    if !RE.is_match(input) {
        return None;
    }

    if let Some(caps) = RE.captures(input) {
        if let Some(matched_yyyy) = caps.name("yyyy") {
            if let Some(matched_mm) = caps.name("mm") {
                if let Some(matched_dd) = caps.name("dd") {
                    return Some(
                        NaiveDate::from_ymd_opt(
                            FromStr::from_str(matched_yyyy.as_str()).unwrap(),
                            FromStr::from_str(matched_mm.as_str()).unwrap(),
                            FromStr::from_str(matched_dd.as_str()).unwrap(),
                        )
                        .unwrap()
                        .and_hms_milli_opt(0, 0, 0, 0)
                        .unwrap()
                        .and_utc(),
                    );
                }
            }
        }
    }

    None
}
