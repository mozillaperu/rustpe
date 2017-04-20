#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket_contrib;
extern crate rocket;
extern crate mysql;
extern crate chrono;
extern crate serde_json;

#[macro_use] extern crate serde_derive;

use std::path::{Path, PathBuf};
use rocket_contrib::Template;
use rocket_contrib::{JSON};
use rocket::response::NamedFile;

mod helper;
mod model;
use model::event::Event;


#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

#[error(404)]
fn not_found() -> Template {
    let context = ();
    Template::render("404", &context)
}

#[get("/events", format = "application/json")]
fn events() -> JSON<Vec<Event>> {
    let pool = helper::DB::connection();
    let selected_events = model::Event::all(pool);
    println!("Events {:?}", selected_events);
    JSON(selected_events)
}

#[get("/")]
fn index() -> Template {
    let context = ();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
    .mount("/", routes![files, index, events])
    .catch(errors![not_found])
    .launch();
}
