extern crate hyper;
extern crate futures;

#[macro_use]
extern crate log;
extern crate env_logger;
use hyper::{Chunk, StatusCode};
use hyper::Method::{Get, Post};
use hyper::server::{Request, Response, Service};

use futures::Stream;
use futures::future::{Future, FutureResult};

struct Microservice;

impl Service for Microservice {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, request: Request) -> Self::Future {
        match (request.method(), request.path()) {
            (&Post, "/") => {
                println!("Microservice received a request: {:?}", request);

                Box::new(futures::future::ok(Response::new()))

            }
            _ => Box::new(futures::future::ok(
                Response::new().with_status(StatusCode::NotFound),
            )),
        }
    }

}
fn main() {
    env_logger::init();
    println!("Running microservice at {}", "address");

    let address = "127.0.0.1:8080".parse().unwrap();
    let server = hyper::server::Http::new()
        .bind(&address, || Ok(Microservice {}))
        .unwrap();
    println!("Running microservice at {}", address);
    server.run().unwrap();
}
