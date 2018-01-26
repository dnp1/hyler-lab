//#![deny(warnings)]
extern crate futures;
extern crate futures_cpupool;
extern crate tokio_proto;
extern crate hyper;
extern crate pretty_env_logger;
extern crate tokio_postgres;
extern crate tokio_core;

use tokio_postgres::Connection;

pub mod router;

use futures::Future;
use futures::Stream;
use tokio_core::reactor::{Handle};
//use futures::future::FutureResult;

use hyper::{Method, StatusCode};
use hyper::header::ContentLength;
use hyper::server::{Http, Service, Request, Response};

use futures_cpupool::CpuPool;

static INDEX: &'static [u8] = b"Try POST /echo";

#[derive(Clone)]
struct Echo {
//    connection: &'a Connection,
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

#[allow(dead_code)]
fn setup_db(h: &Handle) -> Connection {
    Connection::connect(
        "postgres://postgres@localhost:5432",
        tokio_postgres::TlsMode::None,
        h,
    ).wait().unwrap()
}

fn main() {
    pretty_env_logger::init();
    let addr = "127.0.0.1:1337".parse().unwrap();
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let server_handle = core.handle();
//    let db_handle = core.handle();
    let pool =  CpuPool::new_num_cpus();

//    let conn = setup_db(&db_handle);
    let echo = Echo{
        pool: pool.clone(),
//        connection : &conn
    };

    let serve = Http::new().serve_addr_handle(&addr, &server_handle, move || Ok(echo.clone())).unwrap();
    let h2 = server_handle.clone();
    println!("Listening on http://{} with 1 thread.", serve.incoming_ref().local_addr());
    server_handle.spawn(serve.for_each(move |conn| {
        h2.spawn(conn.map(|_| ()).map_err(|err| println!("serve error: {:?}", err)));
        Ok(())
    }).map_err(|_| ()));

    core.run(futures::future::empty::<(), ()>()).unwrap();
}
