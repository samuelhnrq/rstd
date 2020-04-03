use config::ConfigFile;
use diesel::prelude::*;
use log::info;
use model::Todo;
use schema::todo::dsl::*;

#[macro_use]
extern crate diesel;

mod arguments;
mod config;
mod database;
mod logger;
mod model;
mod schema;

fn main() -> std::io::Result<()> {
    let args = arguments::initialize_args();
    logger::initialize(args);
    let conn = database::establish_connection();
    let todos = todo.limit(4).load::<Todo>(&conn).unwrap();
    let c = ConfigFile::initialize();
    info!("Config e log inicializado! {:?}", c);
    info!("Lista de todos {:?}", todos);
    Ok(())
}
