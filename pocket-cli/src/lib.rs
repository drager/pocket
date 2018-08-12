extern crate clap;
extern crate dotenv;
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate pocket_api;
extern crate tokio;

use futures::Future;
use pocket_api::client::PocketClient;
use server::{get_server, ServerHandle, ServerStatus};
use signal_future::SignalFuture;
use std::net::SocketAddr;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

pub mod config;
pub mod server;
pub mod signal_future;

pub fn initialize(client: &PocketClient, server_port: u16) {
    // If user isn't signed in then we need to
    // spin up a local server so the redirect_uri
    // works on successful sign in.
    if !client.is_signed_in() {
        let server_handle = setup_server_handle(server_port);
        let addr = format!("http://localhost:{}", server_handle.port);

        let sign_in_future = sign_in_flow(client, &addr);

        let handle = thread::spawn(|| tokio::run(server_handle.future));
        let _handle_result = handle.join();
    }
}

fn sign_in_flow<'a>(client: &'a PocketClient, addr: &str) -> impl Future + 'a {
    client
        .sign_in_step_1(addr.to_string())
        .map(|step1| {
            (
                Command::new("xdg-open")
                    .arg(step1.url.to_string())
                    .output()
                    .is_ok(),
                step1.code,
            )
        })
        .and_then(move |(_opened, code)| client.sign_in_step_2(code))
}

fn setup_server_handle(server_port: u16) -> ServerHandle {
    let server_addr: SocketAddr = ([127, 0, 0, 1], server_port).into();

    let server_status = ServerStatus {
        task: None,
        got_request: false,
    };

    let server_status = Arc::new(Mutex::new(server_status));

    let signal = SignalFuture(server_status.clone());

    let server_future = get_server(&server_addr, server_status, Box::new(signal));

    ServerHandle {
        future: Box::new(server_future),
        port: server_port,
    }
}
