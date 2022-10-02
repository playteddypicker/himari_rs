use crate::command_handler::command_handler::CommandReturnValue;
use crate::command_handler::command_handler::DefaultCommandMethods;
use rand::Rng;
use serenity::async_trait;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::{
    permissions::Permissions, prelude::interaction::application_command::CommandDataOption,
};

pub struct SaySomething;

const SAY_SOMETHING_LIST: [&'static str; 4] = [
    "후훗. 기다리고 있었습니다, 선생님.",
    "⋯⋯아. 지금 데이터를 분석 중이니 잠시만 기다려주세요.",
    "데카그라마톤⋯⋯ 성가신 상대입니다.",
    "버러지 컽~!",
];

#[async_trait]
impl DefaultCommandMethods for SaySomething {
    async fn run(_ctx: &Context, _options: &[CommandDataOption]) -> CommandReturnValue {
        CommandReturnValue::SingleString(
            SAY_SOMETHING_LIST[rand::thread_rng().gen_range(0..4)].to_string(),
        )
    }

    fn name() -> String {
        String::from("아무말")
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command.name("아무말").description("후후후..")
    }
}
