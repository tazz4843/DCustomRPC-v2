use crate::{CONFIG, ERROR_MESSAGE};
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub client_id: u64,
    pub change_duration: u8,
    pub game_list: Vec<GameList>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct GameList {
    pub details: Option<String>,
    pub state: Option<String>,
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
    pub buttons: Vec<Button>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Button {
    pub title: String,
    pub url: String,
}

pub fn load_config() -> Config {
    let cfg = match fs::read("config.json") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("error while loading config: {}", e);
            println!("writing new config to disk...");

            let cfg = Config {
                client_id: 814550660544200725,
                change_duration: 10,
                game_list: vec![GameList {
                    details: Some("this is the first line of your RPC".to_string()),
                    state: Some("this is the second line of your RPC".to_string()),
                    large_image: None,
                    large_text: None,
                    small_image: Some("small img name".to_string()),
                    small_text: Some("small img text".to_string()),
                    buttons: vec![Button {
                        title: "this is a button".to_string(),
                        url: "https://imaskeleton.me".to_string(),
                    }],
                }],
            };
            fs::write(
                "config.json",
                serde_json::to_string_pretty(&cfg)
                    .expect(ERROR_MESSAGE)
            )
                .expect("couldn't write config to disk, do you have perms to write to the current directory?");

            panic!("new config written to disk, edit it and rerun this file")
        }
    };

    let cfg: Config = serde_json::from_slice(&cfg[..])
        .expect("failed to load config: make sure it's formatted properly");

    for i in cfg.game_list.iter() {
        if i.buttons.len() > 2 {
            panic!("you can't have more than two buttons in a page!")
        }
    }

    CONFIG
        .set(cfg.clone())
        .unwrap_or_else(|_| panic!("{}", ERROR_MESSAGE));

    cfg
}
