use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub author: String,
    pub rating: u8,
    pub reading_date: NaiveDate,
    pub has_review: bool,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ISBN: {}\nTITLE: {}\nAUTHOR: {}\nRATING: {}/10\nREADING DATE: {}",
            self.isbn,
            self.title,
            self.author,
            self.rating,
            self.reading_date.format("%Y-%m-%d"),
        )
    }
}
