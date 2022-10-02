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

use crate::command_handler::commands;

pub enum CommandReturnValue {
    SingleString(String),
    SingleEmbed(CreateEmbed),
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
        _ => CommandReturnValue::SingleString("좆까".to_string()),
    };

    if let Err(why) = command
        .create_interaction_response(&ctx.http, |res| {
            res.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| match cmd_result {
                    CommandReturnValue::SingleString(content) => msg.content(content),
                    CommandReturnValue::SingleEmbed(embed) => msg.set_embed(embed),
                })
        })
        .await
    {
        println!(
            "Failed to send interaction responce : here's why\n {:?}",
            why
        );
    }
}
