mod services;

use services::{fetch_league_data, fetch_team_data};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let league_id = 46301;

    match fetch_league_data::fetch_league_data(league_id).await {
        Ok(pulled_league_data) =>{
            println!("League name: {}", pulled_league_data.league.name);
        },
        Err(e) => println!("Error: {}", e),
    }
    let team_id = 321992;

    match fetch_team_data::fetch_team_data(team_id).await {

        Ok(team_data) => println!("Team data: {}", serde_json::to_string_pretty(&team_data)?),
        Err(e) => println!("Error: {}", e),
    }

    Ok(())
}