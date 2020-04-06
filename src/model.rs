use super::schema::todo;
use chrono::{Local, NaiveDateTime};
use diesel::Queryable;

#[derive(Debug, PartialEq, Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "todo"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub completed: bool,
    pub created_at: NaiveDateTime,
}

// Bruh lifetimes are great conceptually but confusing to use
// apparrently I can't have a lifetime called 'a here too because
// it's already defined, but I can't use it still because it's not
// in scope
impl<'b> NewTodo<'b> {
    /// Creates a new todo not completed and now
    pub fn new(description: &'b str) -> Self {
        NewTodo {
            title: description,
            completed: false,
            created_at: Local::now().naive_local(),
        }
    }
}
