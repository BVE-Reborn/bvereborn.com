use maplit::hashmap;
use rocket::http::Status;
use rocket::{catch, get, Request};
use rocket_contrib::templates::Template;

#[catch(403)]
pub fn error_403(req: &Request) -> Template {
    let ctx = hashmap! {
        "code" => String::from("403"),
        "text" => String::from("Forbidden"),
        "reason" => format!("You are not allowed to access {}!", req.uri().path())
    };
    Template::render("error", &ctx)
}

#[catch(404)]
pub fn error_404(req: &Request) -> Template {
    let ctx = hashmap! {
        "code" => String::from("404"),
        "text" => String::from("Not Found"),
        "reason" => format!("Cannot find the page {}!", req.uri().path())
    };
    Template::render("error", &ctx)
}

#[catch(500)]
pub fn error_500() -> Template {
    let ctx = hashmap! {
        "code" => String::from("500"),
        "text" => String::from("Internal Server Error"),
        "reason" => format!("The server errored when trying to make this page 😭")
    };
    Template::render("error", &ctx)
}

#[get("/403")]
pub fn error_create_403() -> Status {
    Status::Forbidden
}

#[get("/500")]
pub fn error_create_500() -> Status {
    Status::InternalServerError
}
