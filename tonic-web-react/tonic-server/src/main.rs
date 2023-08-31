//! file: main.rs
//! author: Jacob Xie
//! date: 2023/08/31 22:07:31 Thursday
//! brief:

#![allow(non_snake_case)]

use std::time::SystemTime;

// use http::Method;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

pub mod grpc_page {
    tonic::include_proto!("page");
}

pub mod grpc_hw {
    tonic::include_proto!("helloworld");
}

use grpc_page::page_server::{Page, PageServer};
use grpc_page::{
    Block, HeaderBlock, HeaderData, PageReply, PageRequest, ParagraphBlock, ParagraphData,
};
// use tonic_web::GrpcWebLayer;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

use grpc_hw::greeter_server::{Greeter, GreeterServer};
use grpc_hw::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
pub struct MyPage {}

#[tonic::async_trait]
impl Page for MyPage {
    async fn get_page(&self, request: Request<PageRequest>) -> Result<Response<PageReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = match request.into_inner().name.as_str() {
            "home" => PageReply {
                name: "home".into(),
                time: Some(SystemTime::now().into()),
                blocks: vec![
                    Block { block: Some(grpc_page::block::Block::HeaderBlock(HeaderBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "header".into(),
                        data: Some(HeaderData {
                            text: "Data Engineer, Problem Solver".into(),
                            level: 2
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::ParagraphBlock(ParagraphBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "paragraph".into(),
                        data: Some(ParagraphData {
                            text: "<span class=\"highlight\">Data and processes accompany me through my entire professional life. As an expert in data and processes, especially in supply chain management, production and their interfaces, who speaks both the technical and the business language and can interpret in between, I contribute strongly to the understanding and better communication of problems.</span>".into()
                        })
                    }))}
                ],
                version: "0.1.0".into()
            },
            "impressum" => PageReply {
                name: "impressum".into(),
                time: Some(SystemTime::now().into()),
                blocks: vec![
                    Block { block: Some(grpc_page::block::Block::HeaderBlock(HeaderBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "header".into(),
                        data: Some(HeaderData {
                            text: "Angaben gemäß §5 TMG".into(),
                            level: 2
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::ParagraphBlock(ParagraphBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "paragraph".into(),
                        data: Some(ParagraphData {
                            text: "Christopher Scholz<br>An der Dahme 3<br>12527 Berlin".into()
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::HeaderBlock(HeaderBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "header".into(),
                        data: Some(HeaderData {
                            text: "Kontakt".into(),
                            level: 2
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::ParagraphBlock(ParagraphBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "paragraph".into(),
                        data: Some(ParagraphData {
                            text: "Email: <a href=\"mailto:website@christopher-scholz.com\">website@christopher-scholz.com</a>".into()
                        })
                    }))}
                ],
                version: "0.1.0".into()
            },
            _ => PageReply {
                name: "home".into(),
                time: Some(SystemTime::now().into()),
                blocks: vec![
                    Block { block: Some(grpc_page::block::Block::HeaderBlock(HeaderBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "header".into(),
                        data: Some(HeaderData {
                            text: "Unknown".into(),
                            level: 2
                        })
                    }))},
                    Block { block: Some(grpc_page::block::Block::ParagraphBlock(ParagraphBlock {
                        id: Uuid::new_v4().to_string(),
                        r#type: "paragraph".into(),
                        data: Some(ParagraphData {
                            text: "This page is not known.".into()
                        })
                    }))}
                ],
                version: "0.1.0".into()
            },
        };

        Ok(Response::new(reply))
    }
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<HelloReply>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let reply = grpc_hw::HelloReply {
            message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let addr = "[::1]:3000".parse()?;
    let addr = "127.0.0.1:3000".parse()?;

    let page = MyPage::default();
    let page = PageServer::new(page);

    let greeter = MyGreeter::default();
    let greeter = GreeterServer::new(greeter);
    println!("listening on {}", addr);

    let cors = CorsLayer::new()
        .allow_headers(Any)
        // .allow_methods([Method::POST])
        .allow_methods(Any)
        // .allow_origin(["http://localhost:5000".parse()?]);
        .allow_origin(Any);

    Server::builder()
        .accept_http1(true)
        .layer(cors)
        .add_service(tonic_web::enable(page))
        .add_service(tonic_web::enable(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
