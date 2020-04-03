use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::path::PathBuf;

pub fn establish_connection(storage_location: &PathBuf) -> SqliteConnection {
    let str_location = &storage_location.to_string_lossy();
    SqliteConnection::establish(str_location)
        .expect(&format!("Error connecting to {}", str_location))
}
