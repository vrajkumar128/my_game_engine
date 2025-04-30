use crate::sprite_data::SpriteData;
use reqwest::blocking::Client;
use std::sync::mpsc::{Receiver, Sender};
use std::thread::{self, JoinHandle};

// Messages from main thread to network thread
pub enum NetworkMessage {
    FetchSprite,
    Quit,
}

// Messages from network thread to main thread
pub enum NetworkResponse {
    NewSprite(SpriteData),
}

pub fn start_network_thread(
    rx: Receiver<NetworkMessage>,
    tx: Sender<NetworkResponse>,
) -> JoinHandle<()> {
    let client = Client::new();
    
    thread::spawn(move || {
        let api_url = "https://get-random-sprite-data-dan-chiarlones-projects.vercel.app/api/handler";
        
        loop {
            match rx.recv() { // Check for messages from main thread
                Ok(NetworkMessage::FetchSprite) => {
                    if let Ok(sprite_data) = fetch_sprite_data(&client, api_url) { // Make the HTTP request to get sprite data
                        let _ = tx.send(NetworkResponse::NewSprite(sprite_data)); // Send sprite data back to main thread
                    }
                }
                Ok(NetworkMessage::Quit) => {
                    break;
                }
                Err(_) => {
                    break;
                }
            }
        }
    })
}

fn fetch_sprite_data(
    client: &Client,
    url: &str,
) -> Result<SpriteData, Box<dyn std::error::Error>> {
    let response = client.get(url).send()?;
    let sprite_data = response.json::<SpriteData>()?;
    Ok(sprite_data)
}