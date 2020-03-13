use crate::helpers::Headers;
pub use download::*;
use maplit::hashmap;
use rocket::get;
use rocket_contrib::templates::Template;

mod download;

#[get("/")]
pub fn index() -> Headers<Template> {
    Headers::public(Template::render("main", &()))
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

#[get("/support")]
pub fn support() -> Headers<Template> {
    coming_soon("support", "our Patreon")
}
