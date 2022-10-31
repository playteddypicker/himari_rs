use serenity::{
    builder::{CreateActionRow, CreateEmbed},
    client::Context,
    futures::StreamExt,
    model::{
        application::{
            component::ButtonStyle,
            interaction::{
                application_command::ApplicationCommandInteraction, InteractionResponseType,
            },
        },
        channel::ReactionType,
        id::GuildId,
    },
};

use super::assign_command::COMMAND_LIST;

use log::error;

use std::time::Duration;

fn setup_embed(return_embed: &mut CreateEmbed) -> &mut CreateEmbed {
    return_embed
        .title("이 서버에는 명령어가 아직 등록되어 있지 않아요")
        .description("밑에 등록 버튼을 눌러 서버에 있는 명령어를 불러와 등록할 수 있어요")
        .color((253, 218, 164))
}

fn setup_components(pressed: bool) -> CreateActionRow {
    let mut row = CreateActionRow::default();

    row.create_button(|b| {
        b.custom_id("setup_assign")
            .label("등록")
            .style(ButtonStyle::Primary)
            .disabled(pressed)
    })
    .create_button(|b| {
        b.custom_id("setup_help")
            .emoji("❔".parse::<ReactionType>().unwrap())
            .style(ButtonStyle::Secondary)
            .disabled(pressed)
    })
    .create_button(|b| {
        b.custom_id("setup_cancel")
            .label("안할래")
            .style(ButtonStyle::Danger)
            .disabled(pressed)
    });

    row
}

fn update_components(pressed: bool) -> CreateActionRow {
    let mut row = CreateActionRow::default();

    row.create_button(|b| {
        b.custom_id("update_assign")
            .label("추가할래")
            .style(ButtonStyle::Primary)
            .disabled(pressed)
    })
    .create_button(|b| {
        b.custom_id("update_help")
            .label("패치노트 보기")
            .style(ButtonStyle::Secondary)
            .disabled(pressed)
    })
    .create_button(|b| {
        b.custom_id("update_cancel")
            .label("안할래")
            .style(ButtonStyle::Danger)
            .disabled(pressed)
    });

    row
}

//나중에 button framework로 단순화
pub async fn send_first_setup_msg(
    ctx: &Context,
    gid: GuildId,
    command: ApplicationCommandInteraction,
) {
    //먼저 안내용 임베드하고 버튼먼저 보냄
    //defer되어있으니 edit_original_interaction_response로 해야함
    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |msg| {
            msg.embed(|e| setup_embed(e))
                .components(|c| c.set_action_row(setup_components(false)))
        })
        .await
    {
        error!("Failed to response slash command: {:#?}", why);
    };

    //버튼 누를때까지 기다림
    match command.get_interaction_response(&ctx.http).await {
        Ok(msg) => {
            let mut interaction_stream = msg
                .await_component_interactions(&ctx)
                .timeout(Duration::from_secs(60 * 5))
                .filter(move |f| {
                    f.message.id == msg.id
                        //is_some_and 업뎃 후 코드를 다음과 같이 변경
                        // f.member.is_some_and(|&m| m.user.id == interaction.user.id)
                        && f.member.as_ref().unwrap().user.id == command.user.id
                })
                .build();

            if let Some(button_reaction) = interaction_stream.next().await {
                match button_reaction.data.custom_id.as_str() {
                    "setup_assign" => {
                        match button_reaction
                            .create_interaction_response(&ctx.http, |r| {
                                r.kind(InteractionResponseType::UpdateMessage)
                                    .interaction_response_data(|res| {
                                        res.embed(|e| e.title("서버로부터 명령어를 등록하는 중..."))
                                            .components(|c| {
                                                c.set_action_row(setup_components(true))
                                            })
                                    })
                            })
                            .await
                        {
                            Ok(_) => {
                                //register_commands 메소드로 전부 등록
                                COMMAND_LIST.register_commands(gid, &ctx).await;
                                if let Err(why) = button_reaction
                                    .edit_original_interaction_response(&ctx, |msg| {
                                        msg.embed(|e| e.title("명령어 등록이 완료되었습니다."))
                                    })
                                    .await
                                {
                                    error!("Couldn't send complete msg. {:#?}", why);
                                }
                            }
                            Err(why) => {
                                error!("Couldn't edit response msg., {:#?}", why);
                            }
                        }
                    }
                    "setup_help" => {
                        //패치노트 임베드 보내기
                    }
                    "update_cancel" | _ => {
                        command
                            .delete_original_interaction_response(&ctx.http)
                            .await
                            .unwrap();
                    }
                }
            }
        }
        Err(why) => {
            error!("Couldn't get message info from interaction.\n{:#?}", why);
        }
    }
}

pub async fn send_available_updates_msg(
    ctx: &Context,
    gid: GuildId,
    unassigned_commands: Vec<String>,
    command: ApplicationCommandInteraction,
) {
    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |msg| {
            msg.embed(|e| {
                e.title("ℹ️ 아직 등록되지 않은 명령어가 있습니다")
                    .description(unassigned_commands.join("\n"))
            })
            .components(|c| c.set_action_row(update_components(false)))
        })
        .await
    {
        error!("Failed to response slash command: {:#?}", why);
    };

    //버튼 누를때까지 기다림
    match command.get_interaction_response(&ctx.http).await {
        Ok(mut msg) => {
            let mut interaction_stream = msg
                .await_component_interactions(&ctx)
                .timeout(Duration::from_secs(60 * 5))
                .filter(move |f| {
                    f.message.id == msg.id
                        //is_some_and 업뎃 후 코드를 다음과 같이 변경
                        // f.member.is_some_and(|&m| m.user.id == interaction.user.id)
                        && f.member.as_ref().unwrap().user.id == command.user.id
                })
                .build();

            if let Some(button_reaction) = interaction_stream.next().await {
                match button_reaction.data.custom_id.as_str() {
                    "update_assign" => {
                        match button_reaction
                            .create_interaction_response(&ctx.http, |r| {
                                r.kind(InteractionResponseType::UpdateMessage)
                                    .interaction_response_data(|res| {
                                        res.embed(|e| e.title("서버로부터 명령어를 등록하는 중..."))
                                            .components(|c| {
                                                c.set_action_row(setup_components(true))
                                            })
                                    })
                            })
                            .await
                        {
                            Ok(_) => {
                                //register_commands 메소드로 전부 등록
                                COMMAND_LIST.register_commands(gid, &ctx).await;

                                if let Err(why) = msg
                                    .edit(&ctx.http, |m| {
                                        m.embed(|e| e.title("명령어 등록이 완료되었습니다."))
                                    })
                                    .await
                                {
                                    error!("Couldn't send complete msg. {:#?}", why);
                                }
                            }
                            Err(why) => {
                                error!("Couldn't edit response msg., {:#?}", why);
                            }
                        }
                    }
                    "update_help" => {
                        button_reaction
                            .create_interaction_response(&ctx.http, |r| {
                                r.kind(InteractionResponseType::UpdateMessage)
                                    .interaction_response_data(|res| {
                                        res.content("그런건 없다 게이야 ㅋㅋ")
                                    })
                            })
                            .await
                            .unwrap();
                    }
                    "update_cancel" | _ => {
                        command
                            .delete_original_interaction_response(&ctx.http)
                            .await
                            .unwrap();
                    }
                }
            }
        }
        Err(why) => {
            error!("Couldn't get message info from interaction.\n{:#?}", why);
        }
    }
}

pub async fn send_nothing_to_update_msg(ctx: &Context, command: ApplicationCommandInteraction) {
    //먼저 안내용 임베드하고 버튼먼저 보냄
    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |msg| {
            msg.content("✅ 이 서버에는 업데이트 할 명령어가 없습니다.")
        })
        .await
    {
        error!("Failed to response slash command: {:#?}", why);
    };
}

pub async fn send_failed_error_msg(ctx: &Context, command: ApplicationCommandInteraction) {
    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |msg| {
            msg.content("⚠️ 서버 정보를 불러오는 데 실패했습니다.")
        })
        .await
    {
        error!("Failed to reponse slash command: {:#?}", why);
    }
}
