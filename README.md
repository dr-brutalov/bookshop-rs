# bookshop-rs

A simple book store API in need of input validation/sanitization.

This is a part of the University of Wyoming's Secure Software Design Course (Spring 2023). This is the base repository to be forked and updated for various assignments. Alternative language versions are available in:

- [Go](https://github.com/andey-robins/bookshop-go)
- [Javascript](https://github.com/andey-robins/bookshop-js)

## Versioning

`bookshop-rs` is built with:

- cargo 1.70.0-nightly (15d090969 2023-03-21)
- rust edition 2021

## Usage

Start the api using `cargo run`

I recommend using [`httpie`](https://httpie.io) for testing of HTTP endpoints on the terminal. Tutorials are available elsewhere online, and you're free to use whatever tools you deem appropriate for testing your code.

## Initial Security Review

Examining the initial code base I identified the issues outlined below. Please note that since it has already been requested to address input validation, I will no address it in this analysis. This invites misuse of the API and creates a situation where it is very difficult to determine who has taken actions, and even what those actions entailed.

* Many of the functions that interface with the database through SQL lack proper sanitization. For example, [create_book](/src/handlers/books.rs) allows users to input unverified data directly into the database. A lack of input sanitization leaves us open to potential SQL injection attacks. While only one function is referenced here, many of the functions that interact with the underlying database share this risk. These functions will be updated to reflect best practices, including basic sanitization.

* There is the potential for a cross-site scripting attack (XSS) within [get_status](/src/handlers/orders.rs). The HTML served by this function simply accepts raw strings from the user. This will be be reorganized to return JSON rather than `RawHTML`. JSON takes advantage of built-in protections, allowing us to avoid the XSS vulnerability. However, this function is actually a duplicate of another function that is simply returning the shipping status of an order. To ensure no functionally will be lost, the customer's address will be added as a field of the `order Struct`. With this small change, the functionality is identical, `get_order_shipping_status` will be preserved. `get_status` and the `orders/get_status` endpoint will be removed.

* While addressing authentication is beyond the scope of this contract, it is worth noting that both [customers](src/handlers/customers.rs) and [orders](src/handlers/orders.rs) contain functionality that, in its current state, would allow anyone with access to this API to edit customer data and orders. This should be addressed in a future update.

* It is currently possible to create duplicate copies of existing books. This could lead to miscommunication concerning pricing of the duplicated book(s). This duplication should be addressed in a future update.

* There is no current logging implemented in this API. In particular, there is no method to verify when actions are taken. This includes no logging of updates to inventory, user activity, or orders. Logging will be added in this update.

* Error handling throughout is inconsistent. The most egregious example being the chained `unwrap()` methods. Error handling will be addressed in this update.

* The functions [get_balance](src/handlers/customers.rs), [get_price](src/handlers/books.rs), and [get_shipped](src/handlers/orders.rs) are not very descriptive of their intended usage. Without exploring the underlying code base, one wouldn't easily discern that `get_balance` returns the customers balance. The three functions listed above are not the only instances of this issue. Each of these functions will be re-named in this update.
