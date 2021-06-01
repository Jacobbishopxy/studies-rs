#[macro_use]
extern crate diesel;

use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Starting Solar System info server");

    // todo

    Ok(())
}
