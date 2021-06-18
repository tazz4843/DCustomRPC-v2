use crate::config::Config;
use crate::{CONFIG, ERROR_MESSAGE};
use crate::rpc::get_discord_client;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use rustcord::RichPresence;
use std::ops::Add;

pub fn entrypoint() {
    let cfg = CONFIG.get().expect(ERROR_MESSAGE);

    println!("initializing RPC...");
    let client = get_discord_client(cfg.client_id);
    // the initialized message is handled in the handlers so we don't need one here

    // now we need to spawn a background thread to do callbacks and stuff
    let shutdown = Arc::new(AtomicBool::new(false));
    let client_ref = &client;
    let shutdown_ref = Arc::clone(&shutdown);
    let callback_thread = thread::spawn(move || {
        loop {
            client_ref.run_callbacks();
            if shutdown_ref.load(Ordering::Relaxed) {
                break;
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    // also a sigint/ctrl+c handler
    let shutdown_ref = Arc::clone(&shutdown);
    ctrlc::set_handler(move || {
        println!("shutting down, don't press ctrl+c again!");
        shutdown_ref.swap(true, Ordering::Relaxed);
    })


    let party_max = cfg.game_list.len();
    let time_per_loop = Duration::from_secs(cfg.change_duration as u64);

    'outer: loop {
        for (i, item) in cfg.game_list.iter().enumerate() {
            let start_time = SystemTime::now();
            let end_time = start_time + time_per_loop;

            let mut rpc = RichPresence::default();
            rpc.details = item.details.clone();
            rpc.end_time = Some(end_time);
            rpc.large_image_key = item.large_image.clone();
            rpc.large_image_text = item.large_image.clone();
            rpc.party_max = Some(party_max.into());
            rpc.party_size = (i + 1).into();
            rpc.small_image_key = item.small_image.clone();
            rpc.small_image_text = item.small_text.clone();
            rpc.start_time = None;
            rpc.state = item.state.clone();

            client_ref.update_presence(rpc).expect(ERROR_MESSAGE);

            thread::sleep(time_per_loop);

            if shutdown_ref.load(Ordering::Relaxed) {
                break 'outer;
            }
        }
    }

    // clean up the background thread
    callback_thread.join().expect(ERROR_MESSAGE);
}
