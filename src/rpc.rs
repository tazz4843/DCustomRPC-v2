use crate::{ERROR_MESSAGE, CONFIG};
use discord_rich_presence::{new_client, DiscordIpc};

pub fn get_discord_client() -> impl DiscordIpc {
    let app_id = CONFIG.get().expect(ERROR_MESSAGE).client_id;

    new_client(app_id.to_string().as_str()).expect(ERROR_MESSAGE)
}
