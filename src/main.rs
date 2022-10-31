use dotenv::dotenv;
use env_logger;
use log::error;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::env;

mod command_handler;
mod event_handler;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let client_token = env::var("DISCORD_TOKEN").expect("token does not exists");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&client_token, intents)
        .event_handler(event_handler::event_handler::DiscordEventHandler)
        .register_songbird()
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
