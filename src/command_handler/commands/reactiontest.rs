use crate::command_handler::command_handler::{CommandReturnValue, DefaultCommandMethods};

use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::{
        permissions::Permissions, prelude::interaction::application_command::CommandDataOption,
    },
};
use std::cell::RefCell;

pub struct ReactionTest;

#[async_trait]
impl DefaultCommandMethods for ReactionTest {
    async fn run(_ctx: &Context, _options: &[CommandDataOption]) -> CommandReturnValue {
        let mut embeds = Vec::new();
        let mut page1 = CreateEmbed::default();
        page1.title("title1").description("asdfasdf");

        let mut page2 = CreateEmbed::default();
        page2.title("title2").description("asdfasdf");

        embeds.push(page1);
        embeds.push(page2);

        CommandReturnValue::ReactionPages(RefCell::new(embeds))
    }

    fn name() -> String {
        "reactiontest".to_string()
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("reactiontest")
            .description("리액션페이지 테스트용")
    }
}
