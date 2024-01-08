use crate::{Context, Data};
use std::{sync::Arc, time::Duration};

use chrono::{Datelike, NaiveDate, Utc};
use miette::{miette, IntoDiagnostic, Result};
use phf::{phf_map, Map};
use poise::AutocompleteChoice;
use tokio::time::interval;
use tracing::info;

use crate::models::Matches;

pub(crate) const FOOTBALL_DATA_URL: &'static str = "http://api.football-data.org/v4";
pub(crate) const FOOTBALL_DATA_ICON: &'static str =
    "https://www.football-data.org/assets/favicons/favicon-32x32.png";
pub(crate) static VALID_LEAGUES: Map<&str, &str> = phf_map! {
    "wc" => "FIFA World Cup",
    "cl" => "UEFA Champions League",
    "bl1" => "Bundesliga",
    "ded" => "Eredivisie",
    "bsa" => "Campeonato Brasileiro Séria A",
    "pd" => "Primera Division (La Liga)",
    "fl1" => "Ligue 1",
    "elc" => "Championship",
    "ppl" => "Primeira Liga",
    "ec" => "European Championship",
    "sa" => "Serie A",
    "pl" => "Premier League",
    "cli" => "Copa Libertadores",
};

pub(crate) async fn get_week_matches(data: Arc<Data>) -> Result<()> {
    // We poll the endpoint every hour for updated data
    let mut interval = interval(Duration::from_secs(3600));
    loop {
        interval.tick().await;
        let today = Utc::now().date_naive();
        // Get the match data from all supported leagues
        for league in VALID_LEAGUES.keys() {
            let data = data.clone();
            let league = league.to_uppercase();
            tokio::spawn(get_matches(data, today, league));
        }
    }
}

pub(crate) async fn get_matches(data: Arc<Data>, today: NaiveDate, league: String) -> Result<()> {
    // We count day of week number from 0 for easier math,
    // starting from Monday
    let dow_num = today.weekday().num_days_from_monday().into();
    let monday = today - chrono::Duration::days(dow_num);
    let sunday = today + (chrono::Duration::days(6 - dow_num));
    info!(
        "Getting this week's ({} - {}) matches for league {}",
        monday.format("%d/%m/%Y"),
        sunday.format("%d/%m/%Y"),
        league
    );
    match data
        .http_client
        .get(format!("{FOOTBALL_DATA_URL}/matches"))
        .query(&[
            ("filter", today.to_string()),
            ("competitions", league.to_string()),
            ("dateFrom", monday.to_string()),
            ("dateTo", sunday.to_string()),
        ])
        .send()
        .await
    {
        Ok(res) => match res.error_for_status() {
            Ok(body) => {
                let res = body.json::<Matches>().await.into_diagnostic()?;
                if res.matches.len() > 0 {
                    // We store the matches in a hashmap for caching
                    data.matches
                        .write()
                        .await
                        .insert(league.to_string().to_lowercase(), res.matches);
                } else {
                    data.matches
                        .write()
                        .await
                        .remove(&league.to_string().to_lowercase());
                }
                Ok(())
            }
            Err(e) => Err(miette!("Error from the server: {e}")),
        },
        Err(e) => Err(miette!("Error sending request to the server: {e}")),
    }
}

pub(crate) async fn autocomplete_league_name<'a>(
    _: Context<'_>,
    input: &'a str,
) -> impl Iterator<Item = AutocompleteChoice<String>> + 'a {
    VALID_LEAGUES
        .keys()
        .filter(move |l| l.contains(input))
        .map(|l| AutocompleteChoice {
            name: format!("{} - {}", l.to_string(), VALID_LEAGUES.get(l).unwrap()),
            value: l.to_string(),
        })
}
