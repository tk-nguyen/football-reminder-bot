use crate::Data;
use std::time::Duration;

use chrono::Utc;
use miette::{IntoDiagnostic, Result};
use phf::{phf_map, Map};
use tokio::time::sleep;
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
    "pd" => "Primera Division",
    "fl1" => "Ligue 1",
    "elc" => "Championship",
    "ppl" => "Primeira Liga",
    "ec" => "European Championship",
    "sa" => "Serie A",
    "pl" => "Premier League",
    "cli" => "Copa Libertadores",
};

pub(crate) async fn get_today_matches(data: Data) -> Result<()> {
    let today = Utc::now().date_naive();
    info!("Getting today's ({}) matches", today.format("%d/%m/%Y"));
    for league in VALID_LEAGUES.keys() {
        let data = data.clone();
        let league = league.to_uppercase();
        tokio::spawn(async move {
            let res = data
                .http_client
                .get(format!("{FOOTBALL_DATA_URL}/matches"))
                .query(&[
                    ("filter", today.to_string()),
                    ("competitions", league.to_string()),
                ])
                .send()
                .await
                .into_diagnostic()
                .unwrap()
                .json::<Matches>()
                .await
                .into_diagnostic()
                .unwrap();
            if res.matches.len() > 0 {
                data.matches
                    .write()
                    .await
                    .insert(league.to_string().to_lowercase(), res.matches);
            }
        });
    }
    sleep(Duration::from_secs(86400)).await;
    Ok(())
}
