//! Server
//!
//! backend server for web-wasm-demo

use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    str::FromStr,
};

use axum::{response::IntoResponse, routing, Router};
use clap::Parser;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

#[derive(Parser, Debug)]
#[clap(name = "server", about = "A server for our wasm project")]
struct Opt {
    #[clap(short = 'l', long = "log", default_value = "debug")]
    log_level: String,
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

    // 配置日志以及 RUST_LOG
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", format!("{},hyper=info,mio=info", opt.log_level));
    }
    // 启用打印
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", routing::get(hello))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

    let sock_addr = SocketAddr::from((
        IpAddr::from_str(opt.addr.as_str()).unwrap_or(IpAddr::V6(Ipv6Addr::LOCALHOST)),
        opt.port,
    ));

    log::info!("listening on http://{sock_addr}");

    axum::Server::bind(&sock_addr)
        .serve(app.into_make_service())
        .await
        .expect("Unable to start server");
}
