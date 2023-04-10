#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::path::Path;
use rocket::response::NamedFile;
//use rocket::http::uri::Path;
use rocket_contrib::{serve::StaticFiles,};

#[get("/")]
fn index() -> NamedFile {
    NamedFile::open(Path::new("static/index.html")).unwrap()
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, inside world!"
}

#[get("/world")]              // <- route attribute
fn world() -> &'static str {  // <- request handler
    "hello, outside world!"
}

// #[get("/balls")]
// fn static_file()  -> NamedFile {
//     NamedFile::open(Path::new("static/index.html")).unwrap()
// }

fn main() {
    rocket::ignite()
    .mount("/", routes![index, hello, world,])
    .mount("/", StaticFiles::from("static"))
    .launch();
}