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
    #[swift_bridge(already_declared, swift_repr = "struct")]
    struct KeyEvent;

    extern "Rust" {
        // type KeyEvent;

        // fn key(self: &KeyEvent) -> &str;
        // fn get_code(self: &KeyEvent) -> i64;
        // fn state(self: &KeyEvent) -> &KeyState;
        // fn shift(self: &KeyEvent) -> bool;
        // fn ctrl(self: &KeyEvent) -> bool;
        // fn alt(self: &KeyEvent) -> bool;        
        // fn meta(self: &KeyEvent) -> bool;
        // fn is_down(self: &KeyState) -> bool;
    }

    extern "Rust" {
        type PinkyTwirlEngine;

        #[swift_bridge(associated_to = PinkyTwirlEngine)]
        fn new(config_dir: String) -> PinkyTwirlEngine;

        fn macos_handle_key_event(
            &mut self,
            key_code: i64,
            down: bool,
            shift: bool,
            ctrl: bool,
            option: bool,
            meta: bool,
            app_name: &str,
            window_name: &str,
        ) -> bool;

        fn get_synthetic_events(&mut self) -> Vec<KeyEvent>;
    }
}