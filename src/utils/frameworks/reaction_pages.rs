use serenity::{
    async_trait,
    builder::{CreateEmbed, CreateInteractionResponseData},
    model::{channel::Embed, id::InteractionId},
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
    fn next(&self) {
        self.current_idx = if self.current_idx + 1 < self.total {
            self.current_idx + 1
        } else {
            self.current_idx
        }
    }

    fn prev(&self) {
        self.current_idx = if self.current_idx > 0 {
            self.current_idx - 1
        } else {
            self.current_idx
        }
    }

    fn skip_end(&self) {
        self.current_idx = self.total - 1;
    }

    fn skip_start(&self) {
        self.current_idx = 0;
    }

    fn check_disable_button(&self) {
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

pub fn reaction_pages(
    msg: &'static mut CreateInteractionResponseData<'static>,
    interaction_id: &InteractionId,
    embeds: Vec<CreateEmbed>,
) -> &'static mut CreateInteractionResponseData<'static> {
    let pages = SkippableEmbed {
        id: *interaction_id,
        total: embeds.len(),
        current_idx: 0,
        embed_list: embeds,
        button_disable_option: (true, true, true, true),
    };

    msg.set_embed(pages.embed_list[pages.current_idx])
}
