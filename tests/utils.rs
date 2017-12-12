extern crate iron;
extern crate acbidder;

use acbidder::server::start_server;
use self::iron::Listening;

pub struct TestServer(Listening);

impl TestServer {
    pub fn new() -> TestServer {
        TestServer(start_server("127.0.0.1", 0))
    }

    pub fn url(&self) -> String {
        format!("http://{}:{}", self.0.socket.ip(), self.0.socket.port())
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.0.close().expect("Error closing server");
    }
}

