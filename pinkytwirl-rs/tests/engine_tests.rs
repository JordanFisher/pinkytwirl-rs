use pinkytwirl::{KeyEvent, KeyState, PinkyTwirlEngine};

// Helper functions
fn get_engine() -> PinkyTwirlEngine {
    let engine = PinkyTwirlEngine::new("../../../src/user_config".to_string());
    assert!(engine.startup.is_ok(), "Failed to load configurations");
    engine
}

fn key_down(key: &str) -> KeyEvent {
    let (key_str, shift, ctrl, alt, meta) = parse_key_string(key);
    KeyEvent {
        key: key_str,
        code: 0,
        state: KeyState::Down,
        shift,
        ctrl,
        alt,
        meta,
    }
}

fn key_up(key: &str) -> KeyEvent {
    let (key_str, shift, ctrl, alt, meta) = parse_key_string(key);
    KeyEvent {
        key: key_str,
        code: 0,
        state: KeyState::Up,
        shift,
        ctrl,
        alt,
        meta,
    }
}

fn parse_key_string(key: &str) -> (String, bool, bool, bool, bool) {
    let parts: Vec<&str> = key.split(" + ").collect();
    let mut shift = false;
    let mut ctrl = false;
    let mut alt = false;
    let mut meta = false;

    let key_str = if parts.len() > 1 {
        for &part in &parts[..parts.len() - 1] {
            match part.to_lowercase().as_str() {
                "shift" => shift = true,
                "ctrl" => ctrl = true,
                "alt" => alt = true,
                "meta" => meta = true,
                _ => (),
            }
        }
        parts[parts.len() - 1].to_string()
    } else {
        parts[0].to_string()
    };

    (key_str, shift, ctrl, alt, meta)
}

#[test]
fn test_context_matching() {
    let engine = get_engine();

    let test_cases = vec![
        ("Visual Studio Code", "main.rs - MyProject", Some("VSCode")),
        ("FIREFOX", "Google - Mozilla Firefox", Some("Firefox")),
        ("cmd", "Command Prompt", Some("CommandPrompt")),
        (
            "notepad++",
            "config.txt - Notepad++",
            Some("NotepadPlusPlus"),
        ),
        ("unknown_app", "Unknown Window", Some("Default")),
    ];

    for (app_name, window_name, expected_context) in test_cases {
        match engine.get_context(app_name, window_name) {
            Some(context) => {
                assert_eq!(
                    Some(context.name.as_str()),
                    expected_context,
                    "Unexpected context match for '{}' - '{}'",
                    app_name,
                    window_name
                );
            }
            None => {
                assert_eq!(
                    None, expected_context,
                    "Expected no context match for '{}' - '{}'",
                    app_name, window_name
                );
            }
        }
    }
}

#[test]
fn test_chord_resolution_nav_left() {
    // Meta + J -> Left
    let mut engine = get_engine();

    let chord_sequence = vec![
        // Key event, expected to suppress, expected number of pressed keys.
        (key_down("meta"), true, 1),
        (key_down("meta + j"), true, 1),
        (key_up("meta + j"), false, 1),
        (key_up("meta"), false, 0),
    ];

    for i in 0..3 {
        let mut synthetic_events_found = false;
        for (event, expected_suppress, expected_pressed_keys) in &chord_sequence {
            let (suppress, synthetic_events) =
                engine.handle_key_event(event.clone(), "Visual Studio Code", "main.rs - MyProject");
            if !synthetic_events.is_empty() {
                synthetic_events_found = true;
            }
            assert_eq!(engine.pressed_keys.len(), *expected_pressed_keys, "Number of keys still pressed is wrong [iteration {}]", i);
            assert_eq!(suppress, *expected_suppress, "Key event suppression is wrong [iteration {}]", i);
        }

        assert!(
            synthetic_events_found,
            "Chord sequence should generate at least one synthetic event"
        );
    }
}

#[test]
fn test_chord_resolution_select_left_word() {
    // Meta + J -> Left
    let mut engine = get_engine();

    let chord_sequence = vec![
        // Key event, expected to suppress, expected number of pressed keys.
        (key_down("d4"), true, 1),
        (key_down("d4 + m"), true, 1),
        (key_up("d4 + m"), false, 1),
        (key_up("d4"), false, 0),
    ];

    for i in 0..3 {
        let mut synthetic_events_found = false;
        for (event, expected_suppress, expected_pressed_keys) in &chord_sequence {
            let (suppress, synthetic_events) =
                engine.handle_key_event(event.clone(), "Visual Studio Code", "main.rs - MyProject");
            if !synthetic_events.is_empty() {
                synthetic_events_found = true;
            }
            assert_eq!(engine.pressed_keys.len(), *expected_pressed_keys, "Number of keys still pressed is wrong [iteration {}]", i);
            assert_eq!(suppress, *expected_suppress, "Key event suppression is wrong [iteration {}]", i);
            println!();
            dbg!(synthetic_events);
        }

        assert!(
            synthetic_events_found,
            "Chord sequence should generate at least one synthetic event"
        );
    }
}

#[test]
fn test_chord_resolution_letter_l() {
    // L -> L  (no chord)
    let mut engine = get_engine();

    let chord_sequence = vec![
        // Key event, expected to suppress, expected number of pressed keys.
        (key_down("l"), false, 1),
        (key_up("l"), false, 0),
    ];

    for i in 0..3 {
        let mut synthetic_events_found = false;
        for (event, expected_suppress, expected_pressed_keys) in &chord_sequence {
            let (suppress, synthetic_events) =
                engine.handle_key_event(event.clone(), "Visual Studio Code", "main.rs - MyProject");
            if !synthetic_events.is_empty() {
                dbg!(synthetic_events);
                synthetic_events_found = true;
            }
            assert_eq!(engine.pressed_keys.len(), *expected_pressed_keys, "Number of keys still pressed is wrong [iteration {}]", i);
            assert_eq!(suppress, *expected_suppress, "Key event suppression is wrong [iteration {}]", i);
        }

        println!("{:?}", engine.pressed_keys);
        assert!(engine.pressed_keys.is_empty(), "Pressed keys should be empty [iteration {}]", i);

        assert!(
            !synthetic_events_found,
            "There should be no synthetic events for a single key press of 'L' [iteration {}]",
            i
        );
    }
}

#[test]
fn test_chord_resolution_number_key_4() {
    // 4 -> 4  (maybe a chord, but turns out not to be)
    let mut engine = get_engine();

    let chord_sequence = vec![
        // Key event, expected to suppress, expected number of pressed keys.
        (key_down("d4"), true, 1),
        (key_up("d4"), false, 0),
    ];

    for i in 0..3 {
        let mut synthetic_events_found = false;
        for (event, expected_suppress, expected_pressed_keys) in &chord_sequence {
            let (suppress, synthetic_events) =
                engine.handle_key_event(event.clone(), "Visual Studio Code", "main.rs - MyProject");
            if !synthetic_events.is_empty() {
                synthetic_events_found = true;
            }
            assert_eq!(engine.pressed_keys.len(), *expected_pressed_keys, "Number of keys still pressed is wrong [iteration {}]", i);
            assert_eq!(suppress, *expected_suppress, "Key event suppression is wrong [iteration {}]", i);
        }

        println!("{:?}", engine.pressed_keys);
        assert!(engine.pressed_keys.is_empty(), "Pressed keys should be empty [iteration {}]", i);

        assert!(
            synthetic_events_found,
            "There *should* be synthetic events for a single key press of '4' [iteration {}]",
            i
        );
    }
}

#[test]
fn test_config_loading() {
    let engine = get_engine();
    engine.print_config();
}
