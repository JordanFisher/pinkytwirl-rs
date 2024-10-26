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
    }

    extern "Rust" {
        fn print_hello_rust();
    }
}

fn print_hello_rust() {
    println!("Hello, Rust!");
}