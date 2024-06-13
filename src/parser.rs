use chrono::{DateTime, Utc};
use regex::Regex;

pub struct Parser {
    patterns: Vec<Regex>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            patterns: vec![
                Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap(),
                Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap(),
            ],
        }
    }

    pub fn captures(&self, filename: &str) -> Option<DateTime<Utc>> {
        //self.patterns.iter().find_map();
        // https://docs.rs/regex/latest/regex/#example-finding-dates-in-a-haystack
        // Check https://rust-lang-nursery.github.io/rust-cookbook/text/regex.html#replace-all-occurrences-of-one-text-pattern-with-another-pattern
        //    let set = RegexSet::new(&[
        //        r"\w+", r"\d+", r"\pL+", r"foo", r"bar", r"barfoo", r"foobar",
        //    ])
        //    .unwrap();
        None
    }
}

// Implement `Iterator` for `Fibonacci`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
//impl Iterator for Parser {
//    // We can refer to this type using Self::Item
//    type Item = u32;
//
//    // Here, we define the sequence using `.curr` and `.next`.
//    // The return type is `Option<T>`:
//    //     * When the `Iterator` is finished, `None` is returned.
//    //     * Otherwise, the next value is wrapped in `Some` and returned.
//    // We use Self::Item in the return type, so we can change
//    // the type without having to update the function signatures.
//    fn next(&mut self) -> Option<Self::Item> {
//        let current = self.curr;
//
//        self.curr = self.next;
//        self.next = current + self.next;
//
//        // Since there's no endpoint to a Fibonacci sequence, the `Iterator`
//        // will never return `None`, and `Some` is always returned.
//        Some(current)
//    }
//}
