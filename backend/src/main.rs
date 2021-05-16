#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;

use rocket::Rocket;
use rocket::fairing::AdHoc;
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel};

mod book;
use book::Book;

#[database("book_db")]
struct BookDb(diesel::PgConnection);

#[post("/", format = "application/json", data = "<book>")]
fn create_book(book: Json<Book>) -> Json<Book> {
    let book = Book{
        title: "Call of Cthulu".to_owned(),
        author: "H.P. Lovecraft".to_owned(),
    };
    Json(book)
}

#[get("/", format = "application/json")]
fn get_books() -> Json<Vec<Book>> {
    Json(Vec::new())
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
