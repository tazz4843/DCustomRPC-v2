use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    client_id: u64,
    change_duration: u8,
    game_list: Vec<GameList>,
}

#[derive(Serialize, Deserialize)]
pub struct GameList {
    details: Option<String>,
    state: Option<String>,
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
    buttons: Vec<Button>,
}

#[derive(Serialize, Deserialize)]
pub struct Button {
    title: String,
    url: String,
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
            fs::write("config.json", serde_json::to_string_pretty(&cfg).expect("this shouldn't've happened: report it in the support server and include this backtrace"));

            panic!("new config written to disk, edit it and rerun this file")
        }
    };

    serde_json::from_slice(&cfg[..])
        .expect("failed to load config: make sure it's formatted properly")
}
