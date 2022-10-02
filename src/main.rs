use dotenv::dotenv;
use serenity::prelude::*;
use std::env;

mod event_handler;
mod command_handler;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let client_token = env::var("DISCORD_TOKEN").expect("token does not exists");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILD_MEMBERS;

    let mut client = Client::builder(&client_token, intents)
        .event_handler(event_handler::event_handler::DiscordEventHandler)
        .await
        .expect("Error creating client.");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
