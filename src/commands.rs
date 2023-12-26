use crate::{
    utils::{FOOTBALL_DATA_ICON, VALID_LEAGUES},
    Context,
};
use chrono::FixedOffset;
use miette::{IntoDiagnostic, Result};
use poise::serenity_prelude::CreateEmbed;

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
pub async fn matches(
    ctx: Context<'_>,
    #[description = "League ID. For a list of valid leagues, use /leagues"] league: String,
) -> Result<()> {
    ctx.defer().await.into_diagnostic()?;
    let league = league.to_lowercase();
    match ctx.data().matches.read().await.get(&league) {
        Some(matches) => {
            let mut embed = CreateEmbed::default();
            embed.title(VALID_LEAGUES.get(&league).unwrap());

            for m in matches {
                embed.field(
                    "Match",
                    format!(
                        "{} - {} ({})",
                        m.home_team.short_name,
                        m.away_team.short_name,
                        m.utc_date
                            .with_timezone(&FixedOffset::east_opt(7 * 3600).unwrap())
                            .format("%d/%m/%Y %R %Z")
                    ),
                    false,
                );
            }
            embed.footer(|f| {
                f.icon_url(FOOTBALL_DATA_ICON);
                f.text("Data from https://www.football-data.org");
                f
            });
            ctx.send(|rep| {
                rep.embeds.push(embed);
                rep
            })
            .await
            .into_diagnostic()?
        }
        None => ctx.say("Invalid league ID.").await.into_diagnostic()?,
    };
    Ok(())
}
/// List all valid leagues
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn leagues(ctx: Context<'_>) -> Result<()> {
    ctx.send(|rep| {
        rep.embed(|em| {
            em.title("Leagues");
            for (id, name) in &VALID_LEAGUES {
                em.field(id, name, false);
            }
            em
        })
    })
    .await
    .into_diagnostic()?;

    Ok(())
}
