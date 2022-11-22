use super::super::structures::guild_queue::{GuildQueue, LoopMode, PlayStatus};
use songbird::input::Input;
use songbird::ytdl;
use songbird::Call;

use log::{error, warn};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub enum StreamErrorCode {}

enum StreamConcatMode {
    TooLongtoConcat,
    ConcatEnable,
    LazyConcat,
}

pub async fn stream_main(
    guild_queue: &Arc<Mutex<GuildQueue>>,
    voice_manager: Arc<Mutex<Call>>,
    start: &std::time::SystemTime,
) -> Result<(), String> {
    let (mut gq, mut voice_lock) = tokio::join!(guild_queue.lock(), voice_manager.lock());
    //봇이 처음으로 노래를 틀기 시작할 때
    if let PlayStatus::Idle = gq.play_status {
        if gq.queue.len() > 0 {
            if let Err(why) = start_stream(&mut gq, &mut voice_lock, &start).await {
                return Err(why);
            }
            gq.play_status = PlayStatus::NowPlaying(LoopMode::NormalPlay);
        } else {
            warn!(
                "Requested Playing query, but queue is empty on {}. unwrapping..",
                gq.guild_id
            );
            gq.init();
        }
    //재생중이라는거니까 concat할 가능성이 있음. 일시정지, 오토플레이여도 마찬가지
    } else {
        if let Err(why) = concat_stream(&mut gq, &mut voice_lock).await {
            return Err(why);
        }
    }

    drop(gq);
    drop(voice_lock);

    Ok(())
}

async fn start_stream(
    gq: &mut GuildQueue,
    voice_manager: &mut Call,
    start: &std::time::SystemTime,
) -> Result<(), String> {
    return match get_stream(gq.queue[0].url.clone()).await {
        Ok(res_stream) => {
            log::info!("getting stream: {}s", start.elapsed().unwrap().as_secs());
            voice_manager.enqueue_source(res_stream);
            gq.streaming_queue += 1;
            Ok(())
        }
        Err(why) => Err(why),
    };
}

async fn get_stream(song_url: String) -> Result<Input, String> {
    //나중에 사운드클라우드, 스포티파이로부터도 불러와야함
    match ytdl(song_url).await {
        Ok(res) => Ok(res),
        Err(why) => {
            error!("{:#?}", why);
            Err(
                "⚠️ 음성 파일 스트림을 다운로드 하는 데 실패했습니다. 나중에 다시 시도해주세요."
                    .to_string(),
            )
        }
    }
}

pub async fn concat_stream(gq: &mut GuildQueue, voice_manager: &mut Call) -> Result<(), String> {
    if gq.queue.len() == 0 {
        return Err("대기열에 아무 노래도 남아있지 않습니다.".to_string());
    }

    if gq.streaming_queue == 2 {
        return Ok(());
    }

    match concat_helper(gq.queue[0].duration.seconds, gq.queue[1].duration.seconds) {
        StreamConcatMode::ConcatEnable | StreamConcatMode::LazyConcat => {
            //미리 넣기
            let next_stream = get_stream(gq.queue[1].url.clone()).await?;
            voice_manager.enqueue_source(next_stream);
            gq.streaming_queue += 1;
        }
        StreamConcatMode::TooLongtoConcat => {}
    }

    Ok(())
}

fn concat_helper(npsong_duration: Duration, concatsong_duration: Duration) -> StreamConcatMode {
    //몇분 이상은 concat을 안할건지.. 일단은 30분으로 정해둠
    if npsong_duration.as_secs() > 30 * 60 || concatsong_duration.as_secs() > 30 * 60 {
        StreamConcatMode::TooLongtoConcat
    } else {
        StreamConcatMode::ConcatEnable
    }
}

fn unwrap_stream_error() {}
