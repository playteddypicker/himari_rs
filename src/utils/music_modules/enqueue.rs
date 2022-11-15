use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::id::GuildId;

use super::{
    connection_handler::{RequestInfo, RequestType},
    request_search_query,
};

use crate::{
    command_handler::command_handler::CommandReturnValue,
    utils::{music_modules::parse_song_info::SongMetadata, structures::guild_queue},
    GuildQueueType,
};

pub async fn enqueue_main(
    ctx: &Context,
    req: RequestInfo,
    gid: GuildId,
    search_query: (String, bool),
    request_type: RequestType,
) -> Result<CommandReturnValue, ()> {
    let counter = {
        let r = ctx.data.read().await;
        r.get::<GuildQueueType>().unwrap().clone()
    };
    let mut guilds = counter.write().expect("RwLock Poisoned");
    let gq = guilds.get_mut(&gid.0).unwrap();

    let request = request_search_query::request_main(search_query.clone()).await;

    let return_type = match request_type {
        //커맨드나 플레이리스트 명령어에서 요청한 경우
        RequestType::Command | RequestType::PlaylistCommand => match request {
            Some(song) => {
                gq.queue.push(song.clone());
                Ok(CommandReturnValue::SingleStringWithEmbed((
                    if gq.queue.len() == 0 {
                        "재생 시작!".to_string()
                    } else {
                        format!("{}번째 대기열에 추가되었습니다", gq.queue.len())
                    },
                    enqueued_embed(song, req, search_query.1),
                )))
            }
            None => Ok(CommandReturnValue::SingleString(
                "조건에 맞는 검색 결과가 없습니다.".to_string(),
            )),
        },
        //그냥 플레이어 채널에서 큐 넣은 경우. 뭘 넣었다고 안뜨게 해도 됨.
        RequestType::PlayerChannel => Err(()),
    };
    return_type
}

fn enqueued_embed(
    response_data: SongMetadata,
    member_info: RequestInfo,
    is_playlist: bool,
) -> CreateEmbed {
    let mut e = CreateEmbed::default();
    match is_playlist {
        true => {
            e.title(response_data.title)
                .url(response_data.url)
                .thumbnail("대충 플레이리스트 썸넬")
                .author(|a| {
                    a.name("대충 플레이리스트 만든놈 채널 이름")
                    //        .url("대충 플레이리스트 채널 링크")
                    //        .icon_url("대충 플레이리스트 채널 프사 이미지 링크")
                })
                .footer(|f| {
                    f.text(format!("requested by {}", member_info.member_name))
                        .icon_url(member_info.member_avatar_url)
                });
        }
        false => {
            e.title(response_data.title)
                .url(response_data.url)
                .thumbnail(response_data.thumbnail)
                .author(|a| {
                    a.name(response_data.author.name)
                        .url(response_data.author.channel_url)
                })
                .footer(|f| {
                    f.text(format!("requested by {}", member_info.member_name))
                        .icon_url(member_info.member_avatar_url)
                });
        }
    }

    e
}
