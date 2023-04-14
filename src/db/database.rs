use log::{info, warn, error};
use rusqlite::{Connection};
use core::panic;
use std::{fs, path::Path};

pub fn connect() -> Connection {
    let mut must_initialize_db = false;
    if !Path::new("dd.db").exists() {
        must_initialize_db = true;
    }

    let connection = Connection::open("dd.db").unwrap();
    
    if must_initialize_db {
        let query = fs::read_to_string("init.sql").expect("initial schema does not exist");
        let commands = query.split(";\n");

        for command in commands {
            connection.execute(command, ()).unwrap_or_else(|error| {
                if error.to_string() == "not an error" {
                    warn!(target: "warn", "Still not an error.");
                } else {
                    error!("Database initialization is broken, check the init schema. Error: {}", error);
                    panic!("Database initialization is broken: {}", error);
                }
                0
            });
        }
        info!(target: "info", "Database initialized. Not the first logged element? Contact IT!")
    }
    
    connection
}