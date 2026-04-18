//! # Fetis library
//!
//! `fetis` library provides the core functionality for the Fetis key-value store server. It
//! includes parsing of client requests and executing commands on the in-memory storage.

use std::{
    sync::{Arc, RwLock},
    collections::HashMap,
};

// Define query enum for proper query parsing and access
#[derive(Debug, PartialEq)]
pub enum Query {
    Ping,
    Set { key: String, value: String },
    Get(String),
    Del(String),
}

/// Parses the given request.
///
/// # Examples
///
/// ```
/// let request = vec!["SET", "key", "value"];
/// let query = fetis::parse(request).unwrap();
///
/// assert_eq!(query, fetis::Query::Set { key: "key".to_string(), value: "value".to_string() });
/// ```
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

/// Executes the given query and returns the response as a string.
///
/// # Examples
///
/// ```
/// use std::{
///    sync::{Arc, RwLock},
///    collections::HashMap,
/// };
///
/// let query = fetis::Query::Ping;
/// let response = fetis::execute(query, &Arc::new(RwLock::new(HashMap::new())));
/// assert_eq!(response, "PONG\r\n".to_string());
/// ```
pub fn execute(query: Query, storage: &Arc<RwLock<HashMap<String, String>>>) -> String {
    match query {
        Query::Ping => "PONG\r\n".to_string(),
        Query::Set { key, value } => set(&key, &value, storage),
        Query::Get(key) => get(&key, storage),
        Query::Del(key) => delete(&key, storage),
    }
}


// Function to execute a set query
// Adds given key value pair to the storage
fn set(key: &str, value: &str, storage: &Arc<RwLock<HashMap<String, String>>>) -> String {
    let mut map = match storage.write() {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error occured while acquiring write lock: {e}");
            return "ERROR: Internal Server Error\r\n".to_string();
        }
    };
    map.insert(
        key.to_string(),
        value.to_string(),
    );
    "Done\r\n".to_string()
}

// Function to execute a get query
// Returns the value of the given key if it exists, otherwise returns "Not found\r\n"
// Returns everything in the storage if the key is "all"
fn get(key: &str, storage: &Arc<RwLock<HashMap<String,String>>>) -> String {
    let map = match storage.read() {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error occured while acquiring read lock: {e}");
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

// Function to execute a delete query
// Deletes the given key from the storage if it exists, otherwise returns "Not found\r\n"
fn delete(key: &str, storage: &Arc<RwLock<HashMap<String,String>>>) -> String {
    let mut map = match storage.write() {
        Ok(map) => map,
        Err(e) => {
            eprintln!("Error occured while acquiring write lock: {e}");
            return "ERROR: Internal Server Error\r\n".to_string();
        }
    };

    if let None = map.remove(key) {
        "Not found\r\n".to_string()
    } else {
        "Deleted\r\n".to_string()
    }
}
