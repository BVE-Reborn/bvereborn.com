#![feature(proc_macro_hygiene, decl_macro, never_type)]

use rocket::{catchers, routes};
use rocket_contrib::helmet;
use time::Duration;

mod errors;
mod helpers;
mod routes;
mod statics;
mod templating;

fn main() {
    rocket::ignite()
        .attach(templating::configure_templates())
        .attach(
            helmet::SpaceHelmet::default()
                .enable(helmet::Referrer::NoReferrer)
                .enable(helmet::Hsts::Enable(Duration::seconds(365 * 24 * 60 * 60))),
        )
        .mount(
            "/",
            routes![
                routes::index,
                routes::download,
                routes::sample_route,
                routes::install,
                routes::addons,
                routes::community,
                routes::patreon,
                statics::favicon,
                statics::static_files,
                errors::error_create_403,
                errors::error_create_500
            ],
        )
        .register(catchers![errors::error_403, errors::error_404, errors::error_500])
        .launch();
}
