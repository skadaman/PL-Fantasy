use reqwest;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct FplData {
    pub elements: HashMap<String, Element>,
    pub fixtures: Vec<Fixture>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Element {
    pub explain: Vec<Vec<Vec<ExplainItem>>>,
    pub stats: Stats,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExplainItem {
    pub name: String,
    pub points: i32,
    pub value: i32,
    pub stat: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub minutes: i32,
    pub goals_scored: i32,
    pub assists: i32,
    pub clean_sheets: i32,
    pub goals_conceded: i32,
    pub own_goals: i32,
    pub penalties_saved: i32,
    pub penalties_missed: i32,
    pub yellow_cards: i32,
    pub red_cards: i32,
    pub saves: i32,
    pub bonus: i32,
    pub bps: i32,
    pub influence: f64,
    pub creativity: f64,
    pub threat: f64,
    pub ict_index: f64,
    pub starts: i32,
    pub expected_goals: f64,
    pub expected_assists: f64,
    pub expected_goal_involvements: f64,
    pub expected_goals_conceded: f64,
    pub total_points: i32,
    pub in_dreamteam: bool,
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Fixture {
    pub id: i32,
    pub started: bool,
    pub stats: Vec<FixtureStat>,
    pub code: i64,
    pub finished: bool,
    pub finished_provisional: bool,
    pub kickoff_time: String,
    pub minutes: i32,
    pub provisional_start_time: bool,
    pub team_a_score: i32,
    pub team_h_score: i32,
    pub pulse_id: i32,
    pub event: i32,
    pub team_a: i32,
    pub team_h: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FixtureStat {
    pub s: String,
    pub h: Vec<StatItem>,
    pub a: Vec<StatItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatItem {
    pub element: i32,
    pub value: i32,
}


pub async fn fetch_weekly_result_data(week: u32) -> Result<HashMap<String, Element>, Box<dyn std::error::Error>> {
    let url = format!("https://draft.premierleague.com/api/event/{}/live", week);

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;

    if response.status().is_success() {
        let all_data: FplData = response.json().await?;
        // The explain struct has the summary data for the player performance. 
        // It is nested inside a hash map where the index is equal to the player id in 
        // the player data.
        let weekly_result_data: HashMap<String, Element> = all_data.elements;
        Ok(weekly_result_data)
    } else {
        Err(format!("Failed to fetch data: HTTP {}", response.status()).into())
    }
}