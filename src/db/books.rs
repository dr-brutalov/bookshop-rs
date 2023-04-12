use super::db::connect;

pub fn create_book(title: String, author: String, price: f64) {
    let db = connect();
    db.execute(
        "INSERT INTO books (title, author, price) VALUES (?1, ?2, ?3)",
        [&title, &author, &format!("{}", price)],
    )
    .expect("expected to be able to insert into Books table");
}

pub fn get_book_id(title: String, author: String) -> i64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT id FROM books WHERE title = ?1 AND author = ?2")
        .expect("expected to be able to select from Books table");
    let mut rows = stmt
        .query_map([&title, &author], |row| row.get(0))
        .expect("expected to be able to get id from Books table");
    
    rows.next().unwrap().unwrap()
}

pub fn get_book_price(bid: i64) -> f64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT price FROM books WHERE id = ?1")
        .expect("expected to be able to select from Books table");
    let mut rows = stmt
        .query_map([&bid], |row| row.get(0))
        .expect("expected to be able to get price from Books table");
    
    rows.next().unwrap().unwrap()
}
