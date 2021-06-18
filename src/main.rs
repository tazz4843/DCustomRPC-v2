use discord_rpc_builder::config::load_config;

fn main() {
    println!("DCustomRPC-v2 by 0/0#0001");
    println!("For support join the Discord at https://discord.gg/5yXExTsRye");
    println!("Current version {}", env!("CARGO_PKG_VERSION"));

    let cfg = load_config();
}
