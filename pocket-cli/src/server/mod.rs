use failure::Error;
use futures::{future, task, Future, Stream};
use hyper;
use std::net::SocketAddr;
use std::string::FromUtf8Error;
use std::sync::{Arc, Mutex};

type BoxFut = Box<Future<Item = hyper::Response<hyper::Body>, Error = hyper::Error> + Send>;

#[derive(Debug)]
pub struct ServerStatus {
    pub task: Option<task::Task>,
    pub got_request: bool,
}

pub struct ServerHandle {
    pub future: Box<Future<Item = (), Error = ()> + Send>,
    pub port: u16,
}

pub fn get_server<'a>(
    addr: &SocketAddr,
    server_status: Arc<Mutex<ServerStatus>>,
    signal: Box<Future<Item = (), Error = ()> + Send>,
) -> impl Future<Item = (), Error = ()> {
    let server = hyper::server::Server::bind(addr)
        .serve(move || hyper::service::service_fn(serve(server_status.clone())))
        .map_err(|e| eprintln!("server error: {}", e));

    let future = server
        .select(signal.map(|s| {
            println!("Got signal");
            s
        }))
        .map_err(|_e| eprintln!("server error:"))
        .map(|_| ());
    future
}

fn serve(
    server_status: Arc<Mutex<ServerStatus>>,
) -> impl Fn(hyper::Request<hyper::Body>) -> BoxFut + Send {
    move |_request: hyper::Request<hyper::Body>| {
        let mut status = server_status.lock().unwrap();
        status.got_request = true;

        if let Some(ref task) = status.task {
            task.notify();
        }

        Box::new(future::ok(hyper::Response::new(hyper::Body::from(
            format!("Request status {:?}", status),
        ))))
    }
}

pub fn body_to_string(
    body: hyper::Body,
) -> impl Future<Item = Result<String, FromUtf8Error>, Error = Error> {
    body.concat2()
        .map_err(Error::from)
        .map(|chunk| String::from_utf8(chunk.to_vec()))
}
