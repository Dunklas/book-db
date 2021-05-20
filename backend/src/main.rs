#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

mod book;
use book::{NewBook, Book};
mod schema;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel as rocket_contrib_diesel};
use rocket_contrib_diesel::RunQueryDsl;

#[database("book_db")]
struct BookDb(rocket_contrib_diesel::PgConnection);

fn load_from_db(conn: &diesel::PgConnection) -> Vec<Book> {
    let books = schema::books::table.load::<Book>(conn)
        .expect("Error reading books");
    books
}

fn insert_book(conn: &diesel::PgConnection, book: &NewBook) -> Book {
    let book = diesel::insert_into(schema::books::table)
        .values(book)
        .get_result::<Book>(conn)
        .expect("Error saving new book");
    book
}

#[post("/", format = "application/json", data = "<book>")]
async fn create_book(book: Json<NewBook>, db: BookDb) -> Json<Book> {
    db.run(|connection| {
        Json(insert_book(&connection, &book.into_inner()))
    }).await
}

#[get("/", format = "application/json")]
async fn get_books(db: BookDb) -> Json<Vec<Book>> {
    db.run(|connection| {
        Json(load_from_db(&connection))
    }).await
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    embed_migrations!();
    let conn = BookDb::get_one(&rocket).await
        .expect("Database connection");
    conn.run(|c| {
        match embedded_migrations::run(c) {
            Ok(()) => rocket,
            Err(_e) => {
                rocket
            }
        }
    }).await
}

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(BookDb::fairing())
        .attach(AdHoc::on_ignite("Database migrations", run_db_migrations))
        .mount("/books", routes![
            create_book,
            get_books,
        ])
        .launch()
        .await;
}
