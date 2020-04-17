pub use crate::app_server::models::db_object::{DBObject, Documentable}; 
use r2d2::PooledConnection;
use serde::{Serialize, Deserialize};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row, Error, NO_PARAMS};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub name: Option<String>,
    pub hashed_password: Option<String>,
}

impl Documentable for User {
    fn get_collection_name() -> String {
        "users".to_string()
    }

    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn new() -> Self {
        User {
            id: None,
            name: None,
            hashed_password: None,
        }
    }

    fn from_row(row: &Row) -> Result<Self, Error>{
        Ok(User{
            id: row.get(0)?,
            name: row.get(1)?,
            hashed_password: row.get(2)?
        })
    }

    fn to_insert(&self) -> String {
        let uid = match self.id.clone() {
            Some(i) => i,
            None => "".to_string(),
        };
        let uname = match self.name.clone() {
            Some(n) => n,
            None => "".to_string(),
        };
        let upass = match self.hashed_password.clone() {
            Some(h) => h,
            None => "".to_string(),
        };
        format!("\"{}\", \"{}\", \"{}\"", uid, uname, upass)
    }

    // Note: Can't update user id, only name and hashed password
    fn to_update(&self) -> String {
        let mut result = "".to_string();
        
        match self.name.clone() {
            Some(n) => {result = result + format!("\"name\" = \"{}\"", n).as_str();},
            None => {},
        };
        match self.hashed_password.clone() {
            Some(h) => {result = result + format!("\"hashed_password\" = \"{}\"", h).as_str();},
            None => {},
        };

        return result;
    }
}

impl DBObject for User {
    fn insert(&mut self, conn: PooledConnection<SqliteConnectionManager>) -> Result<(),String>{
        if self.get_id() == None {
            return Err("Unable to create a user without an id".to_string());
        }
        if self.hashed_password == None {
            return Err("Unable to create a user without a hashed_password".to_string());
        }
        else {
            let command = format!("INSERT INTO {} VALUES({})", Self::get_collection_name(), self.to_insert());
            match conn.execute(command.as_str(), NO_PARAMS){
                Ok(_) => Ok(()),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}