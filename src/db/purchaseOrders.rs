use super::db::connect;

pub fn create_purchase_order(cid: i64, bid: i64) -> i64 {
    let db = connect();
    db.execute(
        "INSERT INTO PurchaseOrders (customerId, bookId, shipped) VALUES (?1, ?2, 0)",
        &[&cid, &bid],
    )
    .expect("expected to be able to insert into PurchaseOrders table");

    return get_purchase_order_id(cid, bid);
}

pub fn get_purchase_order_id(cid: i64, bid: i64) -> i64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT id FROM PurchaseOrders WHERE customerId = ?1 AND bookId = ?2")
        .expect("expected to be able to select from PurchaseOrders table");
    let mut rows = stmt
        .query_map(&[&cid, &bid], |row| row.get(0))
        .expect("expected to be able to get id from PurchaseOrders table");
    let id = rows.next().unwrap().unwrap();
    return id;
}

pub fn is_po_shipped(poid: i64) -> i64 {
    let db = connect();
    let mut stmt = db
        .prepare("SELECT shipped FROM PurchaseOrders WHERE id = ?1")
        .expect("expected to be able to select from PurchaseOrders table");
    let mut rows = stmt
        .query_map(&[&poid], |row| row.get(0))
        .expect("expected to be able to get shipped from PurchaseOrders table");
    let shipped: i64 = rows.next().unwrap().unwrap();
    return shipped;
}

pub fn ship_po(poid: i64) {
    let db = connect();
    db.execute(
        "UPDATE PurchaseOrders SET shipped = 1 WHERE id = ?1",
        &[&poid],
    )
    .expect("expected to be able to update PurchaseOrders table");
}
