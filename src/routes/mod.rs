use crate::headers::Headers;
use rocket::get;
use rocket_contrib::templates::Template;

#[get("/")]
pub fn index() -> Headers<Template> {
    Headers::public(Template::render("main", &()))
}
