//! telegram-bot

use std::env;

use dotenv::dotenv;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    // run
    run().await
}

async fn run() {
    dotenv().ok();

    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        println!("dice");
        message.answer_dice().await?;
        respond(())
    })
    .await;
}
