use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueData {
    pub league: League,
    pub league_entries: Vec<LeagueEntry>,
    pub standings: Vec<Standing>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct League {
    pub admin_entry: u32,
    pub closed: bool,
    pub draft_dt: String,
    pub draft_pick_time_limit: u32,
    pub draft_status: String,
    pub draft_tz_show: String,
    pub id: u32,
    pub ko_rounds: u32,
    pub make_code_public: bool,
    pub max_entries: u32,
    pub min_entries: u32,
    pub name: String,
    pub scoring: String,
    pub start_event: u32,
    pub stop_event: u32,
    pub trades: String,
    pub transaction_mode: String,
    pub variety: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LeagueEntry {
    pub entry_id: u32,
    pub entry_name: String,
    pub id: u32,
    pub joined_time: String,
    pub player_first_name: String,
    pub player_last_name: String,
    pub short_name: String,
    pub waiver_pick: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Standing {
    pub event_total: u32,
    pub last_rank: Option<u32>,
    pub league_entry: u32,
    pub rank: u32,
    pub rank_sort: u32,
    pub total: u32,
}

pub async fn fetch_league_data(league_id: u32) -> Result<LeagueData, Box<dyn std::error::Error>> {
    let url = format!("https://draft.premierleague.com/api/league/{}/details", league_id);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let league_data: LeagueData = response.json().await?;
        Ok(league_data)
    } else {
        Err(format!("Failed to fetch data: HTTP {}", response.status()).into())
    }
}