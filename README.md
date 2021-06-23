# DCustomRPC: The Rewrite (the second part)!

DCustomRPC is a custom rich presence client that you (you right there, yes you) can customize. 
You may remember the two Python versions of this, but this is the rewite (of the rewrite) in Rust!
Now you don't have to worry about installing dependencies ~~(rust > python)~~!

# Downloads

[Linux](
https://github.com/tazz4843/DCustomRPC-v2/releases/download/v1.1.0/DCustomRPC-v2-x86_64-linux)

[Windows](
https://github.com/tazz4843/DCustomRPC-v2/releases/download/v1.1.0/DCustomRPC-v2-x86_64-windows.exe)

No MacOS downloads. You must build it manually.

# Setup
1. Run the executable once. This will create a new file named `config.json`.
2. Next go to Discord Developers (https://discord.com/developers/applications/) and sign in.
3. From here, click the "New App" button and enter a "App Name". This will show as what you are 
   playing. "App Description" and "App Icon" do not matter for rich presence.
4. After you have done this, you can copy the "Client ID" (under "App Details") and replace the 
   client_id already in the config file.
5. You can set up the config as you desire here.

# Discord Prerequisites

Please make sure that game statuses are turned on:

![Game Toggle](https://i.imgur.com/V4FWevH.png)

# Config Details

## VERY IMPORTANT
* Make sure any values are wrapped in quotes (like `"key": "value"`).
* If you're not using a value, just get rid of it, don't set it to a empty string.
* `large_image` must be set for `small_image` to show up.
* You can only have two buttons per game.
* You can have unlimited games, but the more games you add, the longer it'll take to cycle
through them all.

## General Config

`client_id`: Your Discord application ID.

`change_duration`: How often (in seconds) your game should switch. Must be >0 and <255.
Strongly recommended to be >5 to prevent Discord rate limits.

`game_list`: A list of games to rotate through.
## `game_list` keys
`details`: Line 1 of your status.

`state`: Line 2 of your status.

`large_image`: The large image to display. Must correspond with one of the images uploaded to 
the Discord Developer Portal.

`large_text`: The tooltip to display when hovering over the large image.

`small_image`: The small image to display. Must correspond with one of the images uploaded to
the Discord Developer Portal.

`small_text`: The tooltip to display when hovering over the small image.

`buttons`: List of buttons to display. Max of 2.

### `buttons` keys
`title`: The text to show on the button.

`url`: The URL to send users to when they click on the button.

# FAQ

* **A window pops up then closes instantly!**
  * This probably means the program panicked.
    * Windows: run the program in Command Prompt. 
      https://www.wikihow.com/Run-an-EXE-File-From-Command-Prompt
      When you run the file, make sure not to add the `start` to the start of the command.
    * MacOS: run the program in Terminal.
    * Linux: run the program in Terminal.
  * No matter what OS you're on, you should have gotten output that looks something like
   `thread 'main' panicked at 'something something', src/main.rs:0:0`
    * `thread 'main' panicked at 'failed to load config: make sure it's formatted properly: 
      Error("something", line: 0, column: 0), src/config.rs:68:10'`
      * This means you messed up your config. Make sure you've done it properly 
        (that means you've wrapped all strings in quotes) and that it's actually valid. 
        If you're sure it's valid, run it through a JSON checker. 
        If the JSON checker shows it as valid, then there's a bug somewhere, report it to me.
    * `thread 'main' panicked at 'this shouldn't've happened: report it in the support server 
      and include this backtrace'`
      * There's a bug somewhere in the code. Rerun the command, but put `RUST_BACKTRACE=full`
        before the command, and if you still get that error, copy and paste all of it and
        report it to me in the support server.
    * Some other error
      * We haven't documented it yet, so report it in the support server.
* **I got some sort of error about executable format error?**
  * You're probably on a 32-bit OS, which isn't supported. Switch to a 64-bit OS.
    If you *are* on a 64-bit OS, your CPU probably doesn't support the required instructions.
    Get a new computer. Any CPU that doesn't support the required instructions is really old.

# Building from Source

## Dependencies
`cargo` must be installed on your system. To install it, head to https://rustup.rs and follow the steps.
You can use the default settings for everything, as DCustomRPC does not use nightly features.

## Building

```shell
git clone https://github.com/tazz4843/DCustomRPC-v2
cargo build --release
```
The executable is located in `target/release/`.

# Useful Links

[Rust](https://rust-lang.org)

[Repo Download: Stable](https://github.com/tazz4843/DCustomRPC-v2/archive/refs/tags/v1.1.0.zip)

[Repo Download: Beta](https://github.com/tazz4843/DCustomRPC-v2/archive/refs/heads/main.zip) 

[Support Server](https://discord.gg/5yXExTsRye)
