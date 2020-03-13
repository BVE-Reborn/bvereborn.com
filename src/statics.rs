use crate::headers::Headers;
use rocket::response::{NamedFile, Redirect};
use rocket::{get, uri};
use std::path::{Path, PathBuf};

#[get("/favicon.ico")]
pub fn favicon() -> Redirect {
    Redirect::to(uri!(static_files: file = PathBuf::from("logo-32px.png")))
}

#[get("/static/<file..>")]
pub fn static_files(file: PathBuf) -> Option<Headers<NamedFile>> {
    NamedFile::open(Path::new("static/").join(file))
        .ok()
        .map(Headers::public)
}
