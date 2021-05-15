#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

use rocket_contrib::json::Json;

mod book;
use book::Book;

#[post("/", format = "application/json", data = "<book>")]
fn create_book(book: Json<Book>) -> Json<Book> {
    let book = Book{
        title: "Call of Cthulu".to_owned(),
        author: "H.P. Lovecraft".to_owned(),
        comment: "Skitbra!!".to_owned()
    };
    Json(book)
}

#[get("/", format = "application/json")]
fn get_books() -> Json<Vec<Book>> {
    Json(Vec::new())
}

fn main() {
    rocket::ignite().mount("/books", routes![
        create_book,
        get_books,
    ]).launch();
}
