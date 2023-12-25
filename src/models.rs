use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Matches {
    pub matches: Vec<Match>,
}

#[derive(Debug, Deserialize)]
pub struct Match {
    pub id: usize,
    #[serde(rename = "utcDate")]
    pub utc_date: DateTime<Utc>,
    pub status: String,
    pub matchday: usize,
    pub stage: String,
    pub group: Option<String>,
    #[serde(rename = "lastUpdated")]
    pub last_updated: DateTime<Utc>,
    #[serde(rename = "homeTeam")]
    pub home_team: Team,
    #[serde(rename = "awayTeam")]
    pub away_team: Team,
}

#[derive(Debug, Deserialize)]
pub struct Team {
    pub id: usize,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub tla: String,
    pub crest: Url,
}
