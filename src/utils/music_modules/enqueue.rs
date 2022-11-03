use serenity::builder::CreateEmbed;
use serenity::model::id::GuildId;

use super::stream::RequestType;

pub async fn enqueue_main(
    gid: GuildId,
    search_query: (Option<String>, bool),
    request_type: RequestType,
) -> Option<CreateEmbed> {
    None
}
