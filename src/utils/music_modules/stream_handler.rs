use crate::utils::structures::guild_queue::GuildQueue;
use serenity::client::Context;
use songbird::input::Input;
use songbird::ytdl;
use songbird::Call;

use log::error;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum StreamErrorCode {}

pub async fn start_stream(
    gq: &mut GuildQueue,
    ctx: &Context,
    voice_manager: Arc<Mutex<Call>>,
) -> Result<(), ()> {
    match get_stream(gq.queue[0].url.clone()).await {
        Ok(res_stream) => {
            let mut voice_lock = voice_manager.lock().await;
            voice_lock.play_source(res_stream);
        }
        Err(_) => {}
    }
    Ok(())
}

async fn get_stream(song_url: String) -> Result<Input, ()> {
    match ytdl(song_url).await {
        Ok(res) => Ok(res),
        Err(why) => {
            error!("{:#?}", why);
            Err(())
        }
    }
}

async fn concat_stream() {}

fn unwrap_stream_error() {}
