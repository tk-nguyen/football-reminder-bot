use crate::{
    utils::{autocomplete_league_name, FOOTBALL_DATA_ICON, VALID_LEAGUES},
    Context,
};
use chrono::FixedOffset;
use miette::{IntoDiagnostic, Result};
use poise::{
    serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter},
    CreateReply,
};

/// Show help about the bot
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<()> {
    poise::builtins::help(
        ctx,
        None,
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is a bot to post current week's football matches. Data is from https://www.football-data.org",
            ephemeral: true,
            ..Default::default()
        },
    )
    .await.into_diagnostic()
}

/// Find current week's matches
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn matches(
    ctx: Context<'_>,
    #[description = "League ID"]
    #[autocomplete = "autocomplete_league_name"]
    league: String,
) -> Result<()> {
    // By default, Discord timeout for slash command response is 3s
    // This makes the timeout 15 minutes
    ctx.defer().await.into_diagnostic()?;

    let league = league.to_lowercase();
    if VALID_LEAGUES.contains_key(&league) {
        match ctx.data().matches.read().await.get(&league) {
            Some(matches) => {
                let embed = CreateEmbed::default()
                    .footer(
                        CreateEmbedFooter::new("Data from https://www.football-data.org")
                            .icon_url(FOOTBALL_DATA_ICON),
                    )
                    .colour(Colour::BLURPLE)
                    .title(format!("**{}**", VALID_LEAGUES.get(&league).unwrap()))
                    .fields(matches.iter().enumerate().map(|(idx, m)| {
                        let (home, away) = match (m.score.half_time.home, m.score.full_time.home) {
                            (Some(_), None) => (
                                m.score.half_time.home.unwrap(),
                                m.score.half_time.away.unwrap(),
                            ),
                            (None, None) => (0, 0),
                            (_, Some(_)) => (
                                m.score.full_time.home.unwrap(),
                                m.score.full_time.away.unwrap(),
                            ),
                        };
                        (
                            match m.matchday {
                                Some(round) => format!("Match {} - Round {}", idx + 1, round),
                                None => format!("Match {}", idx + 1),
                            },
                            format!(
                                "**{} {} - {} {}** ({})",
                                m.home_team.short_name,
                                home,
                                away,
                                m.away_team.short_name,
                                m.utc_date
                                    .with_timezone(&FixedOffset::east_opt(7 * 3600).unwrap())
                                    .format("%d/%m/%Y %R %Z")
                            ),
                            false,
                        )
                    }))
                    .thumbnail(match &matches.get(0).unwrap().competition.emblem {
                        Some(url) => url.to_string(),
                        None => "".to_string(),
                    });
                ctx.send(CreateReply::default().embed(embed))
                    .await
                    .into_diagnostic()?
            }
            None => ctx
                .send(
                    CreateReply::default().embed(
                        CreateEmbed::default()
                            .title(format!("**{}**", VALID_LEAGUES.get(&league).unwrap()))
                            .field("", "No matches are scheduled for this week", false)
                            .colour(Colour::RED),
                    ),
                )
                .await
                .into_diagnostic()?,
        };
    } else {
        ctx.send(
            CreateReply::default()
                .content("Invalid league ID. For a list of valid leagues, use /leagues"),
        )
        .await
        .into_diagnostic()?;
    }
    Ok(())
}
/// List all valid leagues
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn leagues(ctx: Context<'_>) -> Result<()> {
    ctx.send(CreateReply::default().embed(CreateEmbed::default()
            .title("**Leagues**")
            .colour(Colour::BLURPLE)
            .description("Use the short league name to query matches, for example: `/matches pl` for Premier League, `/matches sa` for Serie A, ...")
            .fields(VALID_LEAGUES.into_iter().map(|(&id, &name)| (id, name, false)) )
        ))
    .await
    .into_diagnostic()?;

    Ok(())
}

/// Return latency from the bot to Discord
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    let ping = ctx.ping().await.as_millis();
    ctx.say(format!("**Pong! {} ms**", ping))
        .await
        .into_diagnostic()?;
    Ok(())
}
