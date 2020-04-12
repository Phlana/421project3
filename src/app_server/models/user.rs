use serde::{Serialize, Deserialize};
pub use crate::app_server::models::db_object::{DBObject};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
}

impl DBObject for User {
    fn new() -> Self {
        User {
            id: "-1".to_string(),
            name: "".to_string()
        }
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

