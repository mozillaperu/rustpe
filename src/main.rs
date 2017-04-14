#![feature(plugin)]
#![plugin(rocket_codegen)]

use std::io;
use std::path::{Path, PathBuf};

extern crate rocket_contrib;
extern crate rocket;

use rocket_contrib::Template;
use rocket::response::NamedFile;

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[get("/test")]
fn test() -> &'static str {
    "Hello, World!"
}

#[get("/")]
fn index() -> Template {
    let context = ();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite().mount("/", routes![files, index, test]).launch();
}
