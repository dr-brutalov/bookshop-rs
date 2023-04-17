use log::{error, info};

use super::database::connect;

pub fn create_purchase_order(cid: i64, bid: i64) -> i64 {
    let db = connect();

    let query = "INSERT INTO PurchaseOrders (customerId, bookId, shipped) VALUES (:cid, :bid, 0);";
    let mut stmt = db
        .prepare(query)
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while preparing PO with cid \"{}\" and bid \"{}\": {}", cid, bid, error);
            panic!("Error while preparing PO with cid \"{}\" and bid \"{}\": {}", cid, bid, error);
        });

    stmt.execute(&[(":cid", &cid), (":bid", &bid)])
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while creating PO: {}", error);
            panic!("Error while creating PO: {}", error);
        });
    // db
    // .prepare("")
    // .expect("expected to be able to select from PurchaseOrders table")
    // .execute(&[(":cid", &cid), (":bid", &bid)])
    // .expect("expected to be able to insert into PurchaseOrders table");

    info!(target: "info", "Created PO with cid \"{}\" and bid: \"{}\".", cid, bid);

    get_purchase_order_id(cid, bid)
}

pub fn get_purchase_order_id(cid: i64, bid: i64) -> i64 {
    let db = connect();

    let query = "SELECT id FROM PurchaseOrders WHERE customerId = :cid AND bookId = :bid;";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing PO id: {}", error);
        panic!("Error while preparing PO id: {}", error);
    });
    let mut rows = stmt
        .query_map(&[(":cid", &cid), (":bid", &bid)], |row| row.get(0))
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while fetching PO id: {}", error);
            panic!("Error while fetching PO id: {}", error);
        });

    let id = rows
        .next()
        .unwrap_or_else(|| {
            error!(target: "error", "Failed to handle next PO in database");
            panic!("Failed to handle next PO in database");
        })
        .unwrap_or_else(|error| {
            error!(target: "error", "Failed to process PO rows: {}", error);
            panic!("Failed to process PO rows: {}", error);
        });

    info!(target: "info", "Successfully found PO id \"{}\".", id);

    id
}

pub fn get_purchase_order_shipping_status(poid: i64) -> i64 {
    let db = connect();

    let query = "SELECT shipped FROM PurchaseOrders WHERE id = :poid;";
    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing PO shipping status id: {}", error);
        panic!("Error while preparing PO shipping status id: {}", error);
    });
    let mut rows = stmt
        .query_map(&[(":poid", &poid)], |row| row.get(0))
        .unwrap_or_else(|error| {
            error!(target: "error", "Error while checking order shipping status: {}", error);
            panic!("Error while checking order shipping status: {}", error);
        });
    let shipped: i64 = rows
        .next()
        .unwrap_or_else(|| {
            error!(target: "error", "Failed to handle next purchase order in database.");
            panic!("Failed to handle next purchase order in database.");
        })
        .unwrap_or_else(|error| {
            error!(target: "error", "Failed to process purchase order rows: {}", error);
            panic!("Failed to process purchase order rows: {}", error);
        });

    info!(target: "info", "Shipping status of PO with id \"{}\" was checked.", poid);

    shipped
}

pub fn update_purchase_order_shipping_status_to_shipped(poid: i64) {
    let db = connect();

    let query = "UPDATE PurchaseOrders SET shipped = 1 WHERE id = :poid;";

    let mut stmt = db.prepare(query).unwrap_or_else(|error| {
        error!(target: "error", "Error while preparing to update PO id \"{}\": {}", poid, error);
        panic!(
            "Error while preparing to update PO id \"{}\": {}",
            poid, error
        );
    });

    stmt.execute(&[(":poid", &poid)]).unwrap_or_else(|error| {
        error!(target: "error", "Error while updating PO id \"{}\" shipping status: {}", poid, error);
        panic!("Error while updating PO id \"{}\" shipping status: {}", poid, error);
    });

    info!(target: "info", "PO with id \"{}\" has shipped!", poid);
}
