use serenity::builder::CreateEmbed;
use serenity::model::id::GuildId;

use super::request_search_query;
use super::stream::RequestInfo;
use super::stream::RequestType;

pub async fn enqueue_main(
    req: RequestInfo,
    gid: GuildId,
    search_query: (String, bool),
    request_type: RequestType,
) -> Option<(String, CreateEmbed)> {
    request_search_query::request_search(search_query).await;
    None
}

fn enqueued_embed(member_info: RequestInfo, is_playlist: bool) -> CreateEmbed {
    let mut e = CreateEmbed::default();
    match is_playlist {
        true => {
            e.title("대충 플레이리스트 제목")
                .url("대충 플레이리스트 링크")
                .thumbnail("대충 플레이리스트 썸넬")
                .author(|a| {
                    a.name("대충 플레이리스트 만든놈 채널 이름")
                        .url("대충 플레이리스트 채널 링크")
                        .icon_url("대충 플레이리스트 채널 프사 이미지 링크")
                })
                .footer(|f| {
                    f.text(format!("requested by {}", "대충 요청한놈 이름"))
                        .icon_url("대충 요청한놈 프사 이미지 링크")
                });
        }
        false => {
            e.title("대충 노래 제목")
                .url("대충 노래 링크")
                .thumbnail("대충 썸네일 이미지 주소")
                .author(|a| {
                    a.name("대충 영상 채널 이름")
                        .url("대충 영상 채널 주소")
                        .icon_url("대충 영상 채널 아이콘 주소")
                })
                .footer(|f| {
                    f.text(format!("requested by {}", "대충 요청한놈 이름"))
                        .icon_url("대충 요청한놈 프사 이미지 링크")
                });
        }
    }

    e
}
