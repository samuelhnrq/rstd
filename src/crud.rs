use super::model::NewTodo;
use super::schema::todo;
use clap::ArgMatches;
use diesel::{insert_into, SqliteConnection};
use diesel::prelude::*;
use log::info;

pub fn create_task(create_match: &ArgMatches, conn: &SqliteConnection) {
    info!("Creating {:?}", create_match);
    let todo = NewTodo::new(create_match.value_of("description").unwrap());
    // Diesel is great, but really finicky and magical. Error messages are huge
    // and confusing, spanning 20 lines when just 2 matter.
    // Again, builder pattern, self documenting.
    insert_into(todo::table)
        .values(&todo)
        .execute(conn)
        .expect("error while saving todo");
}
