use std::{
    env,
    sync::{Arc, RwLock},
    collections::HashMap,
};

use tokio::{
    net::{ TcpListener },
    io::{ BufReader, AsyncBufReadExt, AsyncWriteExt },
};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut port = &"8080".to_string();
    if args.len() > 1 {
        port = &args[1];
    };
    let url = format!("127.0.0.1:{port}");

    let listener = match TcpListener::bind(url).await {
        Ok(listener) =>{
            println!("Server running at localhost:{port}");
            listener
        },
        Err(e) => {
            eprintln!("Error occured while trying to bind socket: {e}");
            return;
        }
    };

    let storage = Arc::new(RwLock::new(HashMap::<String, String>::new()));

    loop {
        let (mut socket, _) = match listener.accept().await {
            Ok((socket, addr)) => {
                println!("Client connected: {:?}", addr);
                (socket, addr)
            },
            Err(e) => {
                eprintln!("Connection error: {e}");
                continue;
            }
        };


        let storage = Arc::clone(&storage);

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let buf_reader = BufReader::new(reader);

            let mut lines = buf_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {

                let request: Vec<&str> = line.splitn(3, ' ').collect();
                if request[0].is_empty() {
                    break;
                }

                let response = match request[0].to_uppercase().as_str() {
                    "PING" => "PONG\r\n".to_string(),
                    "SET" => {
                        if let (Some(key), Some(value)) = (request.get(1), request.get(2)) {
                            set(key, value, &storage)
                        } else {
                            "ERROR: Missing key or value\r\n".to_string()
                        }
                    },
                    "GET" => {
                        if let Some(key) = request.get(1) {
                            get(key, &storage)
                        } else {
                            "ERROR: Missing key\r\n".to_string()
                        }
                    }
                    _ => "Invalid Command\r\n".to_string(),
                };

                // println!("Map: {:#?}", storage.read().unwrap()); // For Debug

                if let Err(e) = writer.write_all(response.as_bytes()).await {
                    eprintln!("Error while writing back to client: {e}");
                    break;
                }
            }

            println!("Client disconnected!");
        });
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
