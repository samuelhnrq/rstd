use config::ConfigFile;
use diesel::{prelude::*, SqliteConnection};
use diesel_migrations::{find_migrations_directory, run_pending_migrations_in_directory};
use log::{error, info, trace, debug};
use model::Todo;
use schema::todo::dsl::*;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod arguments;
mod config;
mod crud;
mod database;
mod logger;
mod model;
mod schema;

// Because SQLite, what's the fun in using sqlite other than 
// having completly and easily bootstrap-able databases?
embed_migrations!();

fn initialize_db(c: &ConfigFile) -> SqliteConnection {
    let conn = database::establish_connection(&c.storage_location);
    let migrations_loc = find_migrations_directory().expect("Can't find migrations");
    let mut void = std::io::sink();
    match run_pending_migrations_in_directory(&conn, &migrations_loc, &mut void) {
        Err(e) => error!("Error running database migrations!!! {}", e),
        Ok(_) => trace!("Migrations ran successfully."),
    };
    conn
}

fn main() -> std::io::Result<()> {
    let args = arguments::initialize_args();
    trace!("Arguments parsed: {:?}", args);
    logger::initialize(&args);
    let c = ConfigFile::initialize();
    debug!("Config e logging intialized! {:?}", c);
    let conn = initialize_db(&c);
    // This will eventually get huge.
    match args.subcommand() {
        ("create", Some(create)) => crud::create_task(&create, &conn),
        _ => {
            let todos = todo.limit(4).load::<Todo>(&conn).unwrap();
            info!("Lista de todos {:?}", todos);
        }
    }
    Ok(())
}
