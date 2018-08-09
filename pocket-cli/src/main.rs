extern crate clap;
extern crate dotenv;
extern crate pocket_api;
extern crate tokio;

use clap::{App, Arg};
use dotenv::dotenv;
use pocket_api::client::PocketClient;
use std::env;
use std::process::{self, Command};
use std::str;
use tokio::prelude::{Future, Stream};

const PKG_VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
const PKG_NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

fn main() {
    dotenv().ok();

    let matches = App::new(PKG_NAME.unwrap_or_else(|| "pocket-cli"))
        .version(PKG_VERSION.unwrap_or_else(|| "0.0.1"))
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
        .get_matches();

    let api_key = match matches.value_of("api_key") {
        Some(key) => key.to_owned(),
        None => get_or_exit("POCKET_API_KEY"),
    };

    //let future = PocketClient::new(&api_key, &3).sign_in();
    /*.and_then(move |data| {*/
    //let body = data.into_body()
    //.concat2()
    //.map_err(Error::from)
    //.map(|chunk| String::from_utf8(chunk.to_vec()));

    //body
    /*});*/

    /*tokio::run(*/
    //future
    //.map(|data| {
    //println!("Data: {:?}", data);
    //Command::new("xdg-open").arg(data.to_string()).output();
    //})
    //.map_err(|err| eprintln!("Err: {:?}", err)),
    /*);*/}

fn get_or_exit(env_var: &str) -> String {
    env::var(env_var).unwrap_or_else(|err| {
        eprintln!(
            "Problem getting env var: {}, errored with: {}",
            env_var, err
        );
        process::exit(1);
    })
}
