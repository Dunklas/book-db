use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};
use crate::schema::books;

#[derive(Serialize, Debug, Queryable)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "books"]
pub struct NewBook {
    pub title: String,
    pub author: String,
}
