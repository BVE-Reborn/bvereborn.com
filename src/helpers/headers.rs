use rocket::response::Responder;
use rocket::{response, Request, Response};

pub enum Cache {
    Private,
    Public,
}

pub struct Headers<T> {
    inner: T,
    cache: Cache,
}

impl<T> Headers<T> {
    pub fn public(inner: T) -> Self {
        Self {
            inner,
            cache: Cache::Public,
        }
    }
    pub fn private(inner: T) -> Self {
        Self {
            inner,
            cache: Cache::Private,
        }
    }
}

impl<'r, T> Responder<'r> for Headers<T>
where
    T: Responder<'r>,
{
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.inner.respond_to(req)?)
            .raw_header("Cache-Control", match self.cache {
                Cache::Private => "private,nocache,max-age=0",
                Cache::Public => "public,max-age=3600",
            }) //  1h (60 * 60)
            .ok()
    }
}
