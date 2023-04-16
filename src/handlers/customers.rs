use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::customers;
use crate::input_validation::general_iv::{check_for_alphanumeric_input, standardize_whitespace};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Customer {
    id: Option<i64>,
    name: Option<String>,
    shipping_address: Option<String>,
    account_balance: Option<f64>,
}

#[post("/create_new_customer", data = "<customer>")]
pub fn create_new_customer(customer: Json<Customer>) -> Result<(), String> {
    let mut name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".to_string()),
    };
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };
    // Shadow the user name and address to preserve new formatting
    name = standardize_whitespace(name.clone());
    address = standardize_whitespace(address.clone());

    match check_for_alphanumeric_input(name.clone(), "customer name".to_string(), "new_customer".to_string())
    {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    match check_for_alphanumeric_input(
        address.clone(),
        "customer address".to_string(),
        "new_customer".to_string(),
    ) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    customers::create_customer(name, address);
    Ok(())
}

#[put("/update_customer_address", data = "<customer>")]
pub fn update_customer_address(customer: Json<Customer>) -> Result<(), String> {
    let cid = match customer.id {
        Some(i) => i,
        None => return Err("No id provided".to_string()),
    };
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".to_string()),
    };

    address = standardize_whitespace(address.clone());

    match check_for_alphanumeric_input(
        address.clone(),
        "customer address".to_string(),
        "update_customer_address".to_string(),
    ) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    customers::update_customer_address(cid, address);
    Ok(())
}

#[get("/get_customer_balance", format = "json", data = "<customer>")]
pub fn get_customer_balance(customer: Json<Customer>) -> Result<Json<Customer>, String> {
    let mut name = match customer.name.clone() {
        Some(n) => n,
        None => return Err("No name provided".into()),
    };
    let mut address = match customer.shipping_address.clone() {
        Some(a) => a,
        None => return Err("No address provided".into()),
    };

    name = standardize_whitespace(name.clone());
    address = standardize_whitespace(address.clone());

    match check_for_alphanumeric_input(
        name.clone(),
        "customer name".to_string(),
        "get_customer_balance".to_string(),
    ) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };
    match check_for_alphanumeric_input(
        address.clone(),
        "customer address".to_string(),
        "get_customer_balance".to_string(),
    ) {
        Ok(()) => 0,
        Err(user_error_message) => return Err(user_error_message),
    };

    let cid = customers::get_customer_id(name.clone(), address.clone());
    let balance = customers::customer_balance(cid);
    Ok(Json(Customer {
        id: Some(cid),
        name: Some(name),
        shipping_address: Some(address),
        account_balance: Some(balance),
    }))
}
