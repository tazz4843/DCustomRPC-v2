use crate::activity::{Activity, Assets, Party, Timestamps};
use crate::rpc::get_discord_client;
use crate::signal_handler::register_handler;
use crate::{CONFIG, ERROR_MESSAGE};
use discord_rich_presence::DiscordIpc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

pub fn entrypoint() {
    let cfg = CONFIG.get().expect(ERROR_MESSAGE);

    println!("initializing RPC...");
    let mut client = get_discord_client(cfg.client_id);
    client.connect().expect("failed to connect to discord");

    // flag to tell the mainloop to shut down
    let shutdown = Arc::new(AtomicBool::new(false));

    // sigint/ctrl+c handler
    register_handler(&shutdown);

    println!("initialized RPC!");

    let party_max = cfg.game_list.len() as u32;
    let time_per_loop = Duration::from_secs(cfg.change_duration as u64);

    'outer: loop {
        for (i, item) in cfg.game_list.iter().enumerate() {
            let start_time = SystemTime::now();
            let end_time = start_time + time_per_loop;

            #[cfg(debug_assertions)]
            let st = SystemTime::now();
            println!("updating activity...");

            let activity = Activity {
                state: item.state.clone(),
                details: item.details.clone(),
                timestamps: Some(Timestamps {
                    start: None,
                    end: Some(end_time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("check your system clock, it's set to before 1 Jan 1970 00:00:00+00:00")
                    .as_secs() as i32)
                }),
                assets: Some(Assets {
                    large_image: item.large_image.clone(),
                    large_text: item.large_text.clone(),
                    small_image: item.small_image.clone(),
                    small_text: item.small_text.clone()
                }),
                party: Some(Party {
                    size: Some([(i + 1) as u32, party_max])
                })
            };

            client
                .set_activity(activity)
                .expect("failed to update activity");

            #[cfg(debug_assertions)]
            {
                let tt = st
                    .elapsed()
                    .expect("your system clock rolled back")
                    .as_nanos();
                println!("updated status in {}ns", tt);
            }

            println!("updated status");

            thread::sleep(time_per_loop);

            if shutdown.load(Ordering::Relaxed) {
                break 'outer;
            }
        }
    }
    client
        .close()
        .expect("failed to shut down client: restart discord to fix BrokenPipe errors with RPC");
}
