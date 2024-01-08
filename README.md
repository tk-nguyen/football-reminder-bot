# Football Reminder Bot

A simple Discord bot, written in Rust to post current week's football games

## Build

- Download the Rust toolchain from https://rustup.rs/
- Clone the repo: `git clone https://github.com/tk-nguyen/football-reminder-bot/`
- Change directory and build: `cd football-reminder-bot && cargo build`. `cargo build --release` to build in release mode.

## Run

- Get a token from https://www.football-data.org/client/register
- Create an `.env` file with the following keys:
  - `DISCORD_TOKEN`: Token of your Discord bot. See https://docs.discordbotstudio.org/setting-up-dbs/finding-your-bot-token for how to get it.
  - `FOOTBALL_DATA_TOKEN`: The token you get from above
- Finally, run `cargo run` to run the bot. Use `cargo run --release` to run in release mode.


## Usage

```
Commands:
  /help        Show help about the bot
  /matches     Find current week's matches
  /leagues     List all valid leagues
  /ping        Return latency from the bot to Discord

This is a bot to post current week's football matches. Data is from https://www.football-data.org
```
