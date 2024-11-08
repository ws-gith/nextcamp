#![cfg_attr(debug_assertions, allow(unused_imports, dead_code))]
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate nextcamp;

use hyper::Uri;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::{
    fs,
    net::{TcpListener, TcpStream},
};

async fn app() {
    // let config_string = fs::read_to_string("./config.toml").await?;
    // let config: Config = toml::from_str(&config_string)?;
    // println!("{:#?}", config);

    // let url = "http://jsonplaceholder.typicode.com/users".parse::<Uri>()?;
    // let addr = f!(
    //     "{}:{}",
    //     url.host().expect("Host should be available"),
    //     url.port().expect("Port should be available")
    // );

    // let _stream = TcpStream::connect(addr).await?;
    // let io = TokioIo::new(stream);

    // Verify the user with api

    // Let believe it's successful now

    // Verify the smtp
    // Verify the letter
    // Verify the lead's

    // Send using the hyper with tls setup possible and
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .pretty()
            .with_file(true)
            .with_level(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_max_level(tracing::Level::DEBUG)
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .init();

        app().await;
    }

    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .pretty()
            .without_time()
            .with_file(false)
            .with_line_number(false)
            .with_max_level(tracing::Level::DEBUG)
            .init();

        app().await;
    }
}

#[derive(new, Debug, Deserialize, Serialize)]
pub struct Config {
    auth: Auth,
    sender: Sender,

    #[serde(rename = "smtp")]
    smtps: Vec<Smtp>,
}

#[derive(new, Debug, Deserialize, Serialize)]
pub struct Auth {
    key: String,
}

#[derive(new, Debug, Deserialize, Serialize)]
pub struct Sender {
    delay: i64,
}

#[derive(new, Debug, Deserialize, Serialize)]
pub struct Smtp {
    name: String,
    host: String,
    port: i64,
    username: String,
    password: String,
}
