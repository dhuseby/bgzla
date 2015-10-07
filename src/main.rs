extern crate iron;
extern crate hyper;

use iron::prelude::*;
use iron::status;
use iron::headers::Location;

fn main() {
  fn bgzla(req: &mut Request) -> IronResult<Response> {
    match req.url.path.len() {
      1 => {
        let mut res = Response::with((status::MovedPermanently, ""));
        res.headers.set(Location(format!("https://bugzilla.mozilla.org/show_bug.cgi?id={}", req.url.path[0])));
        Ok(res)
      },
      _ => Ok(Response::with((status::NotFound))),
    }
  }

  Iron::new(bgzla).http("127.0.0.1:3000").unwrap();
}
