use crate::{
    db::books,
    input_validation::general_iv::{
        check_for_alphanumeric_input, standardize_price, standardize_whitespace,
    },
};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Book {
    id: Option<i64>,
    title: Option<String>,
    author: Option<String>,
    price: Option<f64>,
}

#[post("/create_new_book", data = "<book>")]
pub fn create_new_book(book: Json<Book>) -> Result<(), String> {
    let mut title = match book.title.clone() {
        Some(t) => t,
        None => return Err("No title provided".to_string()),
    };
    let mut author = match book.author.clone() {
        Some(a) => a,
        None => return Err("No author provided".to_string()),
    };
    let price = match book.price {
        Some(p) => p,
        None => return Err("No price provided".to_string()),
    };

    title = standardize_whitespace(title.clone());
    author = standardize_whitespace(author.clone());

    match check_for_alphanumeric_input(
        title.clone(),
        "book title".to_string(),
        "create_new_book".to_string(),
    ) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    match check_for_alphanumeric_input(
        author.clone(),
        "book author".to_string(),
        "create_new_book".to_string(),
    ) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    match standardize_price(price, "create_new_book".to_string()) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    books::create_book(title, author, price);
    Ok(())
}

// yes this throws a warning, it's how we're going it
// get methods can consume data in my world
// because putting and posting to get the price makes less
// sense in my mind
#[get("/get_book_price", format = "json", data = "<book>")]
pub fn get_book_price(book: Json<Book>) -> Result<Json<Book>, String> {
    let mut title = match book.title.clone() {
        Some(t) => t,
        None => return Err("No title provided".into()),
    };
    let mut author = match book.author.clone() {
        Some(a) => a,
        None => return Err("No author provided".into()),
    };

    title = standardize_whitespace(title.clone());
    author = standardize_whitespace(author.clone());

    match check_for_alphanumeric_input(
        title.clone(),
        "book title".to_string(),
        "create_new_book".to_string(),
    ) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    match check_for_alphanumeric_input(
        author.clone(),
        "book author".to_string(),
        "create_new_book".to_string(),
    ) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    let bid = books::get_book_id(title.clone(), author.clone());
    let price = books::get_book_price(bid);
    Ok(Json(Book {
        id: Some(bid),
        title: Some(title),
        author: Some(author),
        price: Some(price),
    }))
}
