use crate::command_handler::{
    assign_command::CommandInterface, command_handler::CommandReturnValue,
};

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        permissions::Permissions, prelude::interaction::application_command::CommandDataOption,
    },
};

use rand::Rng;

struct SaySomething;

const SAY_SOMETHING_LIST: [&'static str; 4] = [
    "후훗. 기다리고 있었습니다, 선생님.",
    "⋯⋯아. 지금 데이터를 분석 중이니 잠시만 기다려주세요.",
    "데카그라마톤⋯⋯ 성가신 상대입니다.",
    "버러지 컽~!",
];

pub fn get_command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(SaySomething)
}

#[async_trait]
impl CommandInterface for SaySomething {
    async fn run(&self, _ctx: &Context, _options: &[CommandDataOption]) -> CommandReturnValue {
        CommandReturnValue::SingleString(
            SAY_SOMETHING_LIST[rand::thread_rng().gen_range(0..4)].to_string(),
        )
    }

    fn name(&self) -> String {
        String::from("아무말")
    }

    fn register<'a>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'a mut CreateApplicationCommand {
        command
            .name("아무말")
            .description("후후후..")
            .default_member_permissions(Permissions::SEND_MESSAGES | Permissions::ADD_REACTIONS)
    }
}
