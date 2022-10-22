//얘는 CommandInteraction event가 발생했을때 실행됨
//Command를 만들어서 등록하거나 직접적으로 실행하는 함수를 전달하는건 assign_command.rs에서 함

use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        id::GuildId,
        prelude::interaction::application_command::CommandDataOption,
    },
};

use log::{error, info, warn};

use super::assign_command::COMMAND_LIST;
use crate::utils::frameworks::button_embeds;
use crate::utils::frameworks::reaction_pages;

use std::cell::RefCell;

pub enum CommandReturnValue {
    SingleString(String),
    SingleEmbed(CreateEmbed),
    ReactionPages(RefCell<Vec<CreateEmbed>>),
    MultiEmbedFramework(), //매개변수 뭘로할지 생각중
}

//커맨드 실행
pub async fn seperate_command(command: ApplicationCommandInteraction, ctx: &Context) {
    //이렇게 일일이 등록 안하고 자동으로 가게끔 HashMap형태로 구현하기
    //trait object를 이용해서 구현 완료
    //trait 선언 후 여러 명령어를 dyn으로 묶어서 get_command에서 dyn CommandInterface형태로 리턴함
    let cmd_result = match COMMAND_LIST.commands.get(command.data.name.as_str()) {
        Some(result) => result.run(&ctx, &command.data.options).await,
        None => CommandReturnValue::SingleString("아직 구현되지 않은 명령어입니다.".to_string()),
    };

    match cmd_result {
        //단순 문자열 응답일때
        CommandReturnValue::SingleString(content) => {
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.content(&content))
                })
                .await
            {
                error!(
                    "Failed to send Single-string \"{:?}\" from command \"{}\".",
                    content, command.data.name
                );
                error!("{:#?}", why);
            }
        }
        //단순 임베드 하나 응답일 때
        CommandReturnValue::SingleEmbed(embed) => {
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| msg.set_embed(embed.clone()))
                })
                .await
            {
                error!(
                    "Failed to send single-embed \"{:#?}\" from command \"{}\".",
                    embed, command.data.name
                );
                error!("{:#?}", why);
            }
        }
        //skippable한 페이지식 임베드로 되어있을때
        CommandReturnValue::ReactionPages(embeds) => {
            let unwrap_embeds = embeds.into_inner();
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|msg| {
                            msg.set_embed(unwrap_embeds.clone()[0].clone())
                        })
                })
                .await
            {
                error!(
                    "Failed to send reactive-embed \"{:#?}\" from command \"{}\".",
                    unwrap_embeds, command.data.name
                );
                error!("{:#?}", why);
            }
            //여기서 발생한 ApplicationCommandInteraction은 더이상 편집 외엔 사용 안함
            //그러므로 reaction_pages에 소유권 자체를 넘기면 해결
            if let Err(why) = reaction_pages::reaction_pages(command, &ctx, unwrap_embeds).await {
                error!("an error occured while handling reactionable page.");
                error!("{:#?}", why);
            }
        }
        //버튼으로 사이트마냥 돌아다닐수 있는 동적 임베드로 되어있을 때
        CommandReturnValue::MultiEmbedFramework() => {}
    }
}
