use log::{info, error};

use super::database::connect;

pub fn create_customer(name: String, address: String) -> i64 {
    let db = connect();

    let query = "INSERT INTO customers (name, shippingAddress, accountBalance) VALUES (:name, :ShippingAddress, 5.0);";
    let mut stmt = db.prepare(query).unwrap_or_else(|error|{
        error!(target: "error", "Error while preparing to create customer id: {}", error);
        panic!("Failed, yo {}", error)
    });
    stmt.execute(&[(":name", &name), (":ShippingAddress", &address)]).unwrap_or_else(|error| {
        error!(target: "error", "Error while creating customer: {}", error);
        panic!("Failed, yo {}", error)
    });
    info!(target: "info", "Created user: {} with address: {}.", name, address);

    get_customer_id(name, address)
}

pub fn get_customer_id(name: String, address: String) -> i64 {
    let db = connect();

    let query = "SELECT id FROM customers WHERE name = :name AND shippingAddress = :address;";
    let mut stmt = db.prepare(query).unwrap_or_else(|error|{
        error!(target: "error", "Error while preparing customer id: {}", error);
        panic!("Failed, yo {}", error)
    });

    let mut rows = stmt
        .query_map(&[(":name", &name), (":address", &address)], |row| row.get(0))
        .unwrap_or_else(|error|{
            error!(target: "error", "Error while creating customer: {}", error);
            panic!("Failed, yo {}", error)
        });

    let id = rows.next().unwrap_or_else(||{
        error!(target: "error", "Failed to handle next customer in database");
        panic!("Failed, yo.")
    }).unwrap_or_else(|error| {
        error!(target: "error", "Failed to process customer rows: {}", error);
        panic!("Failed, yo {}", error)
    });

    info!(target: "info", "Created user: {} with address: {}.", name, address);

    return id
}

pub fn get_customer_address(cid: i64) -> String {
    let db = connect();

    let query = "SELECT shippingAddress FROM customers WHERE id = :cid;";
    let mut stmt = db.prepare(query).unwrap_or_else(|error|{
        error!(target: "error", "Error while preparing customer id: {}", error);
        panic!("Failed, yo {}", error)
    });

    let mut rows = stmt
    .query_map(&[(":cid", &cid)], |row| row.get(0))
        .unwrap_or_else(|error|{
            error!(target: "error", "Error while creating customer: {}", error);
            panic!("Failed, yo {}", error)
        });

    let address = rows.next().unwrap_or_else(||{
        error!(target: "error", "Failed to handle next customer in database");
        panic!("Failed, yo.")
    }).unwrap_or_else(|error| {
        error!(target: "error", "Failed to process customer rows: {}", error);
        panic!("Failed, yo {}", error)
    });

    info!(target: "info", "Created user with cid: {}.", cid);

    return address

    // let mut stmt = db
    //     .prepare("")
    //     .expect("expected to be able to select from Customers table");
    // let mut rows = stmt
    //     .query_map(&[(":cid", &cid)], |row| row.get(0))
    //     .expect("expected to be able to get shippingAddress from Customers table");
    
    // rows.next().unwrap().unwrap()
}

pub fn update_customer_address(cid: i64, address: String) {
    let db = connect();

    let query = "UPDATE customers SET shippingAddress = :address WHERE id = :cid;";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing to updating customer id {} address to {}: {}", cid, address, error);
        panic!("Failed to prepare address update: {}", error)
    });
    stmt.execute(&[(":address", &address), (":cid", &cid.to_string())]).unwrap_or_else(|error| {
        error!(target: "error", "Error while updating customer id {} address to {}: {}", cid, address, error);
        panic!("Failed to update address: {}", error)
    });
    info!(target: "info", "Customer {} updated their address: {}.", cid, address);

}

pub fn customer_balance(cid: i64) -> f64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT accountBalance FROM customers WHERE id = :cid")
        .expect("expected to be able to select from Customers table");
    let mut rows = stmt
        .query_map(&[(":cid", &cid)], |row| row.get(0))
        .expect("expected to be able to get accountBalance from Customers table");
    
    let balance = rows.next().unwrap_or_else(|| {
        error!(target: "error", "Failed to handle next customer in database");
        panic!("Failed, yo.")
    }).unwrap_or_else(|error| {
        error!(target: "error", "Failed to process customer rows: {}", error);
        panic!("Failed, yo {}", error)
    });

    info!(target: "info", "Customer {} balance was checked.", cid);

    return balance
}
