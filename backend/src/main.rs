#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel_migrations;
#[macro_use] extern crate diesel;

mod book;
use book::{NewBook, Book};
mod schema;

use rocket::{Rocket, Build};
use rocket::fairing::AdHoc;
use rocket::http::Header;
use rocket_contrib::json::Json;
use rocket_contrib::databases::{database, diesel as rocket_contrib_diesel};
use rocket_contrib_diesel::RunQueryDsl;
use diesel::prelude::*;
use schema::books::dsl::*;

#[database("book_db")]
struct BookDb(rocket_contrib_diesel::PgConnection);

#[post("/", format = "application/json", data = "<book>")]
async fn create_book(book: Json<NewBook>, db: BookDb) -> Json<Book> {
    db.run(|connection| {
        let result = diesel::insert_into(books)
            .values(book.into_inner())
            .get_result::<Book>(connection)
            .expect("Error saving new book");
        Json(result)
    }).await
}

#[get("/", format = "application/json")]
async fn get_books(db: BookDb) -> Json<Vec<Book>> {
    db.run(|connection| {
        Json(books.load::<Book>(connection)
            .expect("Error reading books"))
    }).await
}

#[delete("/<book_id>")]
async fn delete_book(book_id: i32, db: BookDb) {
    db.run(move |connection| {
        diesel::delete(books.filter(id.eq(book_id)))
            .execute(connection)
            .expect("Error deleting book");
    }).await;
}

async fn run_db_migrations(rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
    embed_migrations!();
    let db = BookDb::get_one(&rocket).await
        .expect("Database connection");
    db.run(|c| {
        match embedded_migrations::run(c) {
            Ok(()) => Ok(rocket),
            Err(_e) => {
                Err(rocket)
            }
        }
    }).await
}

#[rocket::main]
async fn main() {
    rocket::build()
        .attach(BookDb::fairing())
        .attach(AdHoc::try_on_ignite("Database migrations", run_db_migrations))
        .attach(AdHoc::on_response("CORS Headers", |_request, response| {
            Box::pin(async move {
                response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
                response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, DELETE, OPTIONS"));
                response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
            })
        }))
        .mount("/books", routes![
            create_book,
            get_books,
            delete_book,
        ])
        .launch()
        .await
        .unwrap();
}
