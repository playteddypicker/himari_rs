use crate::command_handler::command_handler::{CommandReturnValue, DefaultCommandMethods};
use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        permissions::Permissions, prelude::interaction::application_command::CommandDataOption,
    },
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
