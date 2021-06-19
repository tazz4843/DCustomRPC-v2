use crate::ERROR_MESSAGE;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub fn register_handler(shutdown: &Arc<AtomicBool>) {
    let shutdown_ref = Arc::clone(shutdown);
    ctrlc::set_handler(move || {
        println!("shutting down, don't press ctrl+c again!");
        eprintln!("you can safely ignore any status update failures after this");
        shutdown_ref.swap(true, Ordering::Relaxed);
    })
    .expect(ERROR_MESSAGE);
}
