use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        application::interaction::{Interaction, InteractionResponseType},
        gateway::Ready,
        guild::Guild,
        id::GuildId,
    },
    prelude::TypeMapKey,
};

use log::{error, info, warn};

use crate::{
    command_handler::{assign_command, command_handler},
    utils::structures::guild_queue,
};

pub struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is now connected.", ready.user.tag());
        guild_queue::load_guild_multi(&ctx).await;
    }

    async fn guild_create(&self, ctx: Context, guild: Guild, is_new: bool) {
        if is_new {
            //봇이 초대될때만 발생
            info!("new guild added: {} (ID: {})", guild.name, guild.id);
            //봇이 추가되면 start라는 커맨드 등록함. start는 초기 설정용
            if let Err(_) = guild
                .id
                .create_application_command(&ctx.http, |c| {
                    c.name("start").description("이 서버에 명령어를 등록해요")
                })
                .await
            {
                error!("error occured while creating 'start' command.");
            }
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::ApplicationCommand(command) => match command.data.name.as_str() {
                "start" => assign_command::start_command(command, &ctx).await,
                _ => command_handler::seperate_command(command, &ctx).await,
            },
            Interaction::MessageComponent(component) => {}
            _ => {
                warn!("Interaction created, but not handled on this case.");
            }
        }
    }
}
