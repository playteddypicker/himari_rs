use super::{command_handler::CommandReturnValue, commands};
use std::collections::HashMap;

use lazy_static::lazy_static;
use log::error;

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        id::GuildId,
        prelude::interaction::application_command::CommandDataOption,
    },
};

#[async_trait]
pub trait CommandInterface {
    async fn run(&self, ctx: &Context, options: &[CommandDataOption]) -> CommandReturnValue;
    fn name(&self) -> String;
    fn register<'a>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &mut CreateApplicationCommand;
}

pub struct CommandList {
    pub commands: HashMap<&'static str, Box<dyn CommandInterface + Send + Sync>>,
}

impl CommandList {}

lazy_static! {
    pub static ref COMMAND_LIST: CommandList = CommandList {
        commands: HashMap::from([
            ("아무말", commands::saysomething::get_command()),
            ("reactiontest", commands::reactiontest::get_command())
        ])
    };
}

pub async fn start_command(command: ApplicationCommandInteraction, ctx: &Context) {}

//pub async fn run_command(cmd_name: &str) -> CommandReturnValue {}
