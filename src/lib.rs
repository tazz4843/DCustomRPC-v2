use once_cell::sync::OnceCell;

pub mod activity;
pub mod config;
pub mod entrypoint;
pub mod rpc;
pub mod signal_handler;

pub const ERROR_MESSAGE: &str =
    "this shouldn't've happened: report it in the support server and include this backtrace";

pub static CONFIG: OnceCell<config::Config> = OnceCell::new();
