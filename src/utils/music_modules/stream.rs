use crate::command_handler::command_handler::CommandReturnValue;
use crate::utils::music_modules::enqueue::enqueue_main;

use lavalink_rs::LavalinkClient;

use serenity::{
    client::Context,
    model::{
        guild::Guild,
        id::{ChannelId, GuildId, UserId},
    },
};

use songbird::Songbird;

use log::error;

enum ConnectionErrorCode {
    JoinVoiceChannelFirst,
    AlreadyConnectedOtherChannel,
    CannotFoundVoiceChannelInfo,
    CannotFoundServerInfo,
}

/* 언제쓰지 ?
enum ConnectionSuccessType {
    AlreadyConnected,
    NewConnect,
}*/

pub enum RequestType {
    Command,
    PlayerChannel,
    PlaylistCommand,
}

//stream함수
//먼저 유저가 음성 채널에 들어가있는지 검사

pub async fn connection_main(
    uid: &UserId,
    gid: GuildId,
    ctx: &Context,
    search_query: (Option<String>, bool),
    request_type: RequestType,
) -> CommandReturnValue {
    //연결 확인
    let voice_manager = songbird::get(ctx).await.unwrap();
    return match connection_filter(&uid, &ctx.cache.guild(&gid), &voice_manager).await {
        //노래를 틀어도 된다고 판단하면
        Ok(ch) => {
            match voice_manager.join(gid.clone(), ch).await.1 {
                Ok(_) => {
                    return match enqueue_main(gid, search_query, request_type).await {
                        Some(result_embed) => CommandReturnValue::SingleEmbed(result_embed),
                        None => CommandReturnValue::SingleString(
                            "조건에 맞는 검색 결과가 없습니다.".to_string(),
                        ),
                    }
                }
                Err(why) => {
                    error!("Connecting Voice Channel Error: {:#?}", why);
                    return CommandReturnValue::SingleString(
                        "🔌 연결하는데 오류가 발생했습니다. 나중에 다시 시도해주세요.".to_string(),
                    );
                }
            };
        }
        //연결하는데 오류나면
        Err(errcode) => match errcode {
            ConnectionErrorCode::JoinVoiceChannelFirst => {
                CommandReturnValue::SingleString("⚠️ 먼저 음성 채널에 들어가주세요.".to_string())
            }
            ConnectionErrorCode::AlreadyConnectedOtherChannel => CommandReturnValue::SingleString(
                "⚠️ 이미 다른 채널에 연결되어 있습니다. ".to_string(),
            ),
            ConnectionErrorCode::CannotFoundServerInfo => {
                CommandReturnValue::SingleString("❔ 서버 정보를 찾을 수 없습니다.".to_string())
            }
            ConnectionErrorCode::CannotFoundVoiceChannelInfo => CommandReturnValue::SingleString(
                "❔ 음성 채널 정보를 찾을 수 없습니다.".to_string(),
            ),
        },
    };
}

async fn connection_filter(
    uid: &UserId,
    guild: &Option<Guild>,
    voice_manager: &Songbird,
) -> Result<ChannelId, ConnectionErrorCode> {
    //Cache로부터 불러온 서버 정보가 제대로 불러와졌는지에 따라 체크
    return match guild {
        //서버 정보 불러와짐
        Some(g) => match g.voice_states.get(uid).and_then(|vs| vs.channel_id) {
            //유저가 음성채널에 있을때
            Some(user_ch_id) => {
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
                                    Ok(user_ch_id)
                                //봇이 다른 채널에 있으면 ntr못함
                                } else {
                                    Err(ConnectionErrorCode::AlreadyConnectedOtherChannel)
                                }
                            }
                            //음성 채널 정보를 불러오는데 실패하면
                            None => Err(ConnectionErrorCode::CannotFoundVoiceChannelInfo),
                        }
                    }
                    //음성채널에 연결되어있지 않으면
                    None => Ok(user_ch_id),
                }
            }
            //유저가 음성채널에 없을때
            None => Err(ConnectionErrorCode::JoinVoiceChannelFirst),
        }, //서버 정보가 안불러와짐
        None => Err(ConnectionErrorCode::CannotFoundServerInfo),
    };
}
