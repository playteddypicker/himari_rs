use crate::command_handler::{
    assign_command::CommandInterface, command_handler::CommandReturnValue,
};

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            command::CommandOptionType,
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        },
    },
};

use crate::utils::music_modules::{stream, stream::RequestType};

struct Play;

pub fn get_command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(Play)
}

#[async_trait]
impl CommandInterface for Play {
    async fn run(
        &self,
        ctx: &Context,
        options: &[CommandDataOption],
        command: &ApplicationCommandInteraction,
    ) -> CommandReturnValue {
        let gid = command.guild_id.unwrap();

        //옵션 2개는 무조건 존재함
        let search_string = if let CommandDataOptionValue::String(request) =
            options.get(0).unwrap().resolved.as_ref().unwrap()
        {
            Some(request.to_string())
        } else {
            None
        };

        //얘는 선택적 옵션임. 디폴트는 false로..
        let search_playlist = match options.get(1) {
            Some(x) => {
                if let CommandDataOptionValue::Boolean(pl_req) = x.resolved.as_ref().unwrap() {
                    *pl_req
                } else {
                    false
                }
            }
            None => false,
        };

        return stream::connection_main(
            &command.user.id,
            gid,
            &ctx,
            (search_string, search_playlist),
            RequestType::Command,
        )
        .await;
    }

    fn name(&self) -> String {
        "play".to_string()
    }

    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'b mut CreateApplicationCommand {
        command
            .name("play")
            .description("음악 재생/노래 추가 명령어")
            .create_option(|opt| {
                opt.name("request")
                    .description("재생할 노래의 제목이나 링크를 적어주세요")
                    .kind(CommandOptionType::String)
                    .max_length(200)
                    .required(true)
            })
            .create_option(|opt| {
                opt.name("isplaylist")
                    .description("플레이리스트만 검색할건지 설정해요")
                    .kind(CommandOptionType::Boolean)
                    .required(false)
            })
    }
}
