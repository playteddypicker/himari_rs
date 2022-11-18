use crate::command_handler::{
    assign_command::CommandInterface, command_handler::CommandReturnValue,
};

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        permissions::Permissions, prelude::interaction::application_command::CommandDataOption,
    },
};

use rand::Rng;

struct SaySomething;

//나중에 json으로 따로 저장..
const HIMARI_SCRIPT_LIST: [&'static str; 5] = [
    "후훗. 기다리고 있었습니다, 선생님.",
    "⋯⋯아. 지금 데이터를 분석 중이니 잠시만 기다려주세요.",
    "데카그라마톤⋯⋯ 성가신 상대입니다.",
    "밀레니엄의 초 천재 병약 미소녀 해커가 당도했답니다.. 후훗.",
    "버러지 컽~!",
];

const MARI_SCRIPT_LIST: [&'static str; 9] = [
    "오늘도 좋은 하루를 보내실 수 있기를 기도할게요.",
    "괴로울 때는 저에게 의지하셔도 괜찮아요.",
    "(저의 기도가 선생님을 지킬 수 있기를...)",
    "지옥은 있습니다 예수님 믿고 천국가세요",
    "날씨도쌀쌀하고모두감기조심하세요몸\n이아프다면병원보다는성당으로가서기\n도를드립시다",
    "건전한 정신을 육성하기 위해 육체를 가꾼다...그 가는 길에 가호가 함께하기를.",
    "평소보다 이래저래 노출이 많은 모습이라...조금 부끄러운 기분이에요...",
    "선생님...저, 괜찮으시다면 곁에 있어도 될까요?",
    "선생님께 부디 가호가 함께하기를...기도에 복장은 상관없답니다.",
];

pub fn get_command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(SaySomething)
}

#[async_trait]
impl CommandInterface for SaySomething {
    async fn run(
        &self,
        ctx: &Context,
        _options: &[CommandDataOption],
        _command: &ApplicationCommandInteraction,
    ) -> CommandReturnValue {
        return match ctx.cache.current_user().name.as_str() {
            "히마리" => CommandReturnValue::SingleString(
                HIMARI_SCRIPT_LIST[rand::thread_rng().gen_range(0..HIMARI_SCRIPT_LIST.len())]
                    .to_string(),
            ),
            "마리" => CommandReturnValue::SingleString(
                MARI_SCRIPT_LIST[rand::thread_rng().gen_range(0..MARI_SCRIPT_LIST.len())]
                    .to_string(),
            ),
            _ => CommandReturnValue::SingleString("이이잉".to_string()),
        };
    }

    fn name(&self) -> String {
        String::from("아무말")
    }

    fn register<'a: 'b, 'b>(
        &self,
        command: &'a mut CreateApplicationCommand,
    ) -> &'b mut CreateApplicationCommand {
        command
            .name("아무말")
            .description("후후후..")
            .default_member_permissions(Permissions::SEND_MESSAGES | Permissions::ADD_REACTIONS)
    }
}
