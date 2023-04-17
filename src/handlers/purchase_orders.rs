use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::db::{customers, purchase_orders};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PurchaseOrder {
    id: Option<i64>,
    book_id: Option<i64>,
    customer_id: Option<i64>,
    customer_addr: Option<String>,
    shipped: Option<i64>,
}

#[post("/create_new_order", data = "<order>")]
pub fn create_new_order(order: Json<PurchaseOrder>) -> Result<(), String> {
    let bid = match order.book_id {
        Some(b) => b,
        None => return Err("No book id provided".to_string()),
    };
    let cid = match order.customer_id {
        Some(c) => c,
        None => return Err("No customer id provided".to_string()),
    };

    purchase_orders::create_purchase_order(cid, bid);
    Ok(())
}

#[get("/get_order_shipping_status", format = "json", data = "<order>")]
pub fn get_order_shipping_status(
    order: Json<PurchaseOrder>,
) -> Result<Json<PurchaseOrder>, String> {
    let bid = match order.book_id {
        Some(b) => b,
        None => return Err("No book id provided".into()),
    };
    let cid = match order.customer_id {
        Some(c) => c,
        None => return Err("No customer id provided".into()),
    };

    let addr = customers::get_customer_address(cid);

    let oid = purchase_orders::get_purchase_order_id(cid, bid);
    let shipped = purchase_orders::get_purchase_order_shipping_status(oid);
    Ok(Json(PurchaseOrder {
        id: Some(oid),
        book_id: Some(bid),
        customer_id: Some(cid),
        customer_addr: Some(addr),
        shipped: Some(shipped),
    }))
}

#[put("/ship_order", data = "<order>")]
pub fn ship_order(order: Json<PurchaseOrder>) -> Result<(), String> {
    let oid = match order.id {
        Some(o) => o,
        None => return Err("No order id provided".to_string()),
    };

    purchase_orders::update_purchase_order_shipping_status_to_shipped(oid);
    Ok(())
}
