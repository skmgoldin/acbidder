extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

pub fn start_server(host: &str, port: u16) -> iron::Listening {
    let mut router = Router::new();
    router.get("/acq/:domain", respond_acq, "registry_query");
    router.any("*", respond_404, "404");

    Iron::new(router).http((host, port)).unwrap()
}

fn respond_404(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::NotFound)))
}

fn respond_acq(req: &mut Request) -> IronResult<Response> {
    let domain = "hi";

    if is_in_registry(domain) {
        return Ok(Response::with((status::Ok, "true")));
    }

    Ok(Response::with((status::Ok, "false")))
}

fn is_in_registry(domain: &str) -> bool {
    false
}


