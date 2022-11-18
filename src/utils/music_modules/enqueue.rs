use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::id::GuildId;

use super::{
    connection_handler::{RequestInfo, RequestType},
    request_search_query,
};

use crate::{
    command_handler::command_handler::CommandReturnValue,
    utils::{music_modules::parse_song_info::SongMetadata, structures::guild_queue::GuildQueue},
};

pub async fn enqueue_main(
    gq: &mut GuildQueue,
    req: RequestInfo,
    search_query: (String, bool),
    request_type: RequestType,
) -> Result<Option<CommandReturnValue>, String> {
    let request = request_search_query::request_main(search_query.clone()).await;

    let return_type = match request {
        Some(song) => {
            //request_type에 대한 handling
            let embed = enqueued_embed(&song, req, search_query.1);
            gq.queue.push(song);
            match request_type {
                RequestType::Command | RequestType::PlaylistCommand => {
                    Ok(Some(CommandReturnValue::SingleStringWithEmbed((
                        if gq.queue.len() == 1 {
                            "재생 시작!".to_string()
                        } else {
                            format!("{}번째 대기열에 추가되었습니다", gq.queue.len() - 1)
                        },
                        embed,
                    ))))
                }
                RequestType::PlayerChannel => Ok(None),
            }
        }
        None => Err("조건에 맞는 검색 결과가 없습니다.".to_string()),
    };
    return_type
}

fn enqueued_embed(
    response_data: &SongMetadata,
    member_info: RequestInfo,
    is_playlist: bool,
) -> CreateEmbed {
    let mut e = CreateEmbed::default();
    match is_playlist {
        true => {
            e.title(response_data.title.clone())
                .url(response_data.url.clone())
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
            e.title(response_data.title.clone())
                .url(response_data.url.clone())
                .thumbnail(response_data.thumbnail.clone())
                .author(|a| {
                    a.name(response_data.author.name.clone())
                        .url(response_data.author.channel_url.clone())
                })
                .footer(|f| {
                    f.text(format!("requested by {}", member_info.member_name))
                        .icon_url(member_info.member_avatar_url)
                });
        }
    }

    e
}
