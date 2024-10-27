mod contexts;
mod engine;
mod keycode_macos;
mod mappings;
mod semantics;

pub use crate::contexts::KeyEvent;
pub use crate::contexts::KeyState;
pub use crate::engine::PinkyTwirlEngine;

#[swift_bridge::bridge]
mod ff {
    extern "Rust" {
        type KeyEvent;
        type KeyState;
    }

    extern "Rust" {
        type PinkyTwirlEngine;

        #[swift_bridge(associated_to = PinkyTwirlEngine)]
        fn new(config_dir: String) -> PinkyTwirlEngine;

        fn macos_handle_key_event(
            &mut self,
            key_code: u16,
            down: bool,
            shift: bool,
            ctrl: bool,
            option: bool,
            meta: bool,
            app_name: &str,
            window_name: &str,
        ) -> Vec<KeyEvent>;    
    }

    extern "Rust" {
        fn print_hello_rust();
    }
}

fn print_hello_rust() {
    println!("Hello, Rust!");
}