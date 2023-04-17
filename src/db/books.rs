use core::panic;

use log::{error, info};

use super::database::connect;

pub fn create_book(title: String, author: String, price: f64) {
    let db = connect();

    let query = "INSERT INTO books (title, author, price) VALUES (:title, :author, :price);";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing to create book id: {}", error);
        panic!("Failed, yo {}", error)
    });
    stmt.execute(&[
        (":title", &title),
        (":author", &author),
        (":price", &format!("{}", price)),
    ])
    .unwrap_or_else(|error| {
        error!(target: "error", "Error while creating book: {}", error);
        panic!("Failed, yo {}", error)
    });

    info!(target: "info", "A book with Title \"{}\" and Author \"{}\" has been added.", title, author)
}

pub fn get_book_id(title: String, author: String) -> i64 {
    let db = connect();

    let query = "SELECT id FROM books WHERE title = :title AND author = :author;";

    let mut stmt = db
        .prepare(query)
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while preparing to get book id for title \"{}\" and author \"{}\": {}", title, author, error);
            panic!("Error while preparing to get book id for title \"{}\" and author \"{}\": {}", title, author, error);
        });
    let mut rows = stmt
        .query_map(&[(":title", &title), (":author", &author)], |row| row.get(0))
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while fetching book id for title \"{}\" and author \"{}\": {}", title, author, error);
            panic!("Error while fetching book id for title \"{}\" and author \"{}\": {}", title, author, error);
        });

    let id = rows
        .next()
        .unwrap_or_else(|| {
            error!(target: "error", "Failed to handle next book in database");
            panic!("Failed to handle next book in database")
        })
        .unwrap_or_else(|error| {
            error!(target: "error", "Failed to process book rows: {}", error);
            panic!("Failed to process book rows: {}", error);
        });

    info!(target: "info", "Book with title \"{}\" and author \"{}\" has id {}.", title, author, id);

    id
}

pub fn get_book_price(bid: i64) -> f64 {
    let db = connect();

    let query = "SELECT price FROM books WHERE id = :bid";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing book id \"{}\": {}", bid, error);
        panic!("Error while preparing book id \"{}\": {}", bid, error);
    });

    let mut rows = stmt
        .query_map(&[(":bid", &bid)], |row| row.get(0))
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while requesting book id \"{}\" price: {}", bid, error);
            panic!(
                "Error while requesting book id \"{}\" price: {}",
                bid, error
            );
        });

    let id = rows
        .next()
        .unwrap_or_else(|| {
            error!(target: "error", "Failed to handle next book id in database");
            panic!("Failed, yo.")
        })
        .unwrap_or_else(|error| {
            error!(target: "error", "Failed to process book id rows: {}", error);
            panic!("Failed to process book id rows: {}", error);
        });

    info!(target:"info", "The price of the book with id \"{}\" was checked.", bid);

    id
}
