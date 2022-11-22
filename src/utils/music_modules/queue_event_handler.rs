use songbird::{
    input::{self, restartable::Restartable},
    Call, Event, EventContext, EventHandler as VoiceEventHandler, TrackEvent,
};

use serenity::{
    async_trait,
    client::Context,
    http::Http,
    model::id::{ChannelId, GuildId},
};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::super::structures::guild_queue::GuildQueue;
use super::stream_handler::concat_stream;

//이벤트 종류
//노래가 끝났을때(혹은 강제종료)
struct TrackEndNotifier {
    http: Arc<Http>,
    guild_queue: Arc<Mutex<GuildQueue>>,
    voice_manager: Arc<Mutex<Call>>,
}

//노래가 시작됐을때
struct TrackPlayNotifier {
    http: Arc<Http>,
    guild_queue: Arc<Mutex<GuildQueue>>,
    voice_manager: Arc<Mutex<Call>>,
}

struct TrackLoopNotifier {
    http: Arc<Http>,
    guild_queue: Arc<Mutex<GuildQueue>>,
}

struct TrackPauseNotifier {
    http: Arc<Http>,
    guild_queue: Arc<Mutex<GuildQueue>>,
}

#[async_trait]
impl VoiceEventHandler for TrackEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let (mut gq_lock, mut voice_manager) =
            tokio::join!(self.guild_queue.lock(), self.voice_manager.lock());
        if let EventContext::Track(track_list) = ctx {
            log::info!("Song has been finished");
            log::info!("{:#?}", track_list);
        }
        concat_stream(&mut gq_lock, &mut voice_manager).await;
        gq_lock.queue.pop_front();
        gq_lock.streaming_queue -= 1;
        drop(gq_lock);
        drop(voice_manager);
        None
    }
}

#[async_trait]
impl VoiceEventHandler for TrackPlayNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        //let gq_lock = self.guild_queue.lock().await;
        log::info!(
            "new song has been started! ",
            //    gq_lock.queue[gq_lock.queue.len() - 1].title
        );
        //drop(gq_lock);
        None
    }
}

pub async fn add_current_event(
    voice_node: &Arc<Mutex<Call>>,
    gq: Arc<Mutex<GuildQueue>>,
    ch_id: ChannelId,
    ctx: &Context,
) {
    let mut handle = voice_node.lock().await;

    handle.add_global_event(
        Event::Track(TrackEvent::End),
        TrackEndNotifier {
            http: ctx.http.clone(),
            guild_queue: gq.clone(),
            voice_manager: voice_node.clone(),
        },
    );

    handle.add_global_event(
        Event::Track(TrackEvent::Play),
        TrackPlayNotifier {
            http: ctx.http.clone(),
            guild_queue: gq.clone(),
            voice_manager: voice_node.clone(),
        },
    );

    /*handle.add_global_event(
        Event::Track(TrackEvent::Pause),
        TrackPauseNotifier {
            http: ctx.http.clone(),
            guild_queue: gq.clone(),
        },
    );*/
}
