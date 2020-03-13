use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use serde::Serialize;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum Os {
    Windows,
    Mac,
    Linux,
    Unknown,
}

impl<'a, 'r> FromRequest<'a, 'r> for Os {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let user_agent = request.headers().get_one("User-Agent");
        Outcome::Success(match user_agent {
            Some(s) if s.contains("Win") => Os::Windows,
            Some(s) if s.contains("Mac") => Os::Mac,
            Some(s) if s.contains("Linux") => Os::Linux,
            _ => Os::Unknown,
        })
    }
}
