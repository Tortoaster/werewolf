#![allow(incomplete_features)]
#![feature(return_position_impl_trait_in_trait)]

use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
};

use axum::{routing::get, Router, Server};
use tracing::info;

mod action;
mod form;
mod param;
mod player;
mod role;
mod state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let host = env::var("HOST")
        .map(|host| host.parse().expect("invalid host"))
        .unwrap_or(Ipv4Addr::LOCALHOST.into());
    let port = env::var("PORT")
        .map(|port| port.parse().expect("invalid port"))
        .unwrap_or(8000);
    let addr = SocketAddr::new(host, port);

    let app = Router::new().route("/", get(index));

    info!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "Hello, World!"
}
