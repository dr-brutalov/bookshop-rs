use super::db::connect;

pub fn create_purchase_order(cid: i64, bid: i64) -> i64 {
    let db = connect();
    db
    .prepare("INSERT INTO PurchaseOrders (customerId, bookId, shipped) VALUES (:cid, :bid, 0)")
    .expect("expected to be able to select from PurchaseOrders table")
    .execute(&[(":cid", &cid), (":bid", &bid)])
    .expect("expected to be able to insert into PurchaseOrders table");

    get_purchase_order_id(cid, bid)
}

pub fn get_purchase_order_id(cid: i64, bid: i64) -> i64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT id FROM PurchaseOrders WHERE customerId = :cid AND bookId = :bid")
        .expect("expected to be able to select from PurchaseOrders table");
    let mut rows = stmt
        .query_map(&[(":cid", &cid), (":bid", &bid)], |row| row.get(0))
        .expect("expected to be able to get id from PurchaseOrders table");
    
    rows.next().unwrap().unwrap()
}

pub fn is_po_shipped(poid: i64) -> i64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT shipped FROM PurchaseOrders WHERE id = :poid")
        .expect("expected to be able to select from PurchaseOrders table");
    let mut rows = stmt
        .query_map(&[(":poid", &poid)], |row| row.get(0))
        .expect("expected to be able to get shipped from PurchaseOrders table");
    let shipped: i64 = rows.next().unwrap().unwrap();
    shipped
}

pub fn ship_po(poid: i64) {
    let db = connect();
    db
    .prepare("UPDATE PurchaseOrders SET shipped = 1 WHERE id = :poid")
    .expect("expected to be able to update to PurchaseOrders table")
    .execute(&[(":poid", &poid)])
    .expect("expected to be able to update PurchaseOrders table");
}
