use super::assign_checker::{
    send_available_updates_msg, send_failed_error_msg, send_first_setup_msg,
    send_nothing_to_update_msg,
};
use super::{command_handler::CommandReturnValue, commands};
use std::collections::HashMap;

use lazy_static::lazy_static;
use log::error;

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction, id::GuildId,
        prelude::interaction::application_command::CommandDataOption,
    },
};

#[async_trait]
pub trait CommandInterface {
    async fn run(&self, ctx: &Context, options: &[CommandDataOption]) -> CommandReturnValue;
    fn name(&self) -> String;
    //수명을 맞춰줘야함. 참조해온 command가 먼저 소멸되어버리면 안되니까..
    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'b mut CreateApplicationCommand;
}

pub struct CommandList {
    //여러 스레드에서 참조할수 있도록 Send랑 Sync를 Box로 묶어서 걸어둠
    pub commands: HashMap<&'static str, Box<dyn CommandInterface + Send + Sync>>,
}

impl CommandList {
    pub async fn register_commands(&'static self, gid: GuildId, ctx: &Context) {
        for (_, cmd) in &self.commands {
            if let Err(why) = gid
                .create_application_command(&ctx.http, |c| cmd.register(c))
                .await
            {
                error!("Couldn't create application command: {:#?}", why);
            }
        }
    }
}

//명령어 만든거를 여기에 등록시킴
lazy_static! {
    pub static ref COMMAND_LIST: CommandList = CommandList {
        commands: HashMap::from([
            ("아무말", commands::saysomething::get_command()),
            ("reactiontest", commands::reactiontest::get_command())
        ])
    };
}

enum UpdateStatus {
    FirstSetting,
    UpdateAvailable(Vec<String>),
    LatestVersion,
    FailedtoLoad,
}

pub async fn start_command(command: ApplicationCommandInteraction, ctx: &Context) {
    //세가지 메시지 응답 타입
    //defer 후 응답하는식으로.. 검사하는데 오래걸릴수도 있음
    command.defer(&ctx.http).await.unwrap();

    match command.guild_id {
        Some(gid) => match check_updates(&ctx, gid).await {
            //1. 서버에서 처음 쓸때(커맨드가 아무것도 존재하지 않음. start밖에..)
            UpdateStatus::FirstSetting => send_first_setup_msg(&ctx, gid, command).await,
            //2. 처음 쓰는거 아닌데 업데이트가 있을때
            UpdateStatus::UpdateAvailable(unassigned_commands) => {
                send_available_updates_msg(&ctx, gid, unassigned_commands, command).await
            }
            //3. 처음 쓰는거도 아니고 업데이트가 없을때(최신버전)
            UpdateStatus::LatestVersion => send_nothing_to_update_msg(&ctx, command).await,
            //4. 서버 에러로 정보를 불러오지 못했을 때
            UpdateStatus::FailedtoLoad => send_failed_error_msg(&ctx, command).await,
        },
        None => send_failed_error_msg(&ctx, command).await,
    }
}

async fn check_updates(ctx: &Context, gid: GuildId) -> UpdateStatus {
    match gid.get_application_commands(&ctx.http).await {
        Ok(cmds) => {
            return if cmds.len() == 1 {
                UpdateStatus::FirstSetting
            } else if cmds.len() == COMMAND_LIST.commands.len() + 1 {
                UpdateStatus::LatestVersion
            } else {
                //아직 등록되지 않은 명령어 목록을 parameter로 전달
                let mut unassigned_commands = Vec::new();
                let cmdname_lists = cmds.iter().map(|c| c.name.clone()).collect::<Vec<String>>();
                for (cmdname, _) in COMMAND_LIST.commands.iter() {
                    if !cmdname_lists.contains(&cmdname.to_string()) {
                        unassigned_commands.push(cmdname.to_string());
                    }
                }
                UpdateStatus::UpdateAvailable(unassigned_commands)
            };
        }
        Err(why) => {
            error!(
                "Failed to get application data from {}. \nwhy: {:#?}",
                gid, why
            );
            return UpdateStatus::FailedtoLoad;
        }
    }
}
