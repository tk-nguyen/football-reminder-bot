use models::Match;
use std::{collections::HashMap, env, sync::Arc};

use dotenvy::dotenv;
use miette::{Error, IntoDiagnostic, Result};
use poise::{
    samples::register_globally,
    serenity_prelude::{self as serenity, GatewayIntents},
    Framework, FrameworkOptions, PrefixFrameworkOptions,
};
use reqwest::{header, Client};
use tokio::sync::RwLock;
use tracing::info;
use tracing_subscriber;

mod commands;
mod models;
mod utils;

type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Clone)]
struct Data {
    http_client: Client,
    matches: Arc<RwLock<HashMap<String, Vec<Match>>>>,
} // User data, which is stored and accessible in all command invocations

#[tokio::main]
async fn main() -> Result<()> {
    if let Err(_) = dotenv() {
        info!(".env file not found. Using environment variables.");
    }

    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "info")
    }
    // install global collector configured based on RUST_LOG env var.
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
    let matches = Arc::new(RwLock::new(HashMap::<String, Vec<Match>>::new()));
    let data = Data {
        http_client,
        matches,
    };

    let _ = tokio::spawn(utils::get_today_matches(data.clone()));

    let intents = serenity::GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let options = FrameworkOptions {
        commands: vec![commands::help(), commands::matches(), commands::leagues()],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("=".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    info!("Initialization complete! Starting the bot...");
    Framework::builder()
        .token(discord_token)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands)
                    .await
                    .into_diagnostic()?;
                Ok(data)
            })
        })
        .options(options)
        .intents(intents)
        .run()
        .await
        .into_diagnostic()
}
