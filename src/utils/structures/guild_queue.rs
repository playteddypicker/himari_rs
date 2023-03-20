use crate::utils::music_modules::parse_song_info::SongMetadata;
use crate::GuildQueueType;

use serenity::client::Context;
use serenity::model::{
    guild::Guild,
    id::{ChannelId, GuildId},
};

use songbird::input::Input;
use songbird::tracks::TrackHandle;

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
    NowPlaying,
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
    pub np_track: Option<TrackHandle>,
    pub streaming_queue: usize,
    pub prev_queue: Box<Vec<SongMetadata>>, //max 10
    pub play_status: PlayStatus,
    pub loop_mode: LoopMode,
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
            np_track: None,
            prev_queue: Box::new(Vec::new()),
            play_status: PlayStatus::Idle,
            loop_mode: LoopMode::NormalPlay,
            volume: 0.3,
            search_filter: SearchFilter {
                //db에서 로드함
                duration_limit: 0,
                ban_keywords: Box::new(Vec::new()),
            },
        }))
    }

    //emits only ejected
    pub fn init(&mut self) {
        self.queue = Box::new(VecDeque::new());
        self.streaming_queue = 0;
    }

    //skips to the next song if it exists or stop the queue.
    fn skip(&mut self) {
        if let Some(tr) = self.np_track.as_mut() {
            tr.stop().unwrap();
            self.play_status = PlayStatus::Idle;
        }
    }

    //delete all tracks in queue and stops current track.
    fn stop() {}

    //delete all tracks in queue, stops current track and leave from voice channel.
    fn eject() {}

    //pauses if track is now plaing or resume.
    fn pause_or_resume(&mut self) {
        match self.play_status {
            PlayStatus::NowPlaying => {
                //나중에 에러핸들링
                self.np_track.as_mut().unwrap().pause().unwrap();
                self.play_status = PlayStatus::Paused;
            }
            PlayStatus::Paused => {
                self.np_track.as_mut().unwrap().play().unwrap();
                self.play_status = PlayStatus::NowPlaying;
            }
            _ => {}
        }
    }

    //seek times of current track if it's playing.
    fn seek() {}

    //shuffle all tracks in queue and re-concat next song.
    fn shuffle() {}

    //set the default volume of the player.
    fn default_volume() {}

    //set the specific volume of current track.
    fn track_volume() {}

    //jumps over inputted index and starts playing track.
    fn jump() {}

    //remove single or range of track(s) in the queue.
    fn remove() {}

    //switch two tracks in the queue.
    fn switch() {}

    //debug mode.
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
