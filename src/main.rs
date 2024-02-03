use crate::utils::get_week_matches;
use std::{collections::HashMap, env, sync::Arc};

use dotenvy::dotenv;
use miette::{Error, IntoDiagnostic, Result};
use models::Match;
use poise::{
    samples::register_globally,
    serenity_prelude::{self as serenity, ActivityData, GatewayIntents},
    Framework, FrameworkOptions, PrefixFrameworkOptions,
};
use reqwest::{header, Client};
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber;

mod commands;
mod models;
mod utils;

type Context<'a> = poise::Context<'a, Arc<Data>, Error>;

#[derive(Debug)]
struct Data {
    http_client: Client,
    // We only write new data occasionally,
    // but read is frequent so RwLock is used
    matches: RwLock<HashMap<String, Vec<Match>>>,
} // User data, which is stored and accessible in all command invocations

#[tokio::main]
async fn main() -> Result<()> {
    // Loading from .env file for tokens
    // If not, use env vars DISCORD_TOKEN and FOOTBALL_DATA_TOKEN
    if let Err(_) = dotenv() {
        info!(".env file not found. Using environment variables.");
    }
    // Default log verbosity is info
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info")
    }
    // Install global collector configured based on RUST_LOG env var.
    tracing_subscriber::fmt::init();

    info!("Initializing the bot...");

    let discord_token = env::var("DISCORD_TOKEN").expect("Missing Discord token!");
    let football_data_token =
        env::var("FOOTBALL_DATA_TOKEN").expect("Missing football-data.org token!");

    let mut headers = header::HeaderMap::new();
    headers.insert(
        "X-Auth-Token",
        football_data_token.parse().into_diagnostic()?,
    );
    let http_client = Client::builder()
        .default_headers(headers)
        .build()
        .into_diagnostic()?;
    let matches = RwLock::new(HashMap::<String, Vec<Match>>::new());
    let data = Arc::new(Data {
        http_client,
        matches,
    });

    // We spawn the poll task on another thread to not block
    tokio::spawn(get_week_matches(data.clone()));

    // We support both slash commands and prefix commands so MESSAGE_INTENT privilege is needed
    // Make sure you enable the Message Content Intent in your Bot settings
    // See https://discordpy.readthedocs.io/en/latest/intents.html#privileged-intents for more details
    let intents = serenity::GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let options = FrameworkOptions {
        commands: vec![
            commands::help(),
            commands::matches(),
            commands::leagues(),
            commands::ping(),
        ],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("=".into()),
            ..Default::default()
        },
        ..Default::default()
    };

    info!("Initialization complete! Starting the bot...");

    // Finally, we run the bot
    let framework = Framework::builder()
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                ctx.set_activity(Some(ActivityData::playing(
                    "football matches | /help | =help",
                )));
                register_globally(ctx, &framework.options().commands)
                    .await
                    .into_diagnostic()?;
                Ok(data)
            })
        })
        .options(options)
        .build();
    let mut client = serenity::Client::builder(discord_token, intents)
        .framework(framework)
        .await
        .into_diagnostic()?;

    client.start_autosharded().await.into_diagnostic()
}
