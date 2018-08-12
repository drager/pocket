extern crate clap;
extern crate dotenv;
extern crate failure;
extern crate futures;
extern crate hyper;
extern crate pocket_api;
extern crate pocket_cli;
extern crate tokio;

use clap::{App, Arg};
use dotenv::dotenv;
use futures::Future;
use pocket_api::client::PocketClient;
use std::env;
use std::process;
use std::str;

const PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

fn main() {
    dotenv().ok();

    let matches = App::new(PKG_NAME.unwrap_or_else(|| "pocket-cli"))
        .version(PKG_VERSION.unwrap_or_else(|| "0.1.0"))
        .author("Jesper Håkansson. <jesper@jesperh.se>")
        .about("Interact with the Pocket API")
        .arg(
            Arg::with_name("api_key")
                .short("k")
                .long("key")
                .value_name("api_key")
                .help("Pocket api key")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("server_port")
                .short("p")
                .long("port")
                .value_name("server_port")
                .help("Local server port")
                .takes_value(true),
        )
        .get_matches();

    let api_key = match matches.value_of("api_key") {
        Some(key) => key.to_owned(),
        None => get_or_exit("POCKET_API_KEY"),
    };

    let server_port = matches
        .value_of("server_port")
        .map(|port| port.parse::<u16>().unwrap())
        .unwrap_or_else(|| 9090);

    let pocket_client = PocketClient::new(&api_key, &3);

    let user_future = pocket_cli::initialize(&pocket_client, server_port);

    tokio::run(
        user_future
            .map(|user| {
                println!("User: {:?}", user);
            })
            .map_err(|err| eprintln!("Error: {:?}", err)),
    );
}

fn get_or_exit(env_var: &str) -> String {
    env::var(env_var).unwrap_or_else(|err| {
        eprintln!(
            "Problem getting env var: {}, errored with: {}",
            env_var, err
        );
        process::exit(1);
    })
}
