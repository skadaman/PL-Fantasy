mod services;

use services::{fetch_league_data, fetch_player_metadata::{self, PlayerMetaData}, fetch_team_data, fetch_weekly_result_data };
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

    let team_selection = match fetch_team_data::fetch_team_data(team_id, week).await {

        Ok(data) => {
            println!("Pulled team selection data.");
            data
        },
        Err(e) => {
            println!("Error: {}", e);
            return Err(e.into());
        }
    };
    if PULL_PLAYER_METADATA{
        let player_metadata: fetch_player_metadata::FilteredPlayerMetaData = match fetch_player_metadata::fetch_player_metadata().await {

            Ok(data) => {
                println!("Pulled player metadata.");
                data
            } ,
            Err(e) => {
                println!("Error: {}", e);
                return Err(e.into());
            },
        };
    }
    // TODO: else statement where we just read from file. This can late be a db store. 

   let league_data = match fetch_league_data::fetch_league_data(league_id).await {
        Ok(data) =>{
            println!("Pulled data for league: {}", data.league.name);
            data
        },
        Err(e) => {
            println!("Error: {}", e);
            return Err(e.into());
        },
    };

    let weekly_result = match fetch_weekly_result_data::fetch_weekly_result_data(week, team_selection).await {

        Ok(data) => {
            println!("Pulled weekly result data.");
            data
        },
        Err(e) => {
        println!("Error: {}", e);
        return Err(e.into());
        }
    };

    //TODO: Collate the structs into a single struct with the dimensions that we want. 
    Ok(())
}


#[derive(Debug)]
pub struct EnrichedTeamResult {
    pub element: u32,
    pub position: u8,
    pub total_points: i32,
    pub bonus: i32,
    pub explain: Vec<ExplainItem>,
    pub team: u32,
    pub web_name: String,
}

pub fn enrich_team_result(combined_data: Vec<CombinedTeamResult>, player_metadata: FilteredPlayerMetaData) -> Vec<EnrichedTeamResult> {
    let player_map: HashMap<u32, &SimplePlayer> = player_metadata.elements
        .iter()
        .map(|player| (player.id, player))
        .collect();

    combined_data.into_iter()
        .filter_map(|result| {
            player_map.get(&result.element).map(|&player| {
                EnrichedTeamResult {
                    element: result.element,
                    position: result.position,
                    total_points: result.total_points,
                    bonus: result.bonus,
                    explain: result.explain,
                    team: player.team,
                    web_name: player.web_name.clone(),
                }
            })
        })
        .collect()
}