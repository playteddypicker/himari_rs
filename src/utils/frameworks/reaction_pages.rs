use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateInteractionResponseData},
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        channel::Embed, id::InteractionId,
    },
};

struct SkippableEmbed {
    id: InteractionId,
    total: usize,
    current_idx: usize,
    embed_list: Vec<CreateEmbed>,
    button_disable_option: (bool, bool, bool, bool),
}

impl SkippableEmbed {
    //current_idx가 total보다 작을때만 발생함
    fn next(&mut self) {
        self.current_idx = if self.current_idx + 1 < self.total {
            self.current_idx + 1
        } else {
            self.current_idx
        }
    }

    fn prev(&mut self) {
        self.current_idx = if self.current_idx > 0 {
            self.current_idx - 1
        } else {
            self.current_idx
        }
    }

    fn skip_end(&mut self) {
        self.current_idx = self.total - 1;
    }

    fn skip_start(&mut self) {
        self.current_idx = 0;
    }

    fn check_disable_button(&mut self) {
        self.button_disable_option = if self.current_idx + 1 == self.total {
            // <<, <만 활성화
            (false, false, true, true)
        } else if self.current_idx == 0 {
            //>, >>만 활성화
            (true, true, false, false)
        } else if self.total == 0 {
            //넘길 페이지가 없으므로 전부 비활
            (true, true, true, true)
        } else {
            (false, false, false, false)
        }
    }
}

pub async fn reaction_pages(interaction: ApplicationCommandInteraction) -> Result<String, Err> {
    //interaction을 edit해서 먼저 button component를 붙이기
    //
    //나중에 multi-embed framework랑 안겹치게 custom id 설정함
    //
    //filter로 거름
    //
    //+ button interaction 계속 받기. 5분동안만 시간 지나면 Ok() 반환
    //
    //만약 받는도중 에러나면 바로 Err 반환
    //
    //command_handler에서 Err 반환된거 처리하기(사용자에게 에러 문구 띄우기)
}
