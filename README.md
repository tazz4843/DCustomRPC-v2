# DCustomRPC: The Rewrite (the second part)!

DCustomRPC is a custom rich presence client that you (you right there, yes you) can customize. 
You may remember the two Python versions of this, but this is the rewite (of the rewrite) in Rust!
Now you don't have to worry about installing dependencies ~~(rust > python)~~!

## Downloads

[Linux](
https://github.com/tazz4843/DCustomRPC-v2/releases/download/v1.0.0/DCustomRPC-v2-x86_64-linux)

[Windows](
https://github.com/tazz4843/DCustomRPC-v2/releases/download/v1.0.0/DCustomRPC-v2-x86_64-windows.exe)

No MacOS downloads. You must build it manually.

## Setup the Discord Application
1. Run the executable once. This will create a new file named `config.json`.
2. Next go to Discord Developers (https://discord.com/developers/applications/) and sign in.
3. From here, click the "New App" button and enter a "App Name". This will show as what you are 
   playing. "App Description" and "App Icon" do not matter for rich presence.
4. After you have done this, you can copy the "Client ID" (under "App Details") and replace the 
   client_id already in the config file.
5. You can set up the config as you desire here.

## Discord Prerequisites

Please make sure that game statuses are turned on:

![Game Toggle](https://i.imgur.com/V4FWevH.png)

## Config Details

`client_id`: Your Discord application ID.

`change_duration`: How often (in seconds) your game should switch. Must be >0 and <255.

`game_list`: A list of games to rotate through.
### `game_list` keys
`details`: Line 1 of your status.

`state`: Line 2 of your status.

`large_image`: The large image to display. Must correspond with one of the images uploaded to 
the Discord Developer Portal.

`large_text`: The tooltip to display when hovering over the large image.

`small_image`: The small image to display. Must correspond with one of the images uploaded to
the Discord Developer Portal.

`small_text`: The tooltip to display when hovering over the small image.

`buttons`: List of buttons to display. Max of 2.

#### `buttons` keys
`title`: The text to show on the button.

`url`: The URL to send users to when they click on the button.


## Building from Source

### Dependencies
`cargo` must be installed on your system. To install it, head to https://rustup.rs and follow the steps.
You can use the default settings for everything, as DCustomRPC does not use nightly features.

### Building

```shell
git clone https://github.com/tazz4843/DCustomRPC-v2
cargo build --release
```
The executable is located in `target/release/`.

## Useful Links

[Rust](https://rust-lang.org)

[Repo Download: Stable](https://github.com/tazz4843/DCustomRPC-v2/archive/refs/tags/v1.0.0.zip)

[Repo Download: Beta](https://github.com/tazz4843/DCustomRPC-v2/archive/refs/heads/main.zip) 

[Support Server](https://discord.gg/5yXExTsRye)
