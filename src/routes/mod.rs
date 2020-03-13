use crate::helpers::Headers;
pub use download::*;
use rocket::get;
use rocket_contrib::templates::Template;

mod download;

#[get("/")]
pub fn index() -> Headers<Template> {
    Headers::public(Template::render("main", &()))
}
