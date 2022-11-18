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
use super::parse_song_info::SongMetadata;

//이벤트 종류
//큐가 다 끝났을때(혹은 강제종료)
struct QueueEndNotifier {
    gid: u64,
    http: Arc<Http>,
    queue: &'static mut Box<Vec<SongMetadata>>,
}

//노래 하나만 끝났을때
struct SongEndNotifier {
    gid: GuildId,
    http: Arc<Http>,
}

#[async_trait]
impl VoiceEventHandler for QueueEndNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track(track_list) = ctx {
            let queue = self.gid;
        }

        None
    }
}

pub async fn add_current_event(
    voice_node: &Arc<Mutex<Call>>,
    gq: &mut GuildQueue,
    ch_id: ChannelId,
    ctx: &Context,
) {
    let mut handle = voice_node.lock().await;

    handle.add_global_event(
        Event::Track(TrackEvent::End),
        QueueEndNotifier {
            gid: gq.guild_id,
            http: ctx.http.clone(),
            queue: &mut gq.queue,
        },
    );

    handle.add_global_event(Event::Track(TrackEvent::Pause))
}
