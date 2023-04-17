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

* There is no current way of verifying if a customer has the balance needed to place an order. Further, there is no functionality to update a customer balance from the default value. The addition of this functionality is beyond the scope of this assignment and should be addressed in the future.

* The naming schema for accessing HTML endpoints was inconsistent. When launching the application, the information given was not indicative of the actual endpoint. For example, attempting to update a customer address, one might attempt access `/customers/update_address` based on what is presented in the start-up information. However, this isn't correct, we would use `/customers/updateAddress`. While this is outlined within the source code, the typical user won't have access to this information. This behavior will be updated for the whole application to be consistent with what is reported on start up.

## Overview of Changes

Fist and foremost, as requested initially, input validation has been added. For both book and customer metadata, only alphanumeric and basic punctuation ("," and ".") input is accepted. If this is too restrictive, select characters may be add allowed with a simple modification. As further checks, users cannot input an empty string or a string containing only spaces as input. In addition to these changes, any excessive spaces before or after any valid input is trimmed out. Excessive spaces between valid characters are replaced with a single space. For the numerical input associated with book prices, a valid price is required to fall within a range of 0.01 to 99,999.99 and must contain both decimals of precision. This is to ensure consistency within the input. It is worth noting that trailing zeros are automatically trimmed and will not create erroneous behavior. With all input validation, a message is printed to the user that clearly states what field was formatted incorrectly. Some sample commands can be found [here](/ex_commands.md).

Logging has been also added. There are three log levels, `info`, `warn`, and `error`:
* [INFO](/logs/info/info_log.txt) provides a robust logging of normal usage. The main usage of such logging is to track normal usage and enable for auditing of typical business tasks.
* [WARN](/logs/warn/warn_log.txt) provides logging for input validation and reports any incorrect inputs. This level of logging allows for easy audits of any invalid input. 
* [ERROR](/logs/error/error_log.txt) provides logging for any behavior that should crash the application. These logs should typically be empty. If errors begin to arise please contact IT.

Some quality of life improvements have been made. Endpoint naming is now consistent with what the application reports on start up. While this will require some adjustment from heavy-use users, it will in the long run reduce misunderstanding and make the application more accessible.

Formatting of database requests has been standardized. While no explicit vulnerabilities were found or exploited, correct interfacing with the database will further mitigate the risk of database manipulation. 

Finally, as part of the logging integration, error handling has been overhauled to be more idiomatic and useful. Error messages are now more informative and allow for faster troubleshooting should issues arise.

As a final note, some example log files have been generated. These logs showcase typical usage and some instances where input validation functions as outlined above.