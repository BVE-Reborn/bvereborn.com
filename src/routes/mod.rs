use crate::helpers::Headers;
pub use download::*;
pub use file_validator::*;
use maplit::hashmap;
use rocket::get;
use rocket_contrib::templates::Template;

mod download;
mod file_validator;

#[get("/")]
pub fn index() -> Headers<Template> {
    Headers::public(Template::render("main", &()))
}

#[get("/community")]
pub fn community() -> Headers<Template> {
    Headers::public(Template::render("community", &()))
}

fn coming_soon(active: &str, name: &str) -> Headers<Template> {
    let ctx = hashmap! {
        "active" => active,
        "page" => name
    };
    Headers::public(Template::render("soon", &ctx))
}

#[get("/download/sample-route")]
pub fn sample_route() -> Headers<Template> {
    coming_soon("download", "the sample route")
}

#[get("/install")]
pub fn install() -> Headers<Template> {
    coming_soon("install", "the installation instructions")
}

#[get("/addons")]
pub fn addons() -> Headers<Template> {
    coming_soon("addons", "the addons page")
}

#[get("/patreon")]
pub fn patreon() -> Headers<Template> {
    coming_soon("patreon", "our Patreon")
}
