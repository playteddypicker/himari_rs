use lavalink_rs::{gateway::*, model::*, LavalinkClient};
use log::info;
use serenity::async_trait;

pub struct LavaHandler;

#[async_trait]
impl LavalinkEventHandler for LavaHandler {
    async fn track_start(&self, _client: LavalinkClient, event: TrackStart) {
        info!("track started at guild: {}", event.guild_id);
    }
}
