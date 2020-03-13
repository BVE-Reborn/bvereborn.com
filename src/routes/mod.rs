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

fn coming_soon(name: &str) -> Headers<Template> {
    let ctx = hashmap! {
        "page" => name
    };
    Headers::public(Template::render("soon", &ctx))
}

#[get("/download/sample-route")]
pub fn sample_route() -> Headers<Template> {
    coming_soon("the sample route")
}

#[get("/install")]
pub fn install() -> Headers<Template> {
    coming_soon("the installation instructions")
}

#[get("/addons")]
pub fn addons() -> Headers<Template> {
    coming_soon("the addons page")
}
