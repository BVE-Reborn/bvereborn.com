#![feature(proc_macro_hygiene, decl_macro, never_type)]

use rocket::{catchers, routes};

mod errors;
mod helpers;
mod routes;
mod statics;
mod templating;

fn main() {
    rocket::ignite()
        .attach(templating::configure_templates())
        .mount(
            "/",
            routes![
                routes::index,
                routes::download,
                statics::favicon,
                statics::static_files,
                errors::error_create_403,
                errors::error_create_500
            ],
        )
        .register(catchers![errors::error_403, errors::error_404, errors::error_500])
        .launch();
}
