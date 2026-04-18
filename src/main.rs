use std::{
    env,
    sync::{Arc, RwLock},
    collections::HashMap,
};
use tokio::{
    net::{ TcpListener },
    io::{ BufReader, AsyncBufReadExt, AsyncWriteExt },
};
use fetis::{ parse, execute };

#[tokio::main]
async fn main() {

    // Load arguements & set port
    let args: Vec<String> = env::args().collect();
    let mut port = &"8080".to_string();
    if args.len() > 1 {
        port = &args[1];
    };

    // Format url to bind TcpListener
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

    // Create the main storage
    let storage = Arc::new(RwLock::new(HashMap::<String, String>::new()));

    // Start listening for connections
    loop {

        // Get the connection
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

        // Clone the storage for new task
        let storage = Arc::clone(&storage);

        tokio::spawn(async move {

            // Split the socket for separate usage without problem
            let (reader, mut writer) = socket.split();

            // Get the request buffer
            let buf_reader = BufReader::new(reader);
            let mut lines = buf_reader.lines();

            // Wait for incoming requests and handle them
            while let Ok(Some(line)) = lines.next_line().await {

                // Get first two words, then rest of the line as the third
                let request: Vec<&str> = line.splitn(3, ' ').collect();

                // Break the connection if the request is empty
                // Doing otherwise would cause infinite loop
                if request[0].is_empty() {
                    break;
                }

                // Parse the request query and execute it & also get the response
                let response = match parse(request) {
                    Ok(query) => execute(query, &storage),
                    Err(e) => e,
                };
                // println!("Map: {:#?}", storage.read().unwrap()); // For Debug

                // Write back to the client
                if let Err(e) = writer.write_all(response.as_bytes()).await {
                    eprintln!("Error while writing back to client: {e}");
                    break;
                }
            }
            println!("Client disconnected!");
        });
    }
}

