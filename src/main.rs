use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, Write};

mod config;
mod isbn;

#[derive(Debug, Serialize, Deserialize)]
struct Book {
    isbn: String,
    title: String,
    author: String,
    rating: u8,
    reading_date: NaiveDate,
    has_review: bool,
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

fn ask(question: &str) -> String {
    print!("{}", question);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn save_book(book: Book) -> Result<(), Box<dyn Error>> {
    let path = &config::get_config().book_file;
    let file = File::create(path).expect("Failed to create file books.csv");
    let mut wtr = csv::Writer::from_writer(file);

    wtr.serialize(book).expect("Failed to serialize book");
    let _ = wtr.flush();
    Ok(())
}

fn add_new_book() {
    let isbn = loop {
        let answer = ask("Please input the ISBN number of your book: ");
        let answer = answer.replace("-", "");
        let answer = answer.trim().to_string();
        if isbn::check_isbn(&answer) {
            break answer;
        }
        println!("wrong ISBN format");
    };
    let title: String;
    let author: String;
    println!("Requesting book information from openlibrary.org");
    if let Ok((_title, _author)) = isbn::api::request_book_with_isbn(&isbn) {
        author = _author;
        title = _title;
    } else {
        println!("Failed to call isbn api");
        title = ask("Please input the books title: ");
        author = ask("Please input the authors name: ");
    }
    let rating = loop {
        let answer = ask("Please rate the book from 0 to 10: ");
        if let Ok(rating) = answer.parse::<u8>() {
            if rating <= 10 {
                break rating;
            }
        }
        println!("Wrong format for rating");
    };
    let book = Book {
        isbn: isbn,
        title: title,
        author: author,
        rating: rating,
        reading_date: Local::now().date_naive(),
        has_review: false,
    };
    println!("this book is added to the file book: \n{}", book);
    let _ = save_book(book);
}
fn retrieve_books_from_file(path: &str) -> Result<Vec<Book>, Box<dyn Error>> {
    let mut books = Vec::new();
    println!("path: {}", path);
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.deserialize() {
        let book: Book = result?;
        books.push(book);
    }
    Ok(books)
}

fn list_all_books() {
    let path = &config::get_config().book_file;
    let books = match retrieve_books_from_file(path) {
        Ok(v) => v,
        Err(e) => {
            println!("failed to read book file att path {} because: {}", path, e);
            return;
        }
    };
    for book in books {
        println!("{}", book);
    }
}

fn run_option(option: &str) {
    let first_letter = match option.chars().next() {
        Some(v) => v,
        None => {
            println!("option string is empty");
            return;
        }
    };
    match first_letter {
        'n' => add_new_book(),
        'l' => list_all_books(),
        _ => println!("{} is not a valid option", first_letter),
    }
}

fn main() {
    config::init_config();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("nothing to do with so few arguments");
        return;
    }
    if (&args[1]).starts_with('-') {
        run_option(&(&args[1])[1..]);
    } else {
        println!("{} could not be read as an option", &args[1]);
    }
}
