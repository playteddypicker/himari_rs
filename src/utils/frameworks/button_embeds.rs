use serenity::{
    async_trait,
    builder::{CreateActionRow, CreateButton, CreateEmbed},
    model::application::component::ButtonStyle,
};

use crate::utils::frameworks::reaction_pages::SkippableEmbed;
use std::collections::HashMap;
use std::collections::VecDeque;

enum NodeType {
    SingleEmbed(CreateEmbed),
    PageEmbed(Vec<CreateEmbed>),
}

/*
 * Root embed : {
 *  main embed : Single
 *  buttons: max to 5
 *  buttons are not have functions. just pages.
 * }
 *
 * node embed : {
 *  main embed: Single or Page
 *   { if type of main embed is page, must contain skippable buttons. }
 *  to other embed buttons: max to 4 for each child : Optional, but must contain back button
 *  function buttons: max to 5 for each child : Optional
 * }
 */
/*
#[async_trait]
trait FunctionButtonExecute {
    async fn execute() -> Result<(), serenity::Error>;
}

struct FunctionButton {
    button: CreateButton,
}

impl<T> NodeEmbed<T>
where
    T: FunctionButtonExecute,
{
    pub async fn execute() {}
}

struct NodeEmbed<T: FunctionButtonExecute> {
    id: String,
    embed: NodeType,
    page_buttons: Vec<String>,
    function_buttons: FunctionButton,
}

struct EmbedTree {
    root_embed: CreateEmbed,
    root_buttons: Vec<CreateButton>,
    back_stack: VecDeque<String>, //Id 넣기
    node_list: HashMap<String, NodeEmbed<T>>,
}*/
