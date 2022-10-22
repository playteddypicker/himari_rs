use crate::command_handler::{
    assign_command::CommandInterface, command_handler::CommandReturnValue,
};

use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateEmbed},
    client::Context,
    model::{
        permissions::Permissions, prelude::interaction::application_command::CommandDataOption,
    },
};
use std::cell::RefCell;

struct ReactionTest;

pub fn get_command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(ReactionTest)
}

#[async_trait]
impl CommandInterface for ReactionTest {
    async fn run(&self, _ctx: &Context, _options: &[CommandDataOption]) -> CommandReturnValue {
        let mut embeds = Vec::new();
        let mut page1 = CreateEmbed::default();
        page1.title("title1").description("asdfasdf");

        let mut page2 = CreateEmbed::default();
        page2.title("title2").description("asdfasdf");

        embeds.push(page1);
        embeds.push(page2);

        CommandReturnValue::ReactionPages(RefCell::new(embeds))
    }

    fn name(&self) -> String {
        "reactiontest".to_string()
    }

    fn register<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand {
        command
            .name("reactiontest")
            .description("리액션페이지 테스트용")
    }
}
