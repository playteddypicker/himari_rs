use serde_json::Value;
use songbird::input::Input;

use std::env;
use std::process::Stdio;
use std::time::Duration;

#[derive(Debug)]
struct JsonError {
    error: serde_json::Error,
    parsed_text: String,
}

use tokio::process::Command as TokioCommand;

pub struct SongMetadata {
    pub title: String,
    pub url: String,
    pub duration: SongDuration,
    pub thumbnail: String,
    pub author: SongAuthor,
}

impl Clone for SongMetadata {
    fn clone(&self) -> SongMetadata {
        *self
    }
}

pub struct SongAuthor {
    pub name: String,
    //pub channel_thumbnail: Option<String>, 나중에 추가
    pub channel_url: String,
}

pub struct SongDuration {
    pub timestamp: String,
    pub seconds: Duration,
}

impl SongMetadata {
    pub fn from_youtube_single(output: Value) -> Self {
        let default_thumbnail = env::var("PLAYEREMBED_URL").unwrap();
        let obj = output.as_object();

        Self {
            title: if let Some(t) = obj
                .and_then(|m| m.get("title"))
                .and_then(Value::as_str)
                .map(str::to_string)
            {
                t
            } else {
                "알 수 없는 제목".to_string()
            },
            url: if let Some(u) = obj
                .and_then(|m| m.get("webpage_url"))
                .and_then(Value::as_str)
                .map(str::to_string)
            {
                u
            } else {
                "https://youtube.com".to_string()
            },
            duration: SongDuration {
                timestamp: if let Some(ts) = obj
                    .and_then(|m| m.get("duration_string"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
                {
                    ts
                } else {
                    "--:--".to_string()
                },
                seconds: if let Some(sec) = obj
                    .and_then(|m| m.get("duration"))
                    .and_then(Value::as_f64)
                    .map(Duration::from_secs_f64)
                {
                    sec
                } else {
                    Duration::from_secs(0)
                },
            },
            thumbnail: if let Some(thumb) = obj
                .and_then(|m| m.get("thumbnail"))
                .and_then(Value::as_str)
                .map(str::to_string)
            {
                thumb
            } else {
                default_thumbnail
            },
            author: SongAuthor {
                name: if let Some(ch) = obj
                    .and_then(|m| m.get("channel"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
                {
                    ch
                } else {
                    "알 수 없는 채널".to_string()
                },
                channel_url: if let Some(ch_url) = obj
                    .and_then(|m| m.get("channel_url"))
                    .and_then(Value::as_str)
                    .map(str::to_string)
                {
                    ch_url
                } else {
                    "https://youtube.com/".to_string()
                },
            },
        }
    }
}

pub async fn yt_single(search_string: String) -> Result<Value, JsonError> {
    let ytdl_args = [
        "--print-json",
        "-f",
        "webm[abr>0]/bestaudio/best/",
        "-R",
        "infinite",
        "--no-playlist",
        "--ignore-config",
        "--no-warnings",
        search_string.as_ref(),
        "-o",
        "-",
    ];

    let ytdl_output = TokioCommand::new("yt-dlp")
        .args(&ytdl_args)
        .stdin(Stdio::null())
        .output()
        .await
        .unwrap();

    let o_vec = ytdl_output.stdout;

    let end = (&o_vec)
        .iter()
        .position(|e| *e == 0xA)
        .unwrap_or_else(|| o_vec.len());

    serde_json::from_slice(&o_vec[..end]).map_err(|err| JsonError {
        error: err,
        parsed_text: std::str::from_utf8(&o_vec).unwrap_or_default().to_string(),
    })
}
/*
pub async fn yt_playlist(search_string: String) -> SongMetadata {}

pub async fn yt_search(search_string: String) -> SongMetadata {}

pub async fn sc_single(search_string: String) -> SongMetadata {}

pub async fn sc_playlist(search_string: String) -> SongMetadata {}

pub async fn spt_single(search_string: String) -> SongMetadata {}

pub async fn spt_playlist(serach_string: String) -> SongMetadata {}
*/
