use std::env;

use dotenvy::dotenv;
use miette::{Error, IntoDiagnostic, Result};
use poise::{
    samples::register_globally,
    serenity_prelude::{self as serenity, GatewayIntents},
    Framework, FrameworkOptions, PrefixFrameworkOptions,
};
use reqwest::{header, Client};
use tracing::info;
use tracing_subscriber;

mod commands;
mod models;

type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {
    http_client: Client,
} // User data, which is stored and accessible in all command invocations

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file not found!");

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

    let intents = serenity::GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let options = FrameworkOptions {
        commands: vec![commands::help(), commands::matches()],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some("=".into()),
            ..Default::default()
        },
        ..Default::default()
    };

    Framework::builder()
        .token(discord_token)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                register_globally(ctx, &framework.options().commands)
                    .await
                    .into_diagnostic()?;
                Ok(Data { http_client })
            })
        })
        .options(options)
        .intents(intents)
        .run()
        .await
        .into_diagnostic()
}
