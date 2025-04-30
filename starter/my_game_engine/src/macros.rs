#[macro_export]
macro_rules! spawn_sprite {
    ($x:expr, $y:expr, $width:expr, $height:expr, $r:expr, $g:expr, $b:expr) => {
        {
            let sprite = $crate::ffi::new_sprite($x, $y, $width, $height, $r, $g, $b);
            $crate::ffi::draw_sprite(sprite);
            sprite
        }
    };
}

#[macro_export]
macro_rules! on_key_press {
    ($key:expr, $action:block) => {
        if $crate::ffi::is_key_pressed($key) {
            $action
        }
    };
}

#[macro_export]
macro_rules! tick {
    ($sleep_ms:expr) => {
        {
            $crate::ffi::update_window();
            std::thread::sleep(std::time::Duration::from_millis($sleep_ms));
        }
    };
}

#[macro_export]
macro_rules! start_window_and_game_loop {
    ($title:expr, $width:expr, $height:expr, $sleep_ms:expr, $setup:block, $loop_body:block, $cleanup:block) => {
        {
            $crate::ffi::create_window($title, $width, $height);
            $setup
            
            while !$crate::ffi::should_close() {
                $loop_body
                $crate::tick!($sleep_ms);
            }
            
            $cleanup
        }
    };
}

#[macro_export]
macro_rules! move_sprite {
    ($sprite:expr, $x:expr, $y:expr, $clear_screen:expr) => {
        {
            if $clear_screen {
                $crate::ffi::clear();
            }
            $crate::ffi::set_sprite_position($sprite, $x, $y);
            $crate::ffi::draw_sprite($sprite);
        }
    };
}