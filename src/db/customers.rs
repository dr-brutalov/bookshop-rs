use log::{error, info};

use super::database::connect;

pub fn create_customer(name: String, address: String) -> i64 {
    let db = connect();

    let query = "INSERT INTO customers (name, shipping_address, account_balance) VALUES (:name, :ShippingAddress, 5.0);";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing to create customer id: {}", error);
        panic!("Error while preparing to create customer id: {}", error);
    });
    stmt.execute(&[(":name", &name), (":ShippingAddress", &address)])
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while creating customer: {}", error);
            panic!("Error while creating customer: {}", error);
        });

    info!(target: "info", "Created user: \"{}\" with address: \"{}\".", name, address);

    get_customer_id(name, address)
}

pub fn get_customer_id(name: String, address: String) -> i64 {
    let db = connect();

    let query = "SELECT id FROM customers WHERE name = :name AND shipping_address = :address;";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing customer id: {}", error);
        panic!("Error while preparing customer id: {}", error);
    });

    let mut rows = stmt
        .query_map(&[(":name", &name), (":address", &address)], |row| {
            row.get(0)
        })
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while creating customer: {}", error);
            panic!("Error while creating customer: {}", error);
        });

    let id = rows
        .next()
        .unwrap_or_else(|| {
            error!(target: "error", "Failed to handle next customer in database (get_customer_id).");
            panic!("Failed to handle next customer in database (get_customer_id).");
        })
        .unwrap_or_else(|error| {
            error!(target: "error", "Failed to process customer rows: {}", error);
            panic!("Failed to process customer rows: {}", error);
        });

    info!(target: "info", "Customer with name \"{}\" and address \"{}\" has id {}.", name, address, id);

    id
}

pub fn get_customer_address(cid: i64) -> String {
    let db = connect();

    let query = "SELECT shipping_address FROM customers WHERE id = :cid;";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing customer id: {}", error);
        panic!("Error while preparing customer id: {}", error);
    });

    let mut rows = stmt
        .query_map(&[(":cid", &cid)], |row| row.get(0))
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while creating customer: {}", error);
            panic!("Error while creating customer: {}", error);
        });

    let address = rows
        .next()
        .unwrap_or_else(|| {
            error!(target: "error", "Failed to handle next customer in database (get_customer_id).");
            panic!("Failed to handle next customer in database (get_customer_id).");
        })
        .unwrap_or_else(|error| {
            error!(target: "error", "Failed to process customer rows: {}", error);
            panic!("Failed to process customer rows: {}", error);
        });

    info!(target: "info", "Created user with cid: \"{}\".", cid);

    address
}

pub fn update_customer_address(cid: i64, address: String) {
    let db = connect();

    let query = "UPDATE customers SET shipping_address = :address WHERE id = :cid;";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing to updating customer id {} address to {}: {}", cid, address, error);
        panic!("Error while preparing to updating customer id {} address to {}: {}", cid, address, error);
    });
    stmt.execute(&[(":address", &address), (":cid", &cid.to_string())]).unwrap_or_else(|error| {
        error!(target: "error", "Error while updating customer id {} address to {}: {}", cid, address, error);
        panic!("Error while updating customer id {} address to {}: {}", cid, address, error);
    });

    info!(target: "info", "Customer \"{}\" updated their address to: \"{}\".", cid, address);
}

pub fn customer_balance(cid: i64) -> f64 {
    let db = connect();

    let query = "SELECT account_balance FROM customers WHERE id = :cid;";
    let mut stmt = db
        .prepare(query)
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while preparing customer balance for cid \"{}\": {}", cid, error);
            panic!("Error while preparing customer balance for cid \"{}\": {}", cid, error);
        });
    let mut rows = stmt
        .query_map(&[(":cid", &cid)], |row| row.get(0))
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while requesting customer balance for cid \"{}\": {}", cid, error);
            panic!("Error while requesting customer balance for cid \"{}\": {}", cid, error);
        });

    let balance = rows
        .next()
        .unwrap_or_else(|| {
            error!(target: "error", "Failed to handle next customer in database (customer_balance).");
            panic!("Failed to handle next customer in database (customer_balance).")
        })
        .unwrap_or_else(|error| {
            error!(target: "error", "Failed to process customer rows: {}", error);
            panic!("Failed to process customer rows: {}", error)
        });

    info!(target: "info", "Customer \"{}\" has a balance of \"{}\".", cid, balance);

    balance
}
