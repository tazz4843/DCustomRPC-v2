use crate::rpc::get_discord_client;
use crate::{CONFIG, ERROR_MESSAGE};
use discord_game_sdk::Activity;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::{Duration, SystemTime};

pub fn entrypoint() {
    let cfg = CONFIG.get().expect(ERROR_MESSAGE);

    println!("initializing RPC...");
    let mut client = get_discord_client(cfg.client_id);

    // flag to tell the mainloop to shut down
    let shutdown = Arc::new(AtomicBool::new(false));

    // sigint/ctrl+c handler
    let shutdown_ref = Arc::clone(&shutdown);
    ctrlc::set_handler(move || {
        println!("shutting down, don't press ctrl+c again!");
        eprintln!("you can safely ignore any status update failures after this");
        shutdown_ref.swap(true, Ordering::Relaxed);
    })
    .expect(ERROR_MESSAGE);

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

            let mut rpc = Activity::empty();
            if let Some(ref details) = item.details {
                rpc.with_details(details.as_str());
            }
            rpc.with_end_time(
                (end_time
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .expect("check your system clock, it's set to before 1 Jan 1970 00:00:00+00:00")
                    .as_secs()) as i64,
            );
            if let Some(ref large_image) = item.large_image {
                rpc.with_large_image_key(large_image.as_str());
            }
            if let Some(ref large_text) = item.large_text {
                rpc.with_large_image_tooltip(large_text.as_str());
            }

            rpc.with_party_amount(party_max);
            rpc.with_party_capacity((i + 1) as u32);

            if let Some(ref small_image) = item.small_image {
                rpc.with_small_image_key(small_image.as_str());
            }
            if let Some(ref small_text) = item.small_text {
                rpc.with_small_image_tooltip(small_text.as_str());
            }
            if let Some(ref state) = item.state {
                rpc.with_state(state.as_str());
            }

            client.update_activity(&rpc, |_, r| {
                if let Err(e) = r {
                    eprintln!("failed to update activity: {}", e);
                }
            });
            client.run_callbacks().expect(ERROR_MESSAGE);

            #[cfg(debug_assertions)]
            let tt = st.elapsed().expect("your system clock rolled back").as_nanos();
            println!("updated status in {}ns", tt);

            println!("updated status");

            thread::sleep(time_per_loop);

            if shutdown.load(Ordering::Relaxed) {
                break 'outer;
            }
        }
    }
}
