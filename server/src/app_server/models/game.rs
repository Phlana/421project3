pub use crate::app_server::models::db_object::{DBObject, Documentable}; 
use r2d2::PooledConnection;
use serde::{Serialize, Deserialize};
use chrono::prelude::*;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row, Error, NO_PARAMS};

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "id")]
    pub game_number: Option<String>,
    pub game_type: Option<String>,
    pub player_1_name: Option<String>,
    pub player_2_name: Option<String>,
    pub winner_name: Option<String>,
    pub text: Option<String>,
    pub created_by: Option<String>,
    pub game_date: Option<String>
}

impl Documentable for Game {
    fn get_collection_name() -> String {
        "games".to_string()
    }

    fn get_id(&self) -> Option<String> {
        self.game_number.clone()
    }

    fn new() -> Self {
        Game {
            game_number: None,
            game_type: None,
            player_1_name: None,
            player_2_name: None,
            winner_name: None,
            text: None,
            created_by: None,
            game_date: None,
        }
    }

    fn from_row(row: &Row) -> Result<Self, Error>{
        Ok(Game{
            game_number: row.get(0)?,
            game_type: row.get(1)?,
            player_1_name: row.get(2)?,
            player_2_name: row.get(3)?,
            winner_name: row.get(4)?,
            text: row.get(5)?,
            created_by: row.get(6)?,
            game_date: row.get(7)?,
        })
    }

    fn to_insert(&self) -> String {
        let ugame_number = match self.game_number.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let ugame_type = match self.game_type.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let uplayer_1_name = match self.player_1_name.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let uplayer_2_name = match self.player_2_name.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let uwinner_name = match self.winner_name.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let utext = match self.text.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let ucreated_by = match self.created_by.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        let ugame_date = match self.game_date.clone() {
            Some(s) => s,
            None => "".to_string(),
        };
        format!("\"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\", \"{}\"", 
            ugame_number, ugame_type, uplayer_1_name, uplayer_2_name, 
            uwinner_name, utext, ucreated_by, ugame_date)
    }

    // Note: Can't update user id, only name and hashed password
    fn to_update(&self) -> String {
        let mut result = "".to_string();

        match self.game_type.clone() {
            Some(s) => {result = result + format!("\"game_type\" = \"{}\"", s).as_str();},
            None => (),
        };
        match self.player_1_name.clone() {
            Some(s) => {result = result + format!("\"player_1_name\" = \"{}\"", s).as_str();},
            None => (),
        };
        match self.player_2_name.clone() {
            Some(s) => {result = result + format!("\"player_2_name\" = \"{}\"", s).as_str();},
            None => (),
        };
        match self.winner_name.clone() {
            Some(s) => {result = result + format!("\"winner_name\" = \"{}\"", s).as_str();},
            None => {},
        };
        match self.text.clone() {
            Some(s) => {result = result + format!("\"text\" = \"{}\"", s).as_str();},
            None => {},
        };
        match self.created_by.clone() {
            Some(s) => {result = result + format!("\"created_by\" = \"{}\"", s).as_str();},
            None => {},
        };
        match self.game_date.clone() {
            Some(s) => {result = result + format!("\"game_date\" = \"{}\"", s).as_str();},
            None => {},
        };

        return result;
    }
}

impl DBObject for Game {
    fn insert(&mut self, conn: PooledConnection<SqliteConnectionManager>) -> Result<(),String>{
        if self.get_id() == None {
            return Err("Unable to create a user without an id".to_string());
        }

        self.game_date = Some(Utc::now().to_string());
        let command = format!("INSERT INTO {} VALUES({})", Self::get_collection_name(), self.to_insert());
        match conn.execute(command.as_str(), NO_PARAMS){
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}

