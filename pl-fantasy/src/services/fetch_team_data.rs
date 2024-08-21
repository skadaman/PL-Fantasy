use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TeamSelection {
    pub entry_history: EntryHistory,
    pub picks: Vec<Pick>,
    pub subs: Vec<Substitution>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryHistory {
    // This is left empty as the JSON shows an empty object
    // We can add fields here if needed in the future
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pick {
    pub element: u32,
    pub is_captain: bool,
    pub is_vice_captain: bool,
    pub multiplier: u8,
    pub position: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Substitution {
    pub element_in: u32,
    pub element_out: u32,
    pub event: u8,
}
pub async fn fetch_team_data(team_id: u32) -> Result<TeamSelection, Box<dyn std::error::Error>> {
    let url = format!("https://draft.premierleague.com/api/entry/{}/event/1", team_id);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let team_selection: TeamData = response.json().await?;
        Ok(team_selection)
    } else {
        Err(format!("Failed to fetch data: HTTP {}", response.status()).into())
    }
}