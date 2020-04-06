use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use std::path::PathBuf;

/// Estabilishes an SQLITE connection, very complex, I know.
pub fn establish_connection(storage_location: &PathBuf) -> SqliteConnection {
    // 2 lazy 2 care. Joking, needs proper handling, e.g. username with special
    // characters. You know that one guy with the المستعمل.
    let str_location = &storage_location.to_string_lossy();
    SqliteConnection::establish(str_location)
        .expect(&format!("Error connecting to {}", str_location))
}
