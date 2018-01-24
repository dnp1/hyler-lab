#![deny(warnings)]
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_proto;
extern crate hyper;
extern crate pretty_env_logger;

mod router;
use tokio_proto::TcpServer;
use futures::Future;
//use futures::future::FutureResult;

use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

use futures_cpupool::CpuPool;

static INDEX: &'static [u8] = b"Try POST /echo";

#[derive(Clone)]
struct Echo {
    pool: CpuPool
}

impl Service for Echo {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;


    fn call(&self, req: Request) -> Self::Future {
        Box::new(self.pool.spawn_fn(move || {
            futures::future::ok(match (req.method(), req.path()) {
                (&Method::Get, "/") | (&Method::Get, "/echo") => {
                    Response::new()
                        .with_header(ContentLength(INDEX.len() as u64))
                        .with_body(INDEX)
                },
                (&Method::Post, "/echo") => {
                    let mut res = Response::new();
                    if let Some(len) = req.headers().get::<ContentLength>() {
                        res.headers_mut().set(len.clone());
                    }
                    res.with_body(req.body())
                },
                _ => {
                    Response::new()
                        .with_status(StatusCode::NotFound)
                }
            })
        }))
    }
}


fn main() {
    pretty_env_logger::init();
    let http = Http::new();
    let addr = "127.0.0.1:1337".parse().unwrap();
    let tcp_server = TcpServer::new(http, addr);
    let echo = Echo{pool: CpuPool::new_num_cpus()};
    tcp_server.serve( move || Ok(echo.clone()));

//    let server = Http::new().bind(&addr, || )).unwrap();
//    println!("Listening on http://{} with 1 thread.", server.local_addr().unwrap());
//    server.run().unwrap();
}
