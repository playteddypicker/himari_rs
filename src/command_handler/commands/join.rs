use crate::command_handler::{
    assign_command::CommandInterface, command_handler::CommandReturnValue,
};

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::interaction::application_command::CommandDataOption,
    },
};

use log::error;

struct Join;

pub fn get_command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(Join)
}

#[async_trait]
impl CommandInterface for Join {
    async fn run(
        &self,
        ctx: &Context,
        _options: &[CommandDataOption],
        command: &ApplicationCommandInteraction,
    ) -> CommandReturnValue {
        let gid = command.guild_id.unwrap();

        CommandReturnValue::SingleString(format!(
            "{}",
            //유저가 들어가있는 음성채널 정보를 따옴
            match &ctx
                .cache
                .guild(&gid)
                .unwrap()
                .voice_states
                .get(&command.user.id)
                .and_then(|vs| vs.channel_id)
            {
                //음성채널에 들어가있으면
                Some(ch) => {
                    let manager = songbird::get(ctx).await.expect("asdf");
                    match manager.join(gid.clone(), *ch).await.1 {
                        Ok(_) => {
                            tokio::spawn(async move {
                                match songbird::ytdl("https://www.youtube.com/watch?v=S7x8p7JNTnI")
                                    .await
                                {
                                    Ok(src) => {
                                        let handle_manager = manager.get(gid).unwrap();
                                        let mut handler = handle_manager.lock().await;
                                        handler.play_source(src);
                                    }
                                    Err(why) => {
                                        error!("Err starting source: {:#?}", why);
                                    }
                                }
                            });
                            "연결되었습니다."
                        }
                        Err(why) => {
                            //음성채널에는 들어가있는데 연결하는데 실패했으면
                            error!("cannot join at voice channel: {:#?}", why);
                            "연결하는데 실패했습니다."
                        }
                    }
                }
                //음성채널에 들어가있지 않으면
                None => "먼저 음성 채널에 들어가주세요.",
            }
        ))
    }

    fn name(&self) -> String {
        "join".to_string()
    }

    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'b mut CreateApplicationCommand {
        command.name("join").description("음성 채널에 들어갑니다")
    }
}
