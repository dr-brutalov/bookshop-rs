use super::db::connect;

pub fn create_customer(name: String, address: String) -> i64 {
    let db = connect();
    db.execute(
        "INSERT INTO customers (name, shippingAddress, accountBalance) VALUES (?1, ?2, 5.0)",
        [&name, &address],
    )
    .expect("expected to be able to insert into Customers table");

    get_customer_id(name, address)
}

pub fn get_customer_id(name: String, address: String) -> i64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT id FROM customers WHERE name = ?1 AND shippingAddress = ?2")
        .expect("expected to be able to select from Customers table");
    let mut rows = stmt
        .query_map([&name, &address], |row| row.get(0))
        .expect("expected to be able to get id from Customers table");
    
    rows.next().unwrap().unwrap()
}

pub fn get_customer_address(cid: i64) -> String {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT shippingAddress FROM customers WHERE id = ?1")
        .expect("expected to be able to select from Customers table");
    let mut rows = stmt
        .query_map([&cid], |row| row.get(0))
        .expect("expected to be able to get shippingAddress from Customers table");
    
    rows.next().unwrap().unwrap()
}

pub fn update_customer_address(cid: i64, address: String) {
    let db = connect();
    db.execute(
        "UPDATE customers SET shippingAddress = ?1 WHERE id = ?2",
        [&address, &cid.to_string()],
    )
    .expect("expected to be able to update Customers table");
}

pub fn customer_balance(cid: i64) -> f64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT accountBalance FROM customers WHERE id = ?1")
        .expect("expected to be able to select from Customers table");
    let mut rows = stmt
        .query_map([&cid], |row| row.get(0))
        .expect("expected to be able to get accountBalance from Customers table");
    
    rows.next().unwrap().unwrap()
}
