#[macro_use]
extern crate rocket;
extern crate serde;

mod db;
mod handlers;
mod input_validation;

#[launch]
fn rocket() -> _ {
    log4rs::init_file("log4rs.yaml", Default::default())
        .unwrap_or_else(|error| println!("Error while loading `log4rs` config: {}", error));

    rocket::build()
        .mount("/books", routes![handlers::books::create_new_book])
        .mount("/books", routes![handlers::books::get_book_price])
        .mount(
            "/customers",
            routes![handlers::customers::create_new_customer],
        )
        .mount(
            "/customers",
            routes![handlers::customers::get_customer_balance],
        )
        .mount(
            "/customers",
            routes![handlers::customers::update_customer_address],
        )
        .mount(
            "/orders",
            routes![handlers::purchase_orders::create_new_order],
        )
        .mount(
            "/orders",
            routes![handlers::purchase_orders::get_order_shipping_status],
        )
        .mount("/orders", routes![handlers::purchase_orders::ship_order])
    // .mount("/orders", routes![handlers::orders::get_order_status])
}
