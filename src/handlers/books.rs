use crate::db::books;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    id: Option<i64>,
    title: Option<String>,
    author: Option<String>,
    price: Option<f64>,
}

#[post("/new", data = "<book>")]
pub fn create_new_book(book: Json<Book>) -> Result<(), String> {
    let title = match book.title.clone() {
        Some(t) => t,
        None => return Err("No title provided".to_string()),
    };
    let author = match book.author.clone() {
        Some(a) => a,
        None => return Err("No author provided".to_string()),
    };
    let price = match book.price {
        Some(p) => p,
        None => return Err("No price provided".to_string()),
    };

    books::create_book(title, author, price);
    Ok(())
}

// yes this throws a warning, it's how we're going it
// get methods can consume data in my world
// because putting and posting to get the price makes less
// sense in my mind
#[get("/price", format = "json", data = "<book>")]
pub fn get_book_price(book: Json<Book>) -> Result<Json<Book>, String> {
    let title = match book.title.clone() {
        Some(t) => t,
        None => return Err("No title provided".into()),
    };
    let tmp_title = title.clone();
    let author = match book.author.clone() {
        Some(a) => a,
        None => return Err("No author provided".into()),
    };
    let tmp_author = author.clone();

    let bid = books::get_book_id(title, author);
    let price = books::get_book_price(bid);
    Ok(Json(Book {
        id: Some(bid),
        title: Some(tmp_title),
        author: Some(tmp_author),
        price: Some(price),
    }))
}
