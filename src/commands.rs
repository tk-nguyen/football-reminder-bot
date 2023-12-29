use crate::{
    utils::{FOOTBALL_DATA_ICON, VALID_LEAGUES},
    Context,
};
use chrono::FixedOffset;
use miette::{IntoDiagnostic, Result};
use poise::serenity_prelude::{Colour, CreateEmbed};

/// Show help about the bot
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<()> {
    poise::builtins::help(
        ctx,
        None,
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "This is a bot to post today's football matches. Data is from https://www.football-data.org",
            ephemeral: true,
            ..Default::default()
        },
    )
    .await.into_diagnostic()
}

/// Find today's matches
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn matches(ctx: Context<'_>, #[description = "League ID"] league: String) -> Result<()> {
    // By default, Discord timeout for slash command response is 3s
    // This makes the timeout 15 minutes
    ctx.defer().await.into_diagnostic()?;

    let league = league.to_lowercase();
    if VALID_LEAGUES.contains_key(&league) {
        match ctx.data().matches.read().await.get(&league) {
            Some(matches) => {
                let mut embed = CreateEmbed::default();
                // There are definitely matches and valid leagues, so it's OK to unwrap here.
                embed.title(format!("**{}**", VALID_LEAGUES.get(&league).unwrap()));
                if let Some(url) = &matches.get(0).unwrap().competition.emblem {
                    embed.thumbnail(url);
                }
                for (idx, m) in matches.iter().enumerate() {
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
                    embed.field(
                        format!("Match {}", idx + 1),
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
                    );
                }
                embed.footer(|f| {
                    f.icon_url(FOOTBALL_DATA_ICON);
                    f.text("Data from https://www.football-data.org");
                    f
                });
                embed.colour(Colour::BLURPLE);
                ctx.send(|rep| {
                    rep.embeds.push(embed);
                    rep
                })
                .await
                .into_diagnostic()?
            }
            None => ctx
                .send(|rep| {
                    rep.embed(|em| {
                        em.title(format!("**{}**", VALID_LEAGUES.get(&league).unwrap()));
                        em.field("", "No matches are scheduled for today", false);
                        em.colour(Colour::RED)
                    })
                })
                .await
                .into_diagnostic()?,
        };
    } else {
        ctx.send(|rep| rep.content("Invalid league ID. For a list of valid leagues, use /leagues"))
            .await
            .into_diagnostic()?;
    }
    Ok(())
}
/// List all valid leagues
#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn leagues(ctx: Context<'_>) -> Result<()> {
    ctx.send(|rep| {
        rep.embed(|em| {
            em.title("**Leagues**");
            for (id, name) in &VALID_LEAGUES {
                em.field(id, name, true);
            }
            em.colour(Colour::BLURPLE)
        })
    })
    .await
    .into_diagnostic()?;

    Ok(())
}
