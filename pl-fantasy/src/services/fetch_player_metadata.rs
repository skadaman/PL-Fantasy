use reqwest;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerMetaData {
   pub elements: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
   pub id: u32,
   pub assists: u32,
   pub bonus: u32,
   pub bps: u32,
   pub clean_sheets: u32,
   pub creativity: String,
   pub goals_conceded: u32,
   pub goals_scored: u32,
   pub ict_index: String,
   pub influence: String,
   pub minutes: u32,
   pub own_goals: u32,
   pub penalties_missed: u32,
   pub penalties_saved: u32,
   pub red_cards: u32,
   pub saves: u32,
   pub threat: String,
   pub yellow_cards: u32,
   pub starts: u32,
   pub expected_goals: String,
   pub expected_assists: String,
   pub expected_goal_involvements: String,
   pub expected_goals_conceded: String,
   pub added: DateTime<Utc>,
   pub chance_of_playing_next_round: Option<u8>,
   pub chance_of_playing_this_round: Option<u8>,
   pub code: u32,
   pub draft_rank: u32,
   pub dreamteam_count: u32,
   pub ep_next: Option<String>,
   pub ep_this: Option<String>,
   pub event_points: u32,
   pub first_name: String,
   pub form: String,
   pub in_dreamteam: bool,
   pub news: String,
   pub news_added: Option<DateTime<Utc>>,
   pub news_return: Option<String>,
   pub news_updated: Option<DateTime<Utc>>,
   pub points_per_game: String,
   pub second_name: String,
   pub squad_number: Option<u32>,
   pub status: String,
   pub total_points: u32,
   pub web_name: String,
   pub influence_rank: u32,
   pub influence_rank_type: u32,
   pub creativity_rank: u32,
   pub creativity_rank_type: u32,
   pub threat_rank: u32,
   pub threat_rank_type: u32,
   pub ict_index_rank: u32,
   pub ict_index_rank_type: u32,
   pub form_rank: Option<u32>,
   pub form_rank_type: Option<u32>,
   pub points_per_game_rank: Option<u32>,
   pub points_per_game_rank_type: Option<u32>,
   pub corners_and_indirect_freekicks_order: Option<u32>,
   pub corners_and_indirect_freekicks_text: String,
   pub direct_freekicks_order: Option<u32>,
   pub direct_freekicks_text: String,
   pub penalties_order: Option<u32>,
   pub penalties_text: String,
   pub element_type: u32,
   pub team: u32,
}

pub async fn fetch_player_metadata() -> Result<FilteredPlayerMetaData, Box<dyn std::error::Error>> {
    let url = format!("https://draft.premierleague.com/api/bootstrap-static");

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let all_player_data: PlayerMetaData = response.json().await?;
        let player_data: filter_player_data(all_player_data);
        Ok(player_data)
    } else {
        Err(format!("Failed to fetch data: HTTP {}", response.status()).into())
    }
}

pub struct FilteredPlayerMetaData {
    pub elements: Vec<SimplePlayer>,
}

pub struct SimplePlayer {
    pub id: u32,
    pub team: u32,
    pub web_name: String,
}


pub fn filter_player_data(long_data: PlayerMetaData) -> FilteredPLayerData{
    FilteredPlayerMetaData {
        elements: long_data.elements.into_iter().map(|player| SimplePlayer {
            id: player.id,
            team: player.team,
            web_name: player.web_name,
        }).collect(),
    }
}