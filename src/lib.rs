use std::{
    sync::{Arc, RwLock},
    collections::HashMap,
};

pub enum Query {
    Ping,
    Set { key: String, value: String },
    Get(String),
    Del(String),
}

pub fn parse(request: Vec<&str>) -> Result<Query, String> {
    let response = match request[0].to_uppercase().as_str() {
        "PING" => Ok(Query::Ping),
        "SET" => {
            if let (Some(key), Some(value)) = (request.get(1), request.get(2)) {
                Ok(Query::Set{ key: key.to_string(), value: value.to_string() })
            } else {
                Err("ERROR: Missing key or value\r\n".to_string())
            }
        },
        "GET" => {
            if let Some(key) = request.get(1) {
                Ok(Query::Get(key.to_string()))
            } else {
                Err("ERROR: Missing key\r\n".to_string())
            }
        },
        "DEL" => {
            if let Some(key) = request.get(1) {
                Ok(Query::Del(key.to_string()))
            } else {
                Err("ERROR: Missing key\r\n".to_string())
            }
        },
        _ => Err("Invalid Command\r\n".to_string()),
    };

    response
}

pub fn execute(query: Query, storage: &Arc<RwLock<HashMap<String, String>>>) -> String {
    match query {
        Query::Ping => "PONG\r\n".to_string(),
        Query::Set { key, value } => set(&key, &value, storage),
        Query::Get(key) => get(&key, storage),
        Query::Del(key) => delete(&key, storage),
    }
}

fn set(key: &str, value: &str, storage: &Arc<RwLock<HashMap<String, String>>>) -> String {
    let mut map = match storage.write() {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error occured while aquiring write lock: {e}");
            return "ERROR: Internal Server Error\r\n".to_string();
        }
    };
    map.insert(
        key.to_string(),
        value.to_string(),
    );
    "Done\r\n".to_string()
}

fn get(key: &str, storage: &Arc<RwLock<HashMap<String,String>>>) -> String {
    let map = match storage.read() {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error occured while aquiring read lock: {e}");
            return "ERROR: Internal Server Error\r\n".to_string();
        }
    };

    if let "all" = key {
        return format!("{:#?}\r\n", map);
    }

    match map.get(key) {
        Some(value) => return format!("{value}\r\n"),
        None => "Not found\r\n".to_string(),
    }
}

fn delete(key: &str, storage: &Arc<RwLock<HashMap<String,String>>>) -> String {
    let mut map = match storage.write() {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error occured while aquiring write lock: {e}");
            return "ERROR: Internal Server Error\r\n".to_string();
        }
    };

    if let None = map.remove(key) {
        "Not found\r\n".to_string()
    } else {
        "Deleted\r\n".to_string()
    }
}
