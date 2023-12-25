use chrono::Utc;
use miette::{IntoDiagnostic, Result};
use tracing::info;

use crate::{models::Matches, Context};

const FOOTBALL_DATA_URL: &'static str = "http://api.football-data.org/v4";

/// Show help about the bot
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<()> {
    poise::builtins::help(
        ctx,
        None,
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is a bot to post daily football matches. Data is from https://www.football-data.org",
            ..Default::default()
        },
    )
    .await.into_diagnostic()
}

/// Find today's matches
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn matches(ctx: Context<'_>) -> Result<()> {
    ctx.defer().await.into_diagnostic()?;
    info!("Got request for matches");
    let today = Utc::now().date_naive();
    let res = ctx
        .data()
        .http_client
        .get(format!("{FOOTBALL_DATA_URL}/matches"))
        .query(&[("filter", today)])
        .send()
        .await
        .into_diagnostic()?
        .json::<Matches>()
        .await
        .into_diagnostic()?;
    ctx.send(|r| r.content(format!("{:?}", res)))
        .await
        .into_diagnostic()?;
    Ok(())
}
