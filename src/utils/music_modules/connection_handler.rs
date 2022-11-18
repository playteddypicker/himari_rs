use super::enqueue::enqueue_main;
use super::queue_event_handler;
use crate::command_handler::command_handler::CommandReturnValue;
use crate::utils::music_modules::stream_handler;
use crate::utils::structures::guild_queue::PlayStatus;
use crate::GuildQueueType;

use lavalink_rs::LavalinkClient;

use serenity::{
    client::Context,
    model::{
        guild::{Guild, Member},
        id::{ChannelId, GuildId, UserId},
    },
};

use songbird::input::Input;
use songbird::Call;
use songbird::Songbird;

use log::error;

enum ConnectionErrorCode {
    JoinVoiceChannelFirst,
    AlreadyConnectedOtherChannel(String),
    CannotFoundVoiceChannelInfo,
    CannotFoundServerInfo,
}

/* 언제쓰지 ?
enum ConnectionSuccessType {
    AlreadyConnected,
    NewConnect,
}*/

//요청한놈 유저 정보 저장하는 구조체
pub struct RequestInfo {
    pub channel_id: ChannelId,
    pub member_name: String,
    pub member_avatar_url: String,
}

pub enum RequestType {
    Command,
    PlayerChannel,
    PlaylistCommand,
}

fn errorcode_wraping(errcode: ConnectionErrorCode) -> String {
    return match errcode {
        ConnectionErrorCode::JoinVoiceChannelFirst => {
            "⚠️ 먼저 음성 채널에 들어가주세요.".to_string()
        }
        ConnectionErrorCode::AlreadyConnectedOtherChannel(ch_id) => {
            format!("이미 저는 {}에서 스트리밍 중입니다.", ch_id)
        }
        ConnectionErrorCode::CannotFoundServerInfo => {
            "❔ 서버 정보를 찾을 수 없습니다.".to_string()
        }
        ConnectionErrorCode::CannotFoundVoiceChannelInfo => {
            "❔ 음성 채널 정보를 찾을 수 없습니다.".to_string()
        }
    };
}

//stream함수
//먼저 유저가 음성 채널에 들어가있는지 검사
pub async fn connection_main(
    uid: &UserId,
    gid: GuildId,
    ctx: &Context,
    search_query: (String, bool),
    request_type: RequestType,
) -> Option<CommandReturnValue> {
    //연결 확인
    let voice_manager = songbird::get(ctx).await.unwrap();

    //GuildQueue 확인
    let counter = {
        let r = ctx.data.read().await;
        r.get::<GuildQueueType>().expect("poisoned data").clone()
    };
    let mut guilds = counter.write().await;
    let gq = guilds.get_mut(&gid.0).unwrap();

    return match connection_filter(&uid, &ctx.cache.guild(&gid), &voice_manager).await {
        //노래를 틀어도 된다고 판단하면
        Ok(res) => {
            let (voice_node, voice_check) = voice_manager.join(gid.clone(), res.channel_id).await;
            {
                queue_event_handler::add_current_event(
                    &voice_node,
                    gq,
                    res.channel_id.clone(),
                    &ctx,
                )
                .await;
            }

            return match voice_check {
                Ok(_) => match enqueue_main(gq, res, search_query, request_type).await {
                    //enqueue를 완료했을때
                    Ok(result_cmdvalue) => {
                        if let PlayStatus::Idle = gq.play_status {
                            if gq.queue.len() > 0 {
                                stream_handler::start_stream(gq, ctx, voice_node).await;
                            }
                        }
                        result_cmdvalue
                    }
                    Err(errmsg) => Some(CommandReturnValue::SingleString(errmsg)),
                },
                Err(why) => {
                    error!("{:#?}", why);
                    Some(CommandReturnValue::SingleString(
                        "⚠️ 음성 채널에 연결하는데 오류가 발생했습니다. 나중에 다시 시도해주세요."
                            .to_string(),
                    ))
                }
            };
        }
        //연결하는데 오류나면
        Err(errcode) => Some(CommandReturnValue::SingleString(errorcode_wraping(errcode))),
    };
}

//가독성 챙기기 vs 극한의 함수형 코딩
/*async fn connection_filter1(
    uid: &UserId,
    guild: &Option<Guild>,
    voice_manager: &Songbird,
) -> Result<RequestInfo, ConnectionErrorCode> {
    //Cache로부터 불러온 서버 정보가 제대로 불러와졌는지 체크
    return guild.ok_or(ConnectionErrorCode::CannotFoundServerInfo).and_then(|g| {
        g.voice_states.get(uid).and_then(|vs| vs.channel_id).ok_or(|usr_chid| {

        })
    }),


    //유저가 음성채널에 없을때
    if let None = guild.
}
*/
async fn connection_filter(
    uid: &UserId,
    guild: &Option<Guild>,
    voice_manager: &Songbird,
) -> Result<RequestInfo, ConnectionErrorCode> {
    //Cache로부터 불러온 서버 정보가 제대로 불러와졌는지에 따라 체크
    return match guild {
        //서버 정보 불러와짐
        Some(g) => {
            match g.voice_states.get(uid).and_then(|vs| vs.channel_id) {
                //유저가 음성채널에 있을때
                Some(user_ch_id) => {
                    let mem = g.members.get(uid);
                    //봇의 음성채널 정보에 따라 구분
                    match voice_manager.get(g.id) {
                        //음성채널에 이미 들어가있으면
                        Some(call) => {
                            let locked_call = call.lock().await;
                            match locked_call.current_channel() {
                                //음성 채널 정보를 성공적으로 불러왔다면
                                Some(bot_ch_id) => {
                                    //봇하고 같은 채널에 있으면
                                    if bot_ch_id.0 == user_ch_id.0 {
                                        Ok(get_request_info(user_ch_id, mem))
                                    //봇이 다른 채널에 있으면 ntr못함
                                    } else {
                                        Err(ConnectionErrorCode::AlreadyConnectedOtherChannel(
                                            format!("<#{}>", bot_ch_id.0),
                                        ))
                                    }
                                }
                                //음성 채널 정보를 불러오는데 실패하면
                                None => Err(ConnectionErrorCode::CannotFoundVoiceChannelInfo),
                            }
                        }
                        //음성채널에 연결되어있지 않으면
                        None => Ok(get_request_info(user_ch_id, mem)),
                    }
                }
                //유저가 음성채널에 없을때
                None => Err(ConnectionErrorCode::JoinVoiceChannelFirst),
            }
        } //서버 정보가 안불러와짐
        None => Err(ConnectionErrorCode::CannotFoundServerInfo),
    };
}

fn get_request_info(chid: ChannelId, mem: Option<&Member>) -> RequestInfo {
    RequestInfo {
        channel_id: chid,
        member_name: match mem {
            Some(m) => m.display_name().to_string(),
            None => "NotFound".to_string(),
        },
        member_avatar_url: match mem {
            Some(m) => match m.avatar_url() {
                Some(u) => u,
                None => "https://i.redd.it/j3t4cvgywd051.png".to_string(),
            },
            None => "https://i.redd.it/j3t4cvgywd051.png".to_string(),
        },
    }
}
