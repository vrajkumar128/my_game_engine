mod network;
mod sprite_data;

use my_game_engine::ffi::{clear, GLFW_KEY_SPACE};
use my_game_engine::{on_key_press, spawn_sprite, start_window_and_game_loop};
use network::{NetworkMessage, NetworkResponse, start_network_thread};
use sprite_data::SpriteData;
use std::sync::mpsc::{channel};

fn main() {
    // Set up communication channels
    let (main_to_network_tx, main_to_network_rx) = channel::<NetworkMessage>();
    let (network_to_main_tx, network_to_main_rx) = channel::<NetworkResponse>();
    
    // Start network thread
    let network_thread = start_network_thread(main_to_network_rx, network_to_main_tx);
    
    // Store all sprites
    let mut sprites: Vec<SpriteData> = Vec::new();
    
    // Main game loop
    start_window_and_game_loop!(
        "Rust Network Sprite Game",
        800,
        600,
        16,
        {},
        {
            clear();
            
            on_key_press!(GLFW_KEY_SPACE, {
                // Request a new sprite from the network thread
                let _ = main_to_network_tx.send(NetworkMessage::FetchSprite);
            });
            
            // Check for messages from the network thread
            if let Ok(response) = network_to_main_rx.try_recv() {
                match response {
                    NetworkResponse::NewSprite(sprite_data) => {
                        sprites.push(sprite_data);
                    }
                }
            }
            
            // Render sprites
            for sprite in &sprites {
                spawn_sprite!(
                    sprite.x as f32, 
                    sprite.y as f32, 
                    sprite.width, 
                    sprite.height, 
                    sprite.r, 
                    sprite.g, 
                    sprite.b
                );
            }
        },
        {
            let _ = main_to_network_tx.send(NetworkMessage::Quit); // Send quit message to network thread
            let _ = network_thread.join(); // Wait for network thread to finish
        }
    );
}