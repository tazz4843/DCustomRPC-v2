use crate::ERROR_MESSAGE;
use discord_rich_presence::{new_client, DiscordIpc};

pub fn get_discord_client(app_id: u64) -> impl DiscordIpc {
    new_client(app_id.to_string().as_str()).expect(ERROR_MESSAGE)
}
