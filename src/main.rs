use dotenv::dotenv;
use env_logger;
use lavalink_rs::LavalinkClient;
use log::error;
use serenity::prelude::*;
use songbird::SerenityInit;
use std::env;

mod command_handler;
mod event_handler;
mod utils;

struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let client_token = env::var("DISCORD_TOKEN").expect("token does not exists");
    let application_id = env::var("APP_ID").expect("id does not exists");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS;

    let lavalink_client = LavalinkClient::builder(application_id.parse::<u64>().unwrap())
        .set_host("127.0.0.1")
        .set_password(env::var("LAVALINK_PASSWORD").expect("mismatched password"))
        .build(event_handler::lavalink_event_handler::LavaHandler)
        .await
        .expect("error creating lavalink client.");

    let mut client = Client::builder(&client_token, intents)
        .event_handler(event_handler::event_handler::DiscordEventHandler)
        .register_songbird()
        .await
        .expect("Error creating client.");

    {
        let mut data = client.data.write().await;
        data.insert::<Lavalink>(lavalink_client);
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
