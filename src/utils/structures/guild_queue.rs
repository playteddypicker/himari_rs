use crate::utils::music_modules::parse_song_info::SongMetadata;
use crate::GuildQueueType;

use serenity::client::Context;
use serenity::model::{
    guild::Guild,
    id::{ChannelId, GuildId},
};

use songbird::input::Input;

use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

pub enum LoopMode {
    NormalPlay,
    AutoPlay,
    RepeatSingle,
    RepeatQueue,
}

pub enum PlayStatus {
    Idle,
    NowPlaying(LoopMode),
    Paused,
    Buffering,
    Error(String),
}

pub struct SearchFilter {
    duration_limit: u16,
    ban_keywords: Box<Vec<String>>,
}

pub struct GuildQueue {
    pub guild_id: u64, //to get guild info from Guild::get().
    pub streaming_channel: Option<ChannelId>,
    pub command_channel: Option<ChannelId>,
    pub queue: Box<VecDeque<SongMetadata>>,
    pub streaming_queue: usize,
    pub prev_queue: Box<Vec<SongMetadata>>, //max 10
    pub play_status: PlayStatus,
    pub volume: f32, //0~1까지
    pub search_filter: SearchFilter,
}

impl GuildQueue {
    fn new(gid: u64) -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(GuildQueue {
            guild_id: gid,
            streaming_channel: None,
            command_channel: None,
            queue: Box::new(VecDeque::new()),
            streaming_queue: 0,
            prev_queue: Box::new(Vec::new()),
            play_status: PlayStatus::Idle,
            volume: 0.3,
            search_filter: SearchFilter {
                //db에서 로드함
                duration_limit: 0,
                ban_keywords: Box::new(Vec::new()),
            },
        }))
    }
    pub fn init(&mut self) {
        self.queue = Box::new(VecDeque::new());
        self.streaming_queue = 0;
    }
    async fn pause() {}
    async fn stop() {}
    async fn skip() {}
    async fn eject() {}
    async fn seek() {}
    fn shuffle() {}
    fn volume() {}
    fn jump() {}
    fn remove() {}
    fn switch() {}
    fn refresh() {}
}

pub async fn load_guild_multi(ctx: &Context) {
    let counter = {
        let data_read = ctx.data.read().await;
        data_read.get::<GuildQueueType>().unwrap().clone()
    };
    {
        let mut counter_lock = counter.write().await;
        //rust docs의 RwLock - write()메소드 체크.
        for gid in ctx.cache.guilds().clone().iter() {
            counter_lock.entry(gid.0).or_insert(GuildQueue::new(gid.0));
        }
    }
}
