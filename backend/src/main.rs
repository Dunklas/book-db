#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

mod book;
use book::{NewBook, Book};
mod schema;

use rocket::Rocket;
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
fn create_book(book: Json<NewBook>, db: BookDb) -> Json<Book> {
    Json(insert_book(&*db, &book.into_inner()))
}

#[get("/", format = "application/json")]
fn get_books(db: BookDb) -> Json<Vec<Book>> {
    let books = load_from_db(&*db);
    Json(books)
}

fn run_db_migrations(rocket: Rocket) -> Result<Rocket, Rocket> {
    embed_migrations!();
    let conn = BookDb::get_one(&rocket)
        .expect("Database connection");
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(rocket),
        Err(_e) => {
            Err(rocket)
        }
    }
}

fn main() {
    rocket::ignite()
        .attach(BookDb::fairing())
        .attach(AdHoc::on_attach("Database migrations", run_db_migrations))
        .mount("/books", routes![
            create_book,
            get_books,
        ])
        .launch();
}
