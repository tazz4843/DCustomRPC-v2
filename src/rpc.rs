use crate::ERROR_MESSAGE;
use discord_game_sdk::{CreateFlags, Discord, EventHandler};

pub struct RpcEventHandler;

impl EventHandler for RpcEventHandler {}

pub fn get_discord_client(app_id: u64) -> Discord<'static, RpcEventHandler> {
    Discord::with_create_flags(app_id as i64, CreateFlags::Default).expect(ERROR_MESSAGE)
}
