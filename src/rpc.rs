use rustcord::{EventHandlers, RichPresenceBuilder, Rustcord, User};
use std::io;
use crate::{ERROR_MESSAGE, CONFIG};
use crate::config::load_config;

pub struct RpcEventHandler;

impl EventHandlers for RpcEventHandler {
    fn ready(user: User) {
        println!(
            "logged in as {}#{} (ID {})",
            user.username, user.discriminator, user.user_id
        );
    }

    fn errored(code: i32, message: &str) {
        let msg = format!("error with RPC, code {}, message `{}`", code, message);

        if CONFIG.get().expect(ERROR_MESSAGE).exit_on_error {
            panic!("{}", msg)
        } else {
            eprintln!("{}", msg)
        }
    }

    fn disconnected(code: i32, message: &str) {
        let msg = format!(
            "disconnected from discord client with code {}, message `{}`",
            code, message
        );

        if CONFIG.get().expect(ERROR_MESSAGE).exit_on_disconnect {
            panic!("{}", msg)
        } else {
            println!("{}", msg)
        }
    }
}

pub fn get_discord_client<'a>(app_id: impl Into<&'a str>) -> Rustcord {
    Rustcord::init::<RpcEventHandler>(app_id.into(), false, None)
        .expect(ERROR_MESSAGE)
}
