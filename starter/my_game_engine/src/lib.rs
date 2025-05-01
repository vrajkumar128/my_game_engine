pub mod ffi;

#[macro_use]
pub mod macros;

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};
    use crate::ffi::{
        new_sprite, clear, GLFW_KEY_SPACE, GLFW_KEY_UP, 
        GLFW_KEY_DOWN, GLFW_KEY_LEFT, GLFW_KEY_RIGHT
    };

    // #[test]
    // #[ignore]
    // fn test_simple_game_loop() {
    //     create_window("Test Game Loop", 800, 600);

    //     while !should_close() {
    //         update_window();
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // }

    #[test]
    #[ignore]
    fn test_simple_game_loop() {
        crate::start_window_and_game_loop!(
            "Test Game Loop",
            800,
            600,
            16,
            {},
            {},
            {}
        );
    }

    // #[test]
    // #[ignore]
    // fn test_sprite_rendering() {
    //     create_window("Test Sprite Rendering", 800, 600);
    //     let sprite = new_sprite(100.0, 150.0, 50, 50, 255, 0, 0);

    //     while !should_close() {
    //         clear();
    //         draw_sprite(sprite);
    //         update_window();
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // }

    #[test]
    #[ignore]
    fn test_sprite_rendering() {
        crate::start_window_and_game_loop!(
            "Test Sprite Rendering",
            800,
            600,
            16,
            {
                let _sprite = crate::spawn_sprite!(100.0, 150.0, 50, 50, 255, 0, 0);
            },
            {
                clear();
                let _sprite = crate::spawn_sprite!(100.0, 150.0, 50, 50, 255, 0, 0);
            },
            {}
        );
    }

    // #[test]
    // #[ignore]
    // fn test_screen_clearing() {
    //     create_window("Test Screen Clearing", 800, 600);

    //     let red_sprite = new_sprite(100.0, 150.0, 50, 50, 255, 0, 0);
    //     let green_sprite = new_sprite(200.0, 300.0, 60, 60, 0, 255, 0);
        
    //     let start_time = std::time::Instant::now();
    //     let switch_time = Duration::from_secs(5);
    //     let mut showing_red = true;
        
    //     while !should_close() {
    //         clear();
            
    //         if showing_red && start_time.elapsed() >= switch_time {
    //             showing_red = false;
    //         }
            
    //         if showing_red {
    //             draw_sprite(red_sprite);
    //         } else {
    //             draw_sprite(green_sprite);
    //         }
            
    //         update_window();
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // }

    #[test]
    #[ignore]
    fn test_screen_clearing() {
        let start_time = std::time::Instant::now();
        let switch_time = Duration::from_secs(5);
        let mut showing_red = true;
        
        crate::start_window_and_game_loop!(
            "Test Screen Clearing",
            800,
            600,
            16,
            {},
            {
                clear();
                if showing_red && start_time.elapsed() >= switch_time {
                    showing_red = false;
                }
                
                if showing_red {
                    let _red_sprite = crate::spawn_sprite!(100.0, 150.0, 50, 50, 255, 0, 0);
                } else {
                    let _green_sprite = crate::spawn_sprite!(200.0, 300.0, 60, 60, 0, 255, 0);
                }
            },
            {}
        );
    }

    // #[test]
    // #[ignore]
    // fn test_key_presses() {
    //     create_window("Test Key Presses", 800, 600);

    //     let mut space_pressed = false;
    //     let mut up_pressed = false;
    //     let mut down_pressed = false;
    //     let mut left_pressed = false;
    //     let mut right_pressed = false;
        
    //     let sprite = new_sprite(350.0, 250.0, 100, 100, 0, 0, 255);
        
    //     while !should_close() {
    //         clear();
            
    //         // Check for key presses
    //         if is_key_pressed(GLFW_KEY_SPACE) {
    //             space_pressed = true;
    //         }
            
    //         if is_key_pressed(GLFW_KEY_UP) {
    //             up_pressed = true;
    //         }
            
    //         if is_key_pressed(GLFW_KEY_DOWN) {
    //             down_pressed = true;
    //         }
            
    //         if is_key_pressed(GLFW_KEY_LEFT) {
    //             left_pressed = true;
    //         }
            
    //         if is_key_pressed(GLFW_KEY_RIGHT) {
    //             right_pressed = true;
    //         }
            
    //         // Change sprite color based on pressed keys
    //         let red = if space_pressed { 255 } else { 0 };
    //         let green = if up_pressed && down_pressed { 255 } else { 0 };
    //         let blue = if left_pressed && right_pressed { 255 } else { 0 };

    //         let colored_sprite = new_sprite(350.0, 250.0, 100, 100, red, green, blue);
            
    //         draw_sprite(colored_sprite);
    //         update_window();
            
    //         if space_pressed && up_pressed && down_pressed && left_pressed && right_pressed {
    //             break;
    //         }
            
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // }

    #[test]
    #[ignore]
    fn test_key_presses() {
        let mut space_pressed = false;
        let mut up_pressed = false;
        let mut down_pressed = false;
        let mut left_pressed = false;
        let mut right_pressed = false;
        
        crate::start_window_and_game_loop!(
            "Test Key Presses",
            800,
            600,
            16,
            {},
            {
                clear();
                
                crate::on_key_press!(GLFW_KEY_SPACE, {
                    space_pressed = true;
                });
                
                crate::on_key_press!(GLFW_KEY_UP, {
                    up_pressed = true;
                });
                
                crate::on_key_press!(GLFW_KEY_DOWN, {
                    down_pressed = true;
                });
                
                crate::on_key_press!(GLFW_KEY_LEFT, {
                    left_pressed = true;
                });
                
                crate::on_key_press!(GLFW_KEY_RIGHT, {
                    right_pressed = true;
                });
                
                // Create a different colored sprite for each key press
                if space_pressed {
                    crate::spawn_sprite!(250.0, 150.0, 80, 80, 255, 0, 0);
                }
                
                if up_pressed {
                    crate::spawn_sprite!(350.0, 150.0, 80, 80, 0, 255, 0);
                }
                
                if down_pressed {
                    crate::spawn_sprite!(450.0, 150.0, 80, 80, 0, 0, 255);
                }
                
                if left_pressed {
                    crate::spawn_sprite!(300.0, 250.0, 80, 80, 255, 255, 0);
                }
                
                if right_pressed {
                    crate::spawn_sprite!(400.0, 250.0, 80, 80, 255, 0, 255);
                }
                
                if space_pressed && up_pressed && down_pressed && left_pressed && right_pressed {
                    // sleep(std::time::Duration::from_secs(2));
                    break;
                }
            },
            {}
        );
    }

    // #[test]
    // #[ignore]
    // fn test_sprite_position_update() {
    //     create_window("Test Sprite Position Update", 800, 600);
    //     let sprite = new_sprite(100.0, 300.0, 50, 50, 0, 0, 255);
        
    //     let mut x = 100.0;
    //     let y = 300.0;
    //     let speed = 2.0;
        
    //     while !should_close() && x < 700.0 {
    //         clear();
    //         x += speed;
    //         set_sprite_position(sprite, x, y);
    //         draw_sprite(sprite);
    //         update_window();
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // }

    #[test]
    #[ignore]
    fn test_sprite_position_update() {
        let mut x = 100.0;
        let y = 300.0;
        let speed = 2.0;
        let sprite = new_sprite(100.0, 300.0, 50, 50, 0, 0, 255);
        
        crate::start_window_and_game_loop!(
            "Test Sprite Position Update",
            800,
            600,
            16,
            {},
            {
                clear();
                x += speed;
                crate::move_sprite!(sprite, x, y, false);
                
                if x >= 700.0 {
                    break;
                }
            },
            {}
        );
    }
}