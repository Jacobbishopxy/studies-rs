use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};

pub mod hello_world {
    // 此处的字符串必须匹配 helloworld.proto 中的 package 名称
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>, // 接收 HelloRequest 类型的请求
    ) -> Result<Response<HelloReply>, Status> {
        // 返回一个 HelloReply 类型的实例
        println!("Got a request: {:?}", request);

        let reply = hello_world::HelloReply {
            // 使用 `.into_inner()` 访问 gRPC 的请求与响应字段
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        // 发送格式化的信息给请求者
        Ok(Response::new(reply))
    }
}

// 使用 tokio 作为异步运行时（需要 Tokio 作为独立依赖，确保在 Cargo.toml 中添加了该依赖）
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
