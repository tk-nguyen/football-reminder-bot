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
    pub competition: Competition,
    pub id: usize,
    pub utc_date: DateTime<Utc>,
    pub status: String,
    pub matchday: Option<usize>,
    pub stage: String,
    pub group: Option<String>,
    pub last_updated: DateTime<Utc>,
    pub home_team: Team,
    pub away_team: Team,
    pub score: Score,
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

#[derive(Debug, Deserialize)]
pub struct Competition {
    pub id: usize,
    pub name: String,
    pub code: String,
    #[serde(rename = "type")]
    pub competition_type: String,
    pub emblem: Option<Url>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub winner: Option<String>,
    pub duration: String,
    pub half_time: DetailedScore,
    pub full_time: DetailedScore,
}

#[derive(Debug, Deserialize)]
pub struct DetailedScore {
    pub home: Option<usize>,
    pub away: Option<usize>,
}
