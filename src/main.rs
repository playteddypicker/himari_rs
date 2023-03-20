use crate::utils::structures::guild_queue;
use dotenv::dotenv;
use env_logger;
use lavalink_rs::LavalinkClient;
use log::error;
use serenity::prelude::*;
use songbird::{Config, SerenityInit};
use std::{collections::HashMap, env, sync::Arc};
use tokio::sync::Mutex;
use tokio::sync::RwLock;

mod command_handler;
mod event_handler;
mod utils;

struct Lavalink;

impl TypeMapKey for Lavalink {
    type Value = LavalinkClient;
}

struct GuildQueueType;

impl TypeMapKey for GuildQueueType {
    type Value = Arc<RwLock<HashMap<u64, Arc<Mutex<guild_queue::GuildQueue>>>>>;
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
        .register_songbird_from_config(Config::default().preallocated_tracks(2))
        .await
        .expect("Error creating client.");

    //global variable을 위해 client에 data를 매달아줌
    {
        let mut data = client.data.write().await;
        data.insert::<Lavalink>(lavalink_client);
        data.insert::<GuildQueueType>(Arc::new(RwLock::new(HashMap::default())));
    }

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
