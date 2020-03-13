#![feature(proc_macro_hygiene, decl_macro)]

use maplit::hashmap;
use rocket::fairing::Fairing;
use rocket::{get, routes};
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use std::fs::read_to_string;

#[get("/")]
fn index() -> Template {
    let map = hashmap! {
        "name" => "connor"
    };
    Template::render("main", &map)
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

fn configure_templates() -> impl Fairing {
    Template::custom(|engines| {
        let handlebars = &mut engines.handlebars;
        register_partial!(handlebars, "page");
    })
}

fn main() {
    rocket::ignite()
        .attach(configure_templates())
        .mount("/static", StaticFiles::from("static"))
        .mount("/", routes![index])
        .launch();
}
