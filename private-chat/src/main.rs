use std::net::TcpListener;
use std::thread;
use tungstenite::accept;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]  // Added Debug here
struct Message {
    username: String,
    content: String,
}

fn main() {
    let server = TcpListener::bind("localhost:8080").unwrap();
    println!("Chat server started on ws://localhost:8080");

    for stream in server.incoming() {
        thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                let msg = match websocket.read_message() {
                    Ok(msg) => msg,
                    Err(e) => {
                        eprintln!("Error reading message: {:?}", e);
                        return;
                    }
                };

                if msg.is_text() {
                    let message: Message = match serde_json::from_str(&msg.to_string()) {
                        Ok(m) => m,
                        Err(e) => {
                            eprintln!("Error parsing message: {:?}", e);
                            continue;
                        }
                    };
                    println!("Received message: {:?}", message);  // Debug format is now valid
                }
            }
        });
    }
}
