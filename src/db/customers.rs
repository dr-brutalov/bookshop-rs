use super::db::connect;

pub fn create_customer(name: String, address: String) -> i64 {
    let db = connect();
    db
    .prepare("INSERT INTO customers (name, shippingAddress, accountBalance) VALUES (:name, :ShippingAddress, 5.0)")
    .expect("expected to be able to select from Customers table")
    .execute(&[(":name", &name), (":ShippingAddress", &address)])
    .expect("expected to be able to insert into Customers table");

    get_customer_id(name, address)
}

pub fn get_customer_id(name: String, address: String) -> i64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT id FROM customers WHERE name = :name AND shippingAddress = :address")
        .expect("expected to be able to select from Customers table");
    let mut rows = stmt
        .query_map(&[(":name", &name), (":address", &address)], |row| row.get(0))
        .expect("expected to be able to get id from Customers table");
    
    rows.next().unwrap().unwrap()
}

pub fn get_customer_address(cid: i64) -> String {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT shippingAddress FROM customers WHERE id = :cid")
        .expect("expected to be able to select from Customers table");
    let mut rows = stmt
        .query_map(&[(":cid", &cid)], |row| row.get(0))
        .expect("expected to be able to get shippingAddress from Customers table");
    
    rows.next().unwrap().unwrap()
}

pub fn update_customer_address(cid: i64, address: String) {
    let db = connect();
    db
    .prepare("UPDATE customers SET shippingAddress = :address WHERE id = :cid")
    .expect("expected to be able to update to shippingAddress")
    .execute(&[(":address", &address), (":cid", &cid.to_string())])
    .expect("expected to be able to update Customers table");
}

pub fn customer_balance(cid: i64) -> f64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT accountBalance FROM customers WHERE id = :cid")
        .expect("expected to be able to select from Customers table");
    let mut rows = stmt
        .query_map(&[(":cid", &cid)], |row| row.get(0))
        .expect("expected to be able to get accountBalance from Customers table");
    
    rows.next().unwrap().unwrap()
}
