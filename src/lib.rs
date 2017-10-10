extern crate iron;
extern crate router;
extern crate web3;

use iron::prelude::*;
use iron::status;
use router::Router;

mod adchain_registry;

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
    const RPC_ENDPOINT: &str = "http://localhost:8545";

    // TODO: Extract to global and keep alive
    let (_eloop, http) = web3::transports::Http::new(RPC_ENDPOINT).unwrap();
    let web3 = web3::Web3::new(http);

    let adchain_registry = adchain_registry::RegistryInstance::new(&web3);

    if adchain_registry.is_in_registry(domain) {
        return Ok(Response::with((status::Ok, "true")));
    }

    Ok(Response::with((status::Ok, "false")))
}


