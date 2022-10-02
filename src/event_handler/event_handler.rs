use crate::{
    command_handler::{command_handler, command_handler::DefaultCommandMethods, commands},
    event_handler::events,
};

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::interaction::{Interaction, InteractionResponseType},
        gateway::Ready,
        guild::Guild,
        id::GuildId,
    },
};

pub struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is now connected.", ready.user.tag());
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        if is_new {
            //봇이 초대될때만 발생
            println!("new guild added: {} (ID: {})", guild.name, guild.id);
            //봇이 추가되면 start라는 커맨드 등록함. start는 초기 설정용
            if let Err(_) = GuildId::set_application_commands(&guild.id, &ctx.http, |cmds| {
                cmds.create_application_command(|c| commands::start::Start::register(c))
                    .create_application_command(|c| {
                        commands::saysomething::SaySomething::register(c)
                    })
            })
            .await
            {
                println!("error occured while creating 'start' command.");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => {
                command_handler::seperate_command(command, &ctx).await;
            }
            _ => {
                println!("asdf");
            }
        }
    }
}
