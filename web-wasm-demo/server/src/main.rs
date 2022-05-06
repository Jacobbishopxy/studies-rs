//! Server
//!
//!  

use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::{response::IntoResponse, routing, Router};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project")]
struct Opt {
    #[clap(short = 'a', long = "addr", default_value = "::1")]
    addr: String,
    #[clap(short = 'p', long = "port", default_value = "8080")]
    port: u16,
}

async fn hello() -> impl IntoResponse {
    "hello from server"
}

#[tokio::main]
async fn main() {
    let opt = Opt::parse();

    let app = Router::new().route("/", routing::get(hello));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    println!("listening on http://{sock_addr}");

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}
