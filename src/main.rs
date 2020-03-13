#![feature(proc_macro_hygiene, decl_macro)]

use crate::headers::Headers;
use maplit::hashmap;
use rocket::fairing::Fairing;
use rocket::http::Status;
use rocket::response::{NamedFile, Redirect};
use rocket::{catch, catchers, get, routes, uri, Request};
use rocket_contrib::templates::handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use rocket_contrib::templates::Template;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

mod headers;

#[catch(403)]
fn error_403(req: &Request) -> Template {
    let ctx = hashmap! {
        "code" => String::from("403"),
        "text" => String::from("Forbidden"),
        "reason" => format!("You are not allowed to access {}!", req.uri().path())
    };
    Template::render("error", &ctx)
}

#[catch(404)]
fn error_404(req: &Request) -> Template {
    let ctx = hashmap! {
        "code" => String::from("404"),
        "text" => String::from("Not Found"),
        "reason" => format!("Cannot find the page {}!", req.uri().path())
    };
    Template::render("error", &ctx)
}

#[catch(500)]
fn error_500() -> Template {
    let ctx = hashmap! {
        "code" => String::from("500"),
        "text" => String::from("Internal Server Error"),
        "reason" => format!("The server errored when trying to make this page 😭")
    };
    Template::render("error", &ctx)
}

#[get("/403")]
fn error_create_403() -> Status {
    Status::Forbidden
}

#[get("/500")]
fn error_create_500() -> Status {
    Status::InternalServerError
}

#[get("/")]
fn index() -> Headers<Template> {
    // let map = hashmap! {};
    Headers::public(Template::render("main", &()))
}

#[get("/favicon.ico")]
fn favicon() -> Redirect {
    Redirect::to(uri!(static_files: file = PathBuf::from("logo-32px.png")))
}

#[get("/static/<file..>")]
fn static_files(file: PathBuf) -> Option<Headers<NamedFile>> {
    NamedFile::open(Path::new("static/").join(file))
        .ok()
        .map(Headers::public)
}

macro_rules! register_partial {
    ($handlebars:expr, $name:literal) => {
        $handlebars
            .register_partial(
                $name,
                read_to_string(concat!("templates/", $name, ".hbs"))
                    .expect(concat!("Unable to read file ", concat!("templates/", $name, ".hbs"))),
            )
            .expect(concat!("Count not register partial ", $name, ".hbs"))
    };
}

fn get_link(h: &Helper, _: &Handlebars, _: &Context, _rc: &mut RenderContext, out: &mut dyn Output) -> HelperResult {
    // get parameter from helper or throw an error
    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
    out.write(match param {
        "github" => "https://github.com/BVE-Reborn/bve-reborn",
        "openbve" => "https://openbve-project.net",
        "openbve-docs" => "https://openbve-project.net/documentation_hugo/en/",
        _ => panic!("Unknown link name {}", param),
    })?;
    Ok(())
}

fn configure_templates() -> impl Fairing {
    Template::custom(|engines| {
        let handlebars = &mut engines.handlebars;
        register_partial!(handlebars, "page");
        handlebars.register_helper("link", Box::new(get_link));
    })
}

fn main() {
    rocket::ignite()
        .attach(configure_templates())
        .mount(
            "/",
            routes![index, favicon, static_files, error_create_403, error_create_500],
        )
        .register(catchers![error_403, error_404, error_500])
        .launch();
}
