use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        permissions::Permissions,
        prelude::interaction::application_command::CommandDataOption,
    },
};

use log::{error, info, warn};

use crate::command_handler::commands;
use crate::utils::frameworks::reaction_pages;

use std::cell::RefCell;

pub enum CommandReturnValue {
    SingleString(String),
    SingleEmbed(CreateEmbed),
    ReactionPages(RefCell<Vec<CreateEmbed>>),
    MultiEmbedFramework(), //매개변수 뭘로할지 생각중
}

#[async_trait]
pub trait DefaultCommandMethods {
    async fn run(ctx: &Context, options: &[CommandDataOption]) -> CommandReturnValue;
    fn name() -> String;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}

#[async_trait]
pub trait InteractiveCommandMethods {
    async fn run(ctx: &Context, options: &[CreateApplicationCommand]) -> CommandReturnValue;
}

//커맨드 실행
pub async fn seperate_command(command: ApplicationCommandInteraction, ctx: &Context) {
    let cmd_result = match command.data.name.as_str() {
        "start" => commands::start::Start::run(&ctx, &command.data.options).await,
        "아무말" => commands::saysomething::SaySomething::run(&ctx, &command.data.options).await,
        "reactiontest" => {
            commands::reactiontest::ReactionTest::run(&ctx, &command.data.options).await
        }
        _ => CommandReturnValue::SingleString("좆까".to_string()),
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
                error!("here's why: {:?}", why);
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
                error!("here's why: {:?}", why);
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
                error!("here's why: {:?}", why);
            };
            //여기서 발생한 ApplicationCommandInteraction은 더이상 편집 외엔 사용 안함
            //그러므로 reaction_pages에 소유권 자체를 넘기면 해결
            if let Err(why) = reaction_pages::reaction_pages(command, &ctx, unwrap_embeds).await {
                error!("an error occured while handling reactionable page.");
                error!("{:#?}", why);
            }
        }
        CommandReturnValue::MultiEmbedFramework() => {}
    }
}
