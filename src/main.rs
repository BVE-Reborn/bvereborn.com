#![feature(proc_macro_hygiene, decl_macro)]

use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/static", StaticFiles::from("static")).mount("/", routes![index]).launch();
}
