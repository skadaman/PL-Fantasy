mod services;

use services::{fetch_league_data, fetch_team_data, fetch_player_metadata, fetch_weekly_result_data };
use std::env;
use services::config::{PULL_PLAYER_METADATA, DEFAULT_WEEK};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let league_id: u32 = 46301;
    let args: Vec<String> = env::args().collect();
    
    let week = if args.len() > 1 {
        args[1].parse().unwrap_or(DEFAULT_WEEK)
    } else {
        DEFAULT_WEEK
    };
    let team_id: u32 = 321992;

    if PULL_PLAYER_METADATA {
        match fetch_player_metadata::fetch_player_metadata().await {

            Ok(player_metadata) => println!("Pulled player metadata.") ,
            Err(e) => println!("Error: {}", e),
        }
    }
    // TODO: else statement where we just read from file. This can late be a db store. 

    match fetch_league_data::fetch_league_data(league_id).await {
        Ok(league_data) =>{
            println!("Pulled data for league: {}", league_data.league.name);
        },
        Err(e) => println!("Error: {}", e),
    }

    match fetch_team_data::fetch_team_data(team_id, week).await {

        Ok(team_data) => println!("Pulled team selection data."),
        Err(e) => println!("Error: {}", e),
    }

    match fetch_weekly_result_data::fetch_weekly_result_data(week).await {

        Ok(weekly_result_data) => println!("Pulled weekly result data."),
        Err(e) => println!("Error: {}", e),
    }

    //TODO: Collate the structs into a single struct with the dimensions that we want. 
    Ok(())
}