use reqwest;
use serde_json::Value;

// TODO: Adds structs for all player data. 

pub async fn fetch_team_data(team_id: u32) -> Result<PlayerMetaData, Box<dyn std::error::Error>> {
    let url = format!("https://draft.premierleague.com/api/bootstrap-static");

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let all_player_data: TeamData = response.json().await?;
        Ok(all_player_data)
    } else {
        Err(format!("Failed to fetch data: HTTP {}", response.status()).into())
    }
}