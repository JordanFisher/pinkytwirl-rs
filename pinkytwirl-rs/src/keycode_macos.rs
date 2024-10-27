use core::panic;
use std::collections::HashMap;

pub fn capitalize(s: String) -> String {
    // Capitalize the first letter of a string.
    let mut chars = s.chars();
    return match chars.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
    };
}

pub struct KeyCodeLookup {
    pub keycode_to_name: HashMap<i64, String>,
    pub name_to_keycode: HashMap<String, i64>,
}

impl KeyCodeLookup {
    pub fn add_pair(&mut self, keycode: i64, name: &str) {
        if !self.keycode_to_name.contains_key(&keycode) {
            self.keycode_to_name.insert(keycode, name.to_string());
            self.keycode_to_name
                .insert(keycode, name.to_string().to_lowercase());
        }
        if self.name_to_keycode.contains_key(name) {
            panic!("Key name '{}' already exists in the map", name);
        } else {
            self.name_to_keycode.insert(name.to_string(), keycode);
            self.name_to_keycode
                .insert(name.to_string().to_lowercase(), keycode);
            self.name_to_keycode
                .insert(capitalize(name.to_string().to_lowercase()), keycode);
            self.name_to_keycode
                .insert(name.to_string().to_uppercase(), keycode);
        }
    }
}

pub fn create_keycode_map() -> KeyCodeLookup {
    let mut lookup = KeyCodeLookup {
        keycode_to_name: HashMap::new(),
        name_to_keycode: HashMap::new(),
    };

    // Special keys
    lookup.add_pair(0x24, "returnKey");
    lookup.add_pair(0x24, "return");
    lookup.add_pair(0x4C, "enter");
    lookup.add_pair(0x30, "tab");
    lookup.add_pair(0x31, "space");
    lookup.add_pair(0x33, "backspace"); // Note: On macOS this is called `delete`.
    lookup.add_pair(0x35, "escape");
    lookup.add_pair(0x37, "meta"); // Note: On macOS this is called `command`.
    lookup.add_pair(0x38, "shift");
    lookup.add_pair(0x39, "capsLock");
    lookup.add_pair(0x3A, "option");
    lookup.add_pair(0x3B, "control");
    lookup.add_pair(0x36, "rightCommand");
    lookup.add_pair(0x3C, "rightShift");
    lookup.add_pair(0x3D, "rightOption");
    lookup.add_pair(0x3E, "rightControl");
    lookup.add_pair(0x3F, "function");

    // Arrow keys
    lookup.add_pair(0x7B, "leftArrow");
    lookup.add_pair(0x7B, "left");
    lookup.add_pair(0x7C, "rightArrow");
    lookup.add_pair(0x7C, "right");
    lookup.add_pair(0x7D, "downArrow");
    lookup.add_pair(0x7D, "down");
    lookup.add_pair(0x7E, "upArrow");
    lookup.add_pair(0x7E, "up");

    // Volume controls
    lookup.add_pair(0x48, "volumeUp");
    lookup.add_pair(0x49, "volumeDown");
    lookup.add_pair(0x4A, "mute");

    // Navigation keys
    lookup.add_pair(0x72, "help");
    lookup.add_pair(0x73, "home");
    lookup.add_pair(0x74, "pageUp");
    lookup.add_pair(0x75, "forwardDelete");
    lookup.add_pair(0x75, "delete"); // Note: On macOS this is called `forward delete`, and `delete` means backspace.
    lookup.add_pair(0x77, "end");
    lookup.add_pair(0x79, "pageDown");

    // Function keys
    lookup.add_pair(0x7A, "f1");
    lookup.add_pair(0x78, "f2");
    lookup.add_pair(0x63, "f3");
    lookup.add_pair(0x76, "f4");
    lookup.add_pair(0x60, "f5");
    lookup.add_pair(0x61, "f6");
    lookup.add_pair(0x62, "f7");
    lookup.add_pair(0x64, "f8");
    lookup.add_pair(0x65, "f9");
    lookup.add_pair(0x6D, "f10");
    lookup.add_pair(0x67, "f11");
    lookup.add_pair(0x6F, "f12");
    lookup.add_pair(0x69, "f13");
    lookup.add_pair(0x6B, "f14");
    lookup.add_pair(0x71, "f15");
    lookup.add_pair(0x6A, "f16");
    lookup.add_pair(0x40, "f17");
    lookup.add_pair(0x4F, "f18");
    lookup.add_pair(0x50, "f19");
    lookup.add_pair(0x5A, "f20");

    // Letters
    lookup.add_pair(0x00, "a");
    lookup.add_pair(0x0B, "b");
    lookup.add_pair(0x08, "c");
    lookup.add_pair(0x02, "d");
    lookup.add_pair(0x0E, "e");
    lookup.add_pair(0x03, "f");
    lookup.add_pair(0x05, "g");
    lookup.add_pair(0x04, "h");
    lookup.add_pair(0x22, "i");
    lookup.add_pair(0x26, "j");
    lookup.add_pair(0x28, "k");
    lookup.add_pair(0x25, "l");
    lookup.add_pair(0x2E, "m");
    lookup.add_pair(0x2D, "n");
    lookup.add_pair(0x1F, "o");
    lookup.add_pair(0x23, "p");
    lookup.add_pair(0x0C, "q");
    lookup.add_pair(0x0F, "r");
    lookup.add_pair(0x01, "s");
    lookup.add_pair(0x11, "t");
    lookup.add_pair(0x20, "u");
    lookup.add_pair(0x09, "v");
    lookup.add_pair(0x0D, "w");
    lookup.add_pair(0x07, "x");
    lookup.add_pair(0x10, "y");
    lookup.add_pair(0x06, "z");

    // Numbers
    lookup.add_pair(0x1D, "zero");
    lookup.add_pair(0x1D, "D0");
    lookup.add_pair(0x12, "one");
    lookup.add_pair(0x12, "D1");
    lookup.add_pair(0x13, "two");
    lookup.add_pair(0x13, "D2");
    lookup.add_pair(0x14, "three");
    lookup.add_pair(0x14, "D3");
    lookup.add_pair(0x15, "four");
    lookup.add_pair(0x15, "D4");
    lookup.add_pair(0x17, "five");
    lookup.add_pair(0x17, "D5");
    lookup.add_pair(0x16, "six");
    lookup.add_pair(0x16, "D6");
    lookup.add_pair(0x1A, "seven");
    lookup.add_pair(0x1A, "D7");
    lookup.add_pair(0x1C, "eight");
    lookup.add_pair(0x1C, "D8");
    lookup.add_pair(0x19, "nine");
    lookup.add_pair(0x19, "D9");

    // Symbols
    lookup.add_pair(0x18, "equals");
    lookup.add_pair(0x1B, "minus");
    lookup.add_pair(0x29, "semicolon");
    lookup.add_pair(0x27, "apostrophe");
    lookup.add_pair(0x2B, "comma");
    lookup.add_pair(0x2F, "period");
    lookup.add_pair(0x2C, "forwardSlash");
    lookup.add_pair(0x2A, "backslash");
    lookup.add_pair(0x32, "grave");
    lookup.add_pair(0x21, "leftBracket");
    lookup.add_pair(0x1E, "rightBracket");

    // Keypad
    lookup.add_pair(0x41, "keypadDecimal");
    lookup.add_pair(0x43, "keypadMultiply");
    lookup.add_pair(0x45, "keypadPlus");
    lookup.add_pair(0x47, "keypadClear");
    lookup.add_pair(0x4B, "keypadDivide");
    lookup.add_pair(0x4C, "keypadEnter");
    lookup.add_pair(0x4E, "keypadMinus");
    lookup.add_pair(0x51, "keypadEquals");
    lookup.add_pair(0x52, "keypad0");
    lookup.add_pair(0x53, "keypad1");
    lookup.add_pair(0x54, "keypad2");
    lookup.add_pair(0x55, "keypad3");
    lookup.add_pair(0x56, "keypad4");
    lookup.add_pair(0x57, "keypad5");
    lookup.add_pair(0x58, "keypad6");
    lookup.add_pair(0x59, "keypad7");
    lookup.add_pair(0x5B, "keypad8");
    lookup.add_pair(0x5C, "keypad9");

    lookup
}
