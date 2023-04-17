# Example Command Run

The sequence of commands below was used to generate example logging and to verify that the application is functioning as intended. 

```bash
echo '{"title": "Dune", "author": "Frank Herbert"}' | http GET localhost:8080/books/get_book_price

echo '{"title": "The Day the Rust Stood Still", "author": "Brutalov", "price": 19.98}' | http POST localhost:8080/books/create_new_book

echo '{"name": "Brutalov Yo", "shipping_address":"123 Lane" }' | http POST localhost:8080/customers/create_new_customer

echo '{"id": 1, "name": "Brutalov Yo", "shipping_address": "1234 Lane"}' | http PUT localhost:8080/customers/update_customer_address

echo '{"book_id": 3, "customer_id": 1}' | http GET localhost:8080/orders/get_order_shipping_status

echo '{"book_id": 4, "customer_id": 1}' | http GET localhost:8080/orders/get_order_shipping_status

echo '{"book_id": 1, "customer_id": 1}' | http POST localhost:8080/orders/create_new_order

echo '{"book_id": 1, "customer_id": 1}' | http GET localhost:8080/orders/get_order_shipping_status

echo '{"id": 3}' | http PUT localhost:8080/orders/ship_order

echo '{"book_id": 1, "customer_id": 1}' | http GET localhost:8080/orders/get_order_shipping_status
```
# Input Validation Commands

Similar to the fist set of commands, each command given below was used to evaluate the effectiveness of the implemented input validation. While not exhaustive, it showcases some common mistakes and possible systemic exploration.

```bash
echo '{"title": "Dune", "author": "Frank Herbert"}' | http GET localhost:8080/books/get_book_price

echo '{"title": "Dune", "author": ""}' | http GET localhost:8080/books/get_book_price

echo '{"title": "Dune", "author": "       "}' | http GET localhost:8080/books/get_book_price

echo '{"title": "Dune", "author": "\<script>"}' | http GET localhost:8080/books/get_book_price

echo '{"title": "", "author": "Frank Herbert"}' | http GET localhost:8080/books/get_book_price

echo '{"title": "        ", "author": "Frank Herbert"}' | http GET localhost:8080/books/get_book_price

echo '{"title": "<script>", "author": "Frank Herbert"}' | http GET localhost:8080/books/get_book_price

echo '{"title": "The Day the Rust Stood Still", "author": "Brutalov", "price": 19.981}' | http POST localhost:8080/books/create_new_book

echo '{"title": "The Day the Rust Stood Still Two Electric Boogaloo", "author": "Brutalov", "price": 111119.98}' | http POST localhost:8080/books/create_new_book

echo '{"name": "Brutalov Yo", "shipping_address":"123 Lane" }' | http POST localhost:8080/customers/create_new_customer

echo '{"name": "Brutalov Yo", "shipping_address":"Robert; DROP TABLE Students;--" }' | http POST localhost:8080/customers/create_new_customer

echo '{"id": 1, "name": "Brutalov Yo", "shipping_address": ""}' | http PUT localhost:8080/customers/update_customer_address
```