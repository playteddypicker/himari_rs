use crate::command_handler::command_handler::CommandReturnValue;
use crate::command_handler::command_handler::DefaultCommandMethods;
use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::{
    permissions::Permissions, prelude::interaction::application_command::CommandDataOption,
};

pub struct Start;

#[async_trait]
impl DefaultCommandMethods for Start {
    async fn run(_ctx: &Context, _options: &[CommandDataOption]) -> CommandReturnValue {
        CommandReturnValue::SingleString(String::from("ㄴㄱㅁ"))
    }

    fn name() -> String {
        String::from("start")
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("start").description("뭐이씨발아")
    }
}
