use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Row, NO_PARAMS, Error};

pub trait Documentable {
    fn get_collection_name() -> String;
    fn get_id(&self) -> Option<String>;
    fn new() -> Self;
    fn from_row(row: &Row) -> Result<Self, Error> where Self: std::marker::Sized;
    fn to_insert(&self) -> String;
    fn to_update(&self) -> String;
}

pub trait DBObject: Documentable{
    fn find_all(conn: PooledConnection<SqliteConnectionManager>) -> Result<Vec<Self>,String> where Self: std::marker::Sized{
        let command = format!("SELECT * FROM {}", Self::get_collection_name());
        let mut statement = match conn.prepare(command.as_str()) {
            Ok(s) => s,
            Err(e) => {return Err(e.to_string());}
        };

        let mut items = match statement.query_map(NO_PARAMS, |row| {
            Self::from_row(row)
        }) {
            Ok(i) => i,
            Err(e) => {return Err(e.to_string());}
        };

        let mut vec = Vec::new();
        for item in items {
            match item {
                Ok(i) => { vec.push(i) },
                Err(e) => { return Err(e.to_string()); }
            }
        }       

        Ok(vec)
    }

    fn find_by_id(conn: PooledConnection<SqliteConnectionManager>, id: String) -> Result<Self, String> where Self: Sized{
        let command = format!("SELECT * FROM {} WHERE id={}", Self::get_collection_name(), id);
        let mut statement = match conn.prepare(command.as_str()) {
            Ok(s) => s,
            Err(e) => {return Err(e.to_string());}
        };

        let mut items = match statement.query_map(NO_PARAMS, |row| {
            Self::from_row(row)
        }) {
            Ok(i) => i,
            Err(e) => {return Err(e.to_string());}
        };

        match items.next() {
            Some(item) => match item {
                Ok(i) => Ok(i),
                Err(e) => Err(e.to_string())
            },
            None => Err("No item found with the given id".to_string())
        }
    }

    fn remove_by_id(conn: PooledConnection<SqliteConnectionManager>, id: String) -> Result<Self, String> where Self: Sized {
        let command = format!("SELECT * FROM {} WHERE id={}", Self::get_collection_name(), id);
        let mut statement = match conn.prepare(command.as_str()) {
            Ok(s) => s,
            Err(e) => {return Err(e.to_string());}
        };

        let mut items = match statement.query_map(NO_PARAMS, |row| {
            Self::from_row(row)
        }) {
            Ok(i) => i,
            Err(e) => {return Err(e.to_string());}
        };

        match items.next() {
            Some(item) => match item {
                Ok(i) => {
                    let command = format!("DELETE FROM {} WHERE id=\"{}\"", Self::get_collection_name(), id);
                    match conn.execute(command.as_str(), NO_PARAMS) {
                        Ok(_) => Ok(i),
                        Err(e) => Err(e.to_string()),
                    }
                }
                Err(e) => {return Err(e.to_string());}                
            }
            None => Err("No item found with the given id".to_string())
        }
    }
    
    fn insert(&mut self, conn: PooledConnection<SqliteConnectionManager>) -> Result<(),String>{
        if self.get_id() == None {
            return Err("Unable to create a user without an id".to_string());
        }

        let command = format!("INSERT INTO {} VALUES({})", Self::get_collection_name(), self.to_insert());
        match conn.execute(command.as_str(), NO_PARAMS){
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn update(&mut self, conn: PooledConnection<SqliteConnectionManager>) -> Result<String,String>{
        if self.get_id() == None {
            return Err("Unable to save item without id".to_string());
        }

        let command = format!("UPDATE {} SET {} WHERE id = \"{}\"", Self::get_collection_name(), self.to_update(), self.get_id().unwrap());
        // Err(command)
        match conn.execute(command.as_str(), NO_PARAMS){
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(e.to_string()),
        }
    }
}

