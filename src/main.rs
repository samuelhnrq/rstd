use config::ConfigFile;
use diesel::{SqliteConnection, prelude::*};
use diesel_migrations::{run_pending_migrations_in_directory, find_migrations_directory};
use log::{info, error, trace};
use model::Todo;
use schema::todo::dsl::*;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod arguments;
mod config;
mod database;
mod logger;
mod model;
mod schema;

embed_migrations!();

fn initialize_db(c: &ConfigFile) -> SqliteConnection {
    let conn = database::establish_connection(&c.storage_location);
    let migrations_loc = find_migrations_directory().expect("Can't find migrations");
    let mut void = std::io::sink();
    match run_pending_migrations_in_directory(&conn, &migrations_loc, &mut void) {
        Err(e) => error!("Error running database migrations!!! {}", e),
        Ok(_) => trace!("Migrations ran successfully.")
    };
    conn
}

fn main() -> std::io::Result<()> {
    let args = arguments::initialize_args();
    logger::initialize(args);
    let c = ConfigFile::initialize();
    let conn = initialize_db(&c);
    let todos = todo.limit(4).load::<Todo>(&conn).unwrap();
    info!("Config e log inicializado! {:?}", c);
    info!("Lista de todos {:?}", todos);
    Ok(())
}
