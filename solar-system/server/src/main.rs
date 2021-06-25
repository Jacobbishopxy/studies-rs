use log::info;
use tonic::transport::Server;

use solar_system_rpc::solar_system_info::solar_system_info_server::SolarSystemInfoServer;
use solar_system_server::{
    persistence::create_connection_pool, run_migrations, SolarSystemInfoService,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path("solar-system/server/.env").ok();
    env_logger::init();

    info!("Starting Solar System info server");

    let addr = std::env::var("GRPC_SERVER_ADDRESS")?.parse()?;

    // 创建连接池
    let pool = create_connection_pool();
    // 初始化数据库（仅一次）
    run_migrations(&pool);
    // 自定义结构体初始化，入参为连接池
    let solar_system_info = SolarSystemInfoService { pool };
    // 具体 gRPC 的业务实现
    let svc = SolarSystemInfoServer::new(solar_system_info);
    // 启动 gRPC 服务端
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
