use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::{customers, purchase_orders};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Order {
    id: Option<i64>,
    customer_id: Option<i64>,
    customer_addr: Option<String>,
    book_id: Option<i64>,
    shipped: Option<i64>,
}

#[post("/new", data = "<order>")]
pub fn create_new_order(order: Json<Order>) -> Result<(), String> {
    let cid = match order.customer_id {
        Some(c) => c,
        None => return Err("No customer id provided".to_string()),
    };
    let bid = match order.book_id {
        Some(b) => b,
        None => return Err("No book id provided".to_string()),
    };

    purchase_orders::create_purchase_order(cid, bid);
    Ok(())
}

#[get("/shipped", format = "json", data = "<order>")]
pub fn get_order_shipping_status(order: Json<Order>) -> Result<Json<Order>, String> {
    let cid = match order.customer_id {
        Some(c) => c,
        None => return Err("No customer id provided".into()),
    };
    let bid = match order.book_id {
        Some(b) => b,
        None => return Err("No book id provided".into()),
    };

    let addr = customers::get_customer_address(cid);

    let oid = purchase_orders::get_purchase_order_id(cid, bid);
    let shipped = purchase_orders::get_purchase_order_shipping_status(oid);
    Ok(Json(Order {
        id: Some(oid),
        customer_id: Some(cid),
        customer_addr: Some(addr),
        book_id: Some(bid),
        shipped: Some(shipped),
    }))
}

#[put("/ship", data = "<order>")]
pub fn ship_order(order: Json<Order>) -> Result<(), String> {
    let oid = match order.id {
        Some(o) => o,
        None => return Err("No order id provided".to_string()),
    };

    purchase_orders::update_purchase_order_shipping_status_to_shipped(oid);
    Ok(())
}
