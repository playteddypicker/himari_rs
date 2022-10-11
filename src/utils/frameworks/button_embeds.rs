use serenity::{
    builder::{CreateActionRow, CreateButton, CreateEmbed},
    model::application::component::ButtonStyle,
};

use crate::utils::frameworks::reaction_pages::SkippableEmbed;
use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug)]
pub enum NodeType {
    SingleEmbed(CreateEmbed),
    PageEmbed(SkippableEmbed),
}

//root embed는 Id가 정해져있음
//back_stack에는 현재 방문한거를 스택 형태로 저장하고
//뒤로가기를 누르면 pop 해서 top에 있는거를 불러오는 식임
//버튼에 연결되어있는 다른 페이지는 해쉬맵으로 접근
pub struct EmbedTree {
    pub(crate) root_embed: EmbedTreeElement,
    pub(crate) root_action_row: CreateActionRow,
    pub(crate) tree: HashMap<String, EmbedTreeElement>,
    back_stack: VecDeque<String>, //embed_tree의 key만 넣기
}

//해쉬맵에는 키값에 따른 임베드가 쭉 나열되어있음
//한 임베드 객체에는 버튼이 가리키는 다음 임베드의 해쉬맵 키값이 저장되어있음
//버튼 커스텀 아이디는 키값과 다를수 있으니 따로 저장
#[derive(Debug)]
pub struct EmbedTreeElement {
    key: String,
    pub(crate) embed: NodeType,
    pub(crate) childs: Option<Vec<String>>,
    pub(crate) action_row: Option<CreateActionRow>, //있어도 되고 없어도 되고
}

/* EmbedTree 만드는 방식
 *
 * use crate::utils::framework::button_embeds::{NodeType, EmbedTree, EmbedTreeElement};
 *
 * async fn run(ctx: &Context, options: CommandDataOption) -> CommandReturnValue {
 *     let help_embed_tree = HashMap::new();
 *
 *     let help_main = EmbedTreeElement {
 *         key: "root".to_string(),
 *         embed: 대충 도움말 메인 페이지 임베드,
 *         childs: Some([
 *              //얘네는 전부 각자 키임
 *             "help_music".to_string(),
 *             "help_basic".to_string(),
 *             "help_others".to_string(),
 *             "help_admin".to_string()
 *             ]),
 *         action_row: Some(대충 childs에 따른 메인 페이지 버튼들),
 *     };
 *     help_embed_tree.insert("root".to_string(), help_main);
 *
 *     let help_music = EmbedTreeElement {
 *         key: "help_music",
 *         embed: 대충 뮤직페이지 도움말 임베드,
 *         childs: None,
 *         action_row: None,
 *     }
 *     help_embed_tree.insert(help_music.key.to_string(), help_music);
 *
 *     let help_tree = EmbedTree {
 *         root_embed: help_main_embed,
 *         root_action_row: help_main_button,
 *         tree: help_embed_tree,
 *         back_stack: Vec::new(),
 *     }
 *
 *     CommandReturnValue::MultiEmbedFramework(help_tree)
 * }
 */

pub async fn button_framework(embed_tree: EmbedTree) -> Result<(), serenity::Error> {
    Ok(())
}
