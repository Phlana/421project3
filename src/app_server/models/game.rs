pub use crate::app_server::models::db_object::{db_object}; 
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub game_number: String,
    pub game_type: String,
    pub player_1_name: String,
    pub player_2_name: String,
    pub winner_name: String,
    pub text: String,
    pub created_by: String,
    pub game_date: String, //TODO: Change this to our date format of choice
}

impl db_object for Game {
    fn new() -> Self {
        Game {
            game_number: "-1".to_string(),
            game_type: "".to_string(),
            player_1_name: "".to_string(),
            player_2_name: "".to_string(),
            winner_name: "".to_string(),
            text: "".to_string(),
            created_by: "".to_string(),
            game_date: "".to_string()
        }
        // TODO: set date to today
    }

    fn find_all() -> Result<String,String> {
        Err("Function Not Yet Implemented".to_string())
    }

    fn find_by_id(id: String) -> Result<Self, String> {
        Err("Function Not Yet Implemented".to_string())
    }

    fn remove_by_id(id: String) -> Result<Self, String> {
        Err("Function Not Yet Implemented".to_string())
    }

    fn save(&self) -> Result<(),String> {
        Err("Function Not Yet Implemented".to_string())
        //TODO: save this game to mongo 
    }
}

