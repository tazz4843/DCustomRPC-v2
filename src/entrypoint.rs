use crate::rpc::get_discord_client;
use crate::signal_handler::register_handler;
use crate::{CONFIG, ERROR_MESSAGE};
use discord_rich_presence::{DiscordIpc, activity};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

pub fn entrypoint() {
    let cfg = CONFIG.get().expect(ERROR_MESSAGE);

    println!("initializing RPC...");
    let mut client = get_discord_client();
    client.connect().expect("failed to connect to discord");

    // flag to tell the mainloop to shut down
    let shutdown = Arc::new(AtomicBool::new(false));

    // sigint/ctrl+c handler
    register_handler(&shutdown);

    println!("initialized RPC!");

    let party_max = cfg.game_list.len() as i32;
    let time_per_loop = Duration::from_secs(cfg.change_duration as u64);

    'outer: loop {
        for (i, item) in cfg.game_list.iter().enumerate() {
            if shutdown.load(Ordering::Relaxed) {
                break 'outer;
            }

            let start_time = SystemTime::now();
            let end_time = start_time + time_per_loop;

            #[cfg(debug_assertions)]
            let st = SystemTime::now();
            if cfg.verbose {
                println!("updating activity...");
            }

            let mut activity = activity::Activity::new();
            activity = if let Some(ref state) = item.state {
                activity.state(state.as_str())
            } else {
                activity
            };

            activity = if let Some(ref details) = item.details {
                activity.details(details.as_str())
            } else {
                activity
            };

            activity = activity.timestamps(activity::Timestamps::new().end(end_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("check your system clock, it's set to before 1 Jan 1970 00:00:00+00:00")
                .as_secs() as i32));

            activity = activity.assets({
                let mut assets = activity::Assets::new();

                assets = if let Some(ref large_image) = item.large_image {
                    if let Some(ref large_text) = item.large_text {
                        assets.large_image(large_image.as_str()).large_text(large_text.as_str())
                    } else {
                        assets.large_image(large_image.as_str())
                    }
                } else {
                    assets
                };

                assets = if let Some(ref small_image) = item.small_image {
                    if let Some(ref small_text) = item.small_text {
                        assets.small_image(small_image.as_str()).small_text(small_text.as_str())
                    } else {
                        assets.small_image(small_image.as_str())
                    }
                } else {
                    assets
                };

                assets
            });

            activity = activity.party(activity::Party::new().size([(i + 1) as i32, party_max]));

            let mut buttons = vec![];
            for button in item.buttons.iter() {
                buttons.push(activity::Button::new(button.title.as_str(), button.url.as_str()))
            }
            activity = activity.buttons(buttons);

            client
                .set_activity(activity)
                .unwrap_or_else(|e| {
                    if CONFIG.get().expect(ERROR_MESSAGE).exit_on_error {
                        panic!("error while updating status: {}", e)
                    } else {
                        eprintln!("error while updating status: {}", e)
                    }
                });

            #[cfg(debug_assertions)]
            {
                let tt = st
                    .elapsed()
                    .expect("your system clock rolled back")
                    .as_nanos();
                println!("updated status in {}ns", tt);
            }

            if cfg.verbose {
                println!("updated status");
            }

            thread::sleep(time_per_loop);
        }
    }
    client
        .close()
        .expect("failed to shut down client: restart discord to fix BrokenPipe errors with RPC");
}
