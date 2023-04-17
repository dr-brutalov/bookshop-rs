CREATE TABLE Books (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    price REAL NOT NULL
);

CREATE TABLE Customers (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    shipping_address TEXT NOT NULL,
    account_balance REAL NOT NULL
);

-- SQLITE has no boolean type, 1 is true, 0 false
CREATE TABLE PurchaseOrders (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    bookId INTEGER NOT NULL REFERENCES Books(id),
    customerId INTEGER NOT NULL REFERENCES Customers(id),
    shipped INTEGER NOT NULL
);

INSERT INTO Books (title, author, price) VALUES ('The Hitchhikers Guide to the Galaxy', 'Douglas Adams', 12.99);
INSERT INTO Books (title, author, price) VALUES ('Dune', 'Frank Herbert', 9.99);
INSERT INTO Books (title, author, price) VALUES ('The Left Hand of Darkness', 'Ursula K. Le Guin', 8.99);
INSERT INTO Books (title, author, price) VALUES ('Foundation', 'Isaac Asimov', 7.99);
INSERT INTO Books (title, author, price) VALUES ('The Player of Games', 'Iain M. Banks', 6.99);

INSERT INTO Customers (name, shipping_address, account_balance) VALUES ('Brutalov Yo', '123 Lane', 5.0);

INSERT INTO PurchaseOrders (bookId, customerId, shipped) VALUES (3, 1, 0);
INSERT INTO PurchaseOrders (bookId, customerId, shipped) VALUES (4, 1, 1);