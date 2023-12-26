use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Matches {
    pub matches: Vec<Match>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub id: usize,
    pub utc_date: DateTime<Utc>,
    pub status: String,
    pub matchday: usize,
    pub stage: String,
    pub group: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub home_team: Team,
    pub away_team: Team,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub id: usize,
    pub name: String,
    pub short_name: String,
    pub tla: String,
    pub crest: Url,
}
