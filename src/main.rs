use std::{
    env,
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

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let buf_reader = BufReader::new(reader);

            let mut lines = buf_reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                if let Err(e) = writer.write_all(line.as_bytes()).await {
                    eprintln!("Error while writing back to client: {e}");
                    break;
                }
            }

            println!("Client disconnected!");
        });
    }
}
