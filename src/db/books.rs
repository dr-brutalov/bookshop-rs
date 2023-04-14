use log::info;

use super::database::connect;

pub fn create_book(title: String, author: String, price: f64) {
    let db = connect();
    db
    .prepare("INSERT INTO books (title, author, price) VALUES (:title, :author, :price)")
    .expect("expected to be able to insert into Books table")
    .execute(&[(":title", &title), (":author", &author), (":price", &format!("{}", price))])
    .expect("expected to be able to insert into Books table");
    info!(target: "info", "A book with Title {} and Author {} has been added", title, author)
}

pub fn get_book_id(title: String, author: String) -> i64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT id FROM books WHERE title = :title AND author = :author")
        .expect("expected to be able to select from Books table");
    let mut rows = stmt
        .query_map(&[(":title", &title), (":author", &author)], |row| row.get(0))
        .expect("expected to be able to get id from Books table");
    
    rows.next().unwrap().unwrap()
}

pub fn get_book_price(bid: i64) -> f64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT price FROM books WHERE id = :bid")
        .expect("expected to be able to select from Books table");
    let mut rows = stmt
        .query_map(&[(":bid", &bid)], |row| row.get(0))
        .expect("expected to be able to get price from Books table");
    info!(target:"info", "The price of the book with ID {} was checked.", bid);
    
    rows.next().unwrap().unwrap()
}
