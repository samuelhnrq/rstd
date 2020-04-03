use chrono::NaiveDateTime;
use diesel::Queryable;

#[derive(Debug, PartialEq, Queryable)]
pub struct Todo {
    id: i32,
    title: String,
    completed: bool,
    created_at: NaiveDateTime
}

