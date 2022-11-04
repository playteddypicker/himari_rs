use serenity::model::id::{ChannelId, GuildId};

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

pub struct SongAuthor {
    name: String,
    channel_thumbnail: Option<String>,
    channel_url: String,
}

pub struct SongDuration {
    timestamp: String,
    seconds: u64,
}

pub struct SearchFilter {
    duration_limit: u16,
    ban_keywords: Box<Vec<String>>,
}

pub struct GuildQueue {
    guild_id: GuildId,
    streaming_channel: Option<ChannelId>,
    command_channel: Option<ChannelId>,
    queue: Box<Vec<SongData>>,
    prev_queue: Box<Vec<SongData>>, //max 10
    play_status: PlayStatus,
    volume: u8, //0~255까지
    search_filter: SearchFilter,
}

impl GuildQueue {
    fn new(gid: GuildId) -> Self {
        GuildQueue {
            guild_id: gid,
            streaming_channel: None,
            command_channel: None,
            queue: Box::new(Vec::new()),
            prev_queue: Box::new(Vec::new()),
            play_status: PlayStatus::Idle,
            volume: 30,
            search_filter: SearchFilter {
                //db에서 로드함
                duration_limit: 0,
                ban_keywords: Box::new(Vec::new()),
            },
        }
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

pub struct SongData {
    title: String,
    url: String,
    duration: SongDuration,
    thumbnail: String,
    author: SongAuthor,
}
